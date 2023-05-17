use crate::error::AppError;

use wasm_bindgen::JsCast;

/// documentを取得するヘルパー関数
pub fn document() -> web_sys::Document {
    leptos::document()
}

/// windowを取得するヘルパー関数
pub fn window() -> web_sys::Window {
    leptos::window()
}

pub fn canvas(
    context2d: &web_sys::CanvasRenderingContext2d,
) -> Result<web_sys::HtmlCanvasElement, AppError> {
    context2d.canvas().ok_or(AppError::DomError(
        "Cannot get canvas from context2d".to_string(),
    ))
}

pub fn context2d(
    canvas: &web_sys::HtmlCanvasElement,
) -> Result<web_sys::CanvasRenderingContext2d, AppError> {
    canvas
        .get_context("2d")?
        .ok_or(AppError::DomError(
            "Cannot get context2d from canvas".to_string(),
        ))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|object| {
            AppError::DomError(format!(
                "Cannot convert from element: {object:?}, into CanvasRenderingContext2d"
            ))
        })
}
