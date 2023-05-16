mod dom_utils;
mod face_detection;
mod laughing_man_image;
mod options;
mod tracker;

use crate::error::AppError;
use dom_utils::{canvas, context2d, get_element_by_id};
use laughing_man_image::{LaughingManImage, LaughingManState};
use options::AppOption;

use bytes::Buf;
use leptos::*;
use rustface::{read_model, Detector, Rectangle};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

/// ビデオ要素をwebカメラと連動させる初期化のための関数
pub async fn initialize_video(video: web_sys::HtmlVideoElement) -> Result<(), AppError> {
    let mut media_constraints = web_sys::MediaStreamConstraints::new();
    media_constraints.audio(&JsValue::FALSE);
    media_constraints.video(&JsValue::TRUE);

    let stream_promise = gloo_utils::window()
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

pub async fn fetch_and_create_detector(url: &str) -> Result<Box<dyn Detector>, AppError> {
    let res_bytes = reqwest::get(url).await?.bytes().await?;

    let model = read_model(res_bytes.reader())
        .map_err(|e| AppError::OtherError(format!("read_model failed: {e:?}")))?;
    Ok(rustface::create_detector_with_model(model))
}

#[component]
pub fn CanvasApp(cx: Scope) -> impl IntoView {
    let (laughing_man_state, set_laughing_man_state) = create_signal(
        cx,
        LaughingManState {
            top: 10,
            left: 20,
            width: 50,
            height: 50,
        },
    );

    view! {cx,
        <LaughingManImage id="laughing-man-1" state=laughing_man_state/>
    }
}
