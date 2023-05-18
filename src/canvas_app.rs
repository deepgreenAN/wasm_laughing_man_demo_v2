mod dom_utils;
mod laughing_man_image;
mod side_menu;
mod video_face_tracker;

use crate::Interval;
use crate::IsSideMenuActive;
use dom_utils::context2d;
use laughing_man_image::{LaughingManImage, LaughingManOptions, LaughingManState};
use side_menu::SideMenu;
use video_face_tracker::{TrackerOptions, VideoFaceInfo, VideoFaceTracker};

use leptos::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;

use std::ops::Deref;

fn get_video_canvas_size(
    header_height: u32,
    video_element: &web_sys::HtmlVideoElement,
) -> ((f64, f64), f64) {
    let (video_width, video_height) = (video_element.video_width(), video_element.video_height());
    let video_width_over_height = (video_width as f64) / (video_height as f64); //  ビデオ幅 / ビデオ高さ

    let (screen_width, screen_height) = (
        window()
            .inner_width()
            .expect("Cannot Get inner_width")
            .as_f64()
            .expect("Sanity Check"),
        window()
            .inner_height()
            .expect("Cannot inner_height")
            .as_f64()
            .expect("Sanity Check"),
    );

    let (canvas_app_width, canvas_app_height) = if video_width > video_height {
        // ビデオ幅 > ビデオ高さの場合 -> 高さを合わせる
        let canvas_app_height = screen_height - (header_height as f64);
        let canvas_app_width = canvas_app_height * video_width_over_height;
        (canvas_app_width, canvas_app_height)
    } else {
        // ビデオ幅 < ビデオ高さの場合 -> 幅を合わせる
        let canvas_app_width = screen_width;
        let canvas_app_height = canvas_app_width / video_width_over_height;
        (canvas_app_width, canvas_app_height)
    };

    let canvas_app_over_video_scale = canvas_app_width / (video_width as f64);

    (
        (canvas_app_width, canvas_app_height),
        canvas_app_over_video_scale,
    )
}

#[derive(Clone, Debug)]
pub struct CanvasAppOptions {
    pub tracker_options: TrackerOptions,
    pub laughing_man_options: LaughingManOptions,
    pub is_active_laughing_man: bool,
    pub interval_span: u32,
}

impl Default for CanvasAppOptions {
    fn default() -> Self {
        Self {
            tracker_options: Default::default(),
            laughing_man_options: Default::default(),
            is_active_laughing_man: true,
            interval_span: 100,
        }
    }
}

#[component]
pub fn CanvasApp(cx: Scope, header_height: u32) -> impl IntoView {
    let CanvasAppOptions {
        tracker_options,
        laughing_man_options,
        is_active_laughing_man,
        interval_span,
    } = Default::default();

    // 笑い男画像の状態
    let laughing_man_images = Rc::new(RefCell::new(Vec::<LaughingManImage>::new()));

    // node_ref
    let container_node_ref = create_node_ref::<leptos::html::Div>(cx);
    let video_node_ref = create_node_ref::<leptos::html::Video>(cx);
    let canvas_node_ref = create_node_ref::<leptos::html::Canvas>(cx);

    // トラッカー
    let video_face_tracker = Rc::new(RefCell::new(Option::<VideoFaceTracker>::None));

    // width, heightを動的に変更するためのシグナル
    let (canvas_app_width, set_canvas_app_width) = create_signal(cx, 0 as u32);
    let (canvas_app_height, set_canvas_app_height) = create_signal(cx, 0 as u32);

    // 各種オプション
    let tracker_options = Rc::new(RefCell::new(tracker_options));
    let laughing_man_options = Rc::new(RefCell::new(laughing_man_options)); // インターバル内で利用
    let is_active_laughing_man = Rc::new(Cell::new(is_active_laughing_man)); // インターバル内で利用
    let interval_span = Rc::new(Cell::new(interval_span));

    // インターバルのLAII
    let interval_state = Rc::new(RefCell::new(Option::<Interval>::None));

    // サイドメニューが開いているかどうか
    let is_side_menu_active = use_context::<IsSideMenuActive>(cx).expect("Cannot get context");
    let expand_menu_class = move || {
        if is_side_menu_active.0.get() {
            "expand-menu active"
        } else {
            "expand-menu"
        }
    };

    // トラッカーの初期化
    create_effect(cx, {
        // 他の場所で使う場合はここでクローン
        // 状態
        let laughing_man_images = laughing_man_images.clone();
        let video_face_tracker = video_face_tracker.clone();
        // インターバルハンドル
        let interval_state = interval_state.clone();
        // オプション
        let laughing_man_options = laughing_man_options.clone();
        let is_active_laughing_man = is_active_laughing_man.clone();

        move |_| {
            log::info!("VideoFaceTracker initialize.");
            // 状態
            let laughing_man_images = laughing_man_images.clone();
            let video_face_tracker = video_face_tracker.clone();
            // インターバルハンドル
            let interval_state = interval_state.clone();
            // オプション
            let tracker_options = tracker_options.clone();
            let laughing_man_options = laughing_man_options.clone();
            let is_active_laughing_man = is_active_laughing_man.clone();
            let interval_span = interval_span.clone();
            spawn_local(async move {
                let video_element: web_sys::HtmlVideoElement =
                    video_node_ref.get().expect("Sanity Check").deref().clone(); // ここでやらないといけない．しかし範囲外の警告がでる
                let canvas_element: web_sys::HtmlCanvasElement =
                    canvas_node_ref.get().expect("Sanity Check").deref().clone(); // ここでやらないといけない．しかし範囲外の警告がでる

                let container_node: web_sys::Node = {
                    let container_div: web_sys::HtmlDivElement = container_node_ref
                        .get()
                        .expect("Sanity Check")
                        .deref()
                        .clone();
                    container_div.into()
                };

                let image_over_video_scale =
                    { tracker_options.borrow_mut().image_over_video_scale }; // 画像 / ビデオ のスケール

                let tracker_res = VideoFaceTracker::new(
                    video_element.clone(),
                    "https://dl.dropboxusercontent.com/s/ypb7jrufzgghp62/seeta_fd_frontal_v1.0.bin"
                        .to_string(),
                    tracker_options.replace_with(|options| options.clone()),
                )
                .await;

                match tracker_res {
                    Ok(tracker) => {
                        {
                            *video_face_tracker.borrow_mut() = Some(tracker);
                        }

                        let ((canvas_app_width, canvas_app_height), canvas_app_over_video_scale) =
                            get_video_canvas_size(header_height, &video_element);

                        // 各種要素に高さと幅を設定
                        video_element.set_width(canvas_app_width as u32);
                        video_element.set_height(canvas_app_height as u32);

                        canvas_element.set_width(canvas_app_width as u32);
                        canvas_element.set_height(canvas_app_height as u32);

                        set_canvas_app_width.set(canvas_app_width as u32);
                        set_canvas_app_height.set(canvas_app_height as u32);

                        // 表示画像 / 入力画像
                        let canvas_app_over_input_image =
                            (1.0 / image_over_video_scale) * canvas_app_over_video_scale;

                        // キャンバスのコンテキストの設定
                        let canvas_context =
                            context2d(&canvas_element).expect("Cannot get context2d.");

                        canvas_context.set_font("20px serif");
                        canvas_context.set_fill_style(&wasm_bindgen::JsValue::from_str("#FF0000"));
                        canvas_context
                            .set_stroke_style(&wasm_bindgen::JsValue::from_str("#FF0000"));

                        // インターバルを設定
                        let interval = Interval::new(interval_span.get(), {
                            let video_face_tracker = video_face_tracker.clone();

                            move || {
                                if let Some(video_face_tracker) =
                                    video_face_tracker.borrow_mut().as_mut()
                                {
                                    let VideoFaceInfo {
                                        rois,
                                        added_rois,
                                        removed_rois,
                                        span_time,
                                    } = video_face_tracker
                                        .step()
                                        .expect("Cannot VideoFaceTracker step");

                                    let roi_numbers = rois.len();

                                    // キャンパスの初期化
                                    canvas_context.clear_rect(
                                        0.0,
                                        0.0,
                                        canvas_app_width,
                                        canvas_app_height,
                                    );

                                    // domの追加
                                    if is_active_laughing_man.get() {
                                        // 笑い男モードの場合
                                        for roi in added_rois.into_iter() {
                                            let state = LaughingManState {
                                                id: roi.id,
                                                left: (roi.tl_x * canvas_app_over_input_image)
                                                    as u32,
                                                top: (roi.tl_y * canvas_app_over_input_image)
                                                    as u32,
                                                width: (roi.width * canvas_app_over_input_image)
                                                    as u32,
                                                height: (roi.height * canvas_app_over_input_image)
                                                    as u32,
                                            };

                                            let laughing_man_img = LaughingManImage::new(
                                                container_node.clone(),
                                                "/laughing-man.svg",
                                                laughing_man_options
                                                    .replace_with(|options| options.clone()),
                                                state,
                                            )
                                            .expect("Cannot create laughing_man_img");

                                            {
                                                laughing_man_images
                                                    .borrow_mut()
                                                    .push(laughing_man_img)
                                            }
                                        }
                                    }

                                    // domの削除
                                    if is_active_laughing_man.get() {
                                        // 笑い男モードの場合
                                        laughing_man_images.borrow_mut().retain(
                                            |laughing_man_img| {
                                                !removed_rois
                                                    .iter()
                                                    .any(|roi| laughing_man_img.id() == roi.id)
                                            },
                                        )
                                    }

                                    // 笑い男の遷移(描画)
                                    if is_active_laughing_man.get() {
                                        // 笑い男モードの場合
                                        for (roi, laughing_man_img) in rois
                                            .into_iter()
                                            .zip(laughing_man_images.borrow_mut().iter_mut())
                                        {
                                            let state = LaughingManState {
                                                id: roi.id,
                                                left: (roi.tl_x * canvas_app_over_input_image)
                                                    as u32,
                                                top: (roi.tl_y * canvas_app_over_input_image)
                                                    as u32,
                                                width: (roi.width * canvas_app_over_input_image)
                                                    as u32,
                                                height: (roi.height * canvas_app_over_input_image)
                                                    as u32,
                                            };

                                            laughing_man_img.step(state).unwrap_throw();
                                        }
                                    } else {
                                        // 笑い男モードでない場合
                                        for roi in rois {
                                            canvas_context.stroke_rect(
                                                roi.tl_x * canvas_app_over_input_image,
                                                roi.tl_y * canvas_app_over_input_image,
                                                roi.width * canvas_app_over_input_image,
                                                roi.height * canvas_app_over_input_image,
                                            );

                                            canvas_context
                                                .fill_text(
                                                    &format!("id: {}", roi.id),
                                                    roi.tl_x * canvas_app_over_input_image,
                                                    roi.tl_y * canvas_app_over_input_image,
                                                )
                                                .unwrap_throw();
                                        }
                                    }

                                    let text_info = format!(
                                        "{:03} faces detected in {:.1}[ms]",
                                        roi_numbers, span_time
                                    );
                                    canvas_context
                                        .fill_text(&text_info, 20.0, 20.0)
                                        .expect("Cannot add text");
                                }
                            }
                        });

                        {
                            *interval_state.borrow_mut() = Some(interval);
                        }
                    }
                    Err(e) => {
                        log::error!("{e}");
                    }
                }
            });
        }
    });

    // オプションの適用時の処理
    let on_apply = {
        // 状態
        let laughing_man_images = laughing_man_images.clone();
        let video_face_tracker = video_face_tracker.clone();
        // インターバルハンドル
        let interval_state = interval_state.clone();
        // オプション
        let laughing_man_options = laughing_man_options.clone();
        let is_active_laughing_man = is_active_laughing_man.clone();

        move |new_canvas_app_options| {
            let CanvasAppOptions {
                tracker_options: new_tracker_options,
                laughing_man_options: new_laughing_man_options,
                is_active_laughing_man: new_is_active_laughing_man,
                interval_span: new_interval_span,
            } = new_canvas_app_options;

            // トラッカーの初期化
            {
                if let Some(video_face_tracker) = video_face_tracker.borrow_mut().as_mut() {
                    if let Err(e) = video_face_tracker.initialize_tracker(new_tracker_options) {
                        log::error!("{e}");
                    }
                }
            }
            // 笑い男画像の初期化
            {
                *laughing_man_images.borrow_mut() = Vec::new();
            }

            // インターバル内で利用する状態の更新
            {
                *laughing_man_options.borrow_mut() = new_laughing_man_options;
                is_active_laughing_man.set(new_is_active_laughing_man);
            }

            // インターバルハンドルの初期化
            {
                let mut interval_state_opt_mut = interval_state.borrow_mut();
                if let Some(interval_state_mut) = interval_state_opt_mut.as_mut() {
                    let old_interval_state = std::mem::take(interval_state_mut);
                    let new_interval =
                        Interval::from_closure(new_interval_span, old_interval_state.cancel());
                    *interval_state_mut = new_interval;
                }
            }
        }
    };

    view! {cx,
        <div class=expand_menu_class>
            <SideMenu canvas_app_options={Default::default()} on_apply=on_apply/>
        </div>
        <div
            class="canvas-app-container"
            style:width = move ||{format!("{}px", canvas_app_width.get())}
            style:height = move ||{format!("{}px", canvas_app_height.get())}
            node_ref=container_node_ref
        >
            <video class="stream-video" node_ref=video_node_ref></video>
            <canvas class="canvas-app" node_ref=canvas_node_ref></canvas>
        </div>
    }
}
