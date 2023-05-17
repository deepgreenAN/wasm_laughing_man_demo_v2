mod face_detection;
mod tracker;
mod tracker_roi;

use super::dom_utils::{canvas, context2d, document, window};
use crate::error::AppError;
use face_detection::{convert_rgba_to_luma, detect_faces};
use tracker::Tracker;
use tracker_roi::TrackerRoi;

use bytes::Buf;
use rustface::{read_model, Detector, Rectangle};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

/// ビデオ要素をwebカメラと連動させる初期化のための関数
async fn initialize_video(video: &web_sys::HtmlVideoElement) -> Result<(), AppError> {
    let mut media_constraints = web_sys::MediaStreamConstraints::new();
    media_constraints.audio(&JsValue::FALSE);
    media_constraints.video(&JsValue::TRUE);

    let stream_promise = window()
        .navigator()
        .media_devices()?
        .get_user_media_with_constraints(&media_constraints)?;

    let stream = JsFuture::from(stream_promise)
        .await?
        .dyn_into::<web_sys::MediaStream>()
        .map_err(|js_value| {
            AppError::JsError(format!(
                "Sanity Check. Cannot convert from: {js_value:?} into web_sys::MediaStream"
            ))
        })?;

    video.set_src_object(Some(&stream));
    JsFuture::from(video.play()?).await?;
    Ok(())
}

/// モデルをフェッチして検出器を作成
async fn fetch_and_create_detector(url: &str) -> Result<Box<dyn Detector>, AppError> {
    let res_bytes = reqwest::get(url).await?.bytes().await?;

    let model = read_model(res_bytes.reader())
        .map_err(|e| AppError::OtherError(format!("read_model failed: {e:?}")))?;
    Ok(rustface::create_detector_with_model(model))
}

/// VideoFaceTrackerの初期設定
#[derive(Clone, Debug)]
pub struct TrackerOptions {
    /// 入力画像 / ビデオのスケール
    pub image_over_video_scale: f64,
    /// 検出器のmin_face_size
    pub min_face_size: u32,
    /// 検出器のscore_thresh
    pub score_thresh: f64,
    /// 検出器のpyramid_scale_factor
    pub pyramid_scale_factor: f32,
    /// 検出器のslide_window_step
    pub slide_window_step: u32,
    /// トラッカーのallowable_not_detect_count
    pub allowable_not_detect_count: u32,
}

impl Default for TrackerOptions {
    fn default() -> Self {
        TrackerOptions {
            image_over_video_scale: 0.2,
            min_face_size: 40,
            score_thresh: 2.0,
            pyramid_scale_factor: 0.5,
            slide_window_step: 4,
            allowable_not_detect_count: 4,
        }
    }
}

/// ビデオをもとにトラッキングするトラッカー
pub struct VideoFaceTracker {
    /// ビデオから画像を作成するためのキャンバスのコンテキスト
    make_image_context: web_sys::CanvasRenderingContext2d,
    /// 画像を取得するビデオ
    stream_video: web_sys::HtmlVideoElement,
    /// 検出器への入力画像のサイズ(width, height)
    image_size: (u32, u32),
    /// 検出器
    detector: Box<dyn Detector>,
    /// トラッカー
    tracker: Tracker,
    /// パフォーマンス
    performance: web_sys::Performance,
    /// トラッカーオプション
    tracker_option: TrackerOptions,
}

/// ビデオトラッカーの返す情報
pub struct VideoFaceInfo<'a> {
    pub rois: Vec<&'a TrackerRoi>,
    pub added_rois: Vec<TrackerRoi>,
    pub removed_rois: Vec<TrackerRoi>,
    pub span_time: f64,
}

impl VideoFaceTracker {
    /// コンストラクタ
    /// - image_over_video_scale: 入力画像 / ビデオ のスケール
    pub async fn new(
        stream_video: web_sys::HtmlVideoElement,
        model_url: String,
        tracker_option: TrackerOptions,
    ) -> Result<VideoFaceTracker, AppError> {
        // ビデオの初期化
        initialize_video(&stream_video).await?;

        // ビデオからサイズを取得
        let (video_width, video_height) = (stream_video.video_width(), stream_video.video_height());
        let (image_width, image_height) = (
            (video_width as f64 * tracker_option.image_over_video_scale) as u32,
            (video_height as f64 * tracker_option.image_over_video_scale) as u32,
        );

        // 画像作成用のキャンパスを作成する
        let make_image_canvas = document()
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|element| {
                AppError::DomError(format!(
                    "Cannot convert from element: {element:?}, into HtmlCanvasElement"
                ))
            })?;
        make_image_canvas.set_width(image_width);
        make_image_canvas.set_height(image_height);
        let make_image_canvas_style = make_image_canvas.style();
        make_image_canvas_style.set_css_text("display:none");
        let make_image_context = context2d(&make_image_canvas)?;

        // 検出器を作成
        let mut detector = fetch_and_create_detector(&model_url).await?;

        detector.set_min_face_size(tracker_option.min_face_size);
        detector.set_score_thresh(tracker_option.score_thresh);
        detector.set_pyramid_scale_factor(tracker_option.pyramid_scale_factor);
        detector.set_slide_window_step(
            tracker_option.slide_window_step,
            tracker_option.slide_window_step,
        );

        // トラッカー
        let tracker = Tracker::new(tracker_option.allowable_not_detect_count);

        // パフォーマンス
        let performance = window()
            .performance()
            .ok_or(AppError::OtherError("Cannot get Performance.".to_string()))?;

        Ok(Self {
            make_image_context,
            stream_video,
            image_size: (image_width, image_height),
            detector,
            tracker,
            performance,
            tracker_option,
        })
    }

    /// ビデオトラッカーを遷移
    pub fn step(&mut self) -> Result<VideoFaceInfo<'_>, AppError> {
        let start_time = self.performance.now();

        self.make_image_context
            .draw_image_with_html_video_element_and_dw_and_dh(
                &self.stream_video,
                0.0,
                0.0,
                self.image_size.0 as f64,
                self.image_size.1 as f64,
            )?;

        let image_vec = self
            .make_image_context
            .get_image_data(0.0, 0.0, self.image_size.0 as f64, self.image_size.1 as f64)?
            .data()
            .0;

        let grey_image_vec = convert_rgba_to_luma(image_vec, self.image_size.0, self.image_size.1);

        let faces = detect_faces(
            &mut *self.detector,
            &grey_image_vec,
            self.image_size.0,
            self.image_size.1,
        );
        let faces: Vec<Rectangle> = faces.iter().map(|face| *face.bbox()).collect();

        // tracking
        let (added_rois, removed_rois) = self.tracker.track(&faces);

        let rois = self.tracker.rois().iter().collect::<Vec<&TrackerRoi>>();

        Ok(VideoFaceInfo {
            rois,
            added_rois,
            removed_rois,
            span_time: self.performance.now() - start_time,
        })
    }

    /// トラッカーを再設定
    pub fn initialize_tracker(&mut self, tracker_option: TrackerOptions) -> Result<(), AppError> {
        let (image_width, image_height) = (
            (self.image_size.0 as f64
                * (tracker_option.image_over_video_scale as f64
                    / self.tracker_option.image_over_video_scale)) as u32,
            (self.image_size.1 as f64
                * (tracker_option.image_over_video_scale as f64
                    / self.tracker_option.image_over_video_scale)) as u32,
        );

        // 画像作成用のキャンパスを再設定
        let make_image_canvas = canvas(&self.make_image_context)?;
        make_image_canvas.set_width(image_width);
        make_image_canvas.set_height(image_height);

        // 検出器の再設定
        self.detector
            .set_min_face_size(tracker_option.min_face_size);
        self.detector.set_score_thresh(tracker_option.score_thresh);
        self.detector
            .set_pyramid_scale_factor(tracker_option.pyramid_scale_factor);
        self.detector.set_slide_window_step(
            tracker_option.slide_window_step,
            tracker_option.slide_window_step,
        );

        // トラッカー
        self.tracker = Tracker::new(tracker_option.allowable_not_detect_count);

        Ok(())
    }
}

impl Drop for VideoFaceTracker {
    fn drop(&mut self) {
        canvas(&self.make_image_context)
            .expect("Sanity Check")
            .remove(); // DOMツリーから削除
    }
}
