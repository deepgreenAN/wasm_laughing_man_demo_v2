use crate::error::AppError;

use wasm_bindgen::JsCast;

pub fn get_element_by_id<T: JsCast>(id: &str) -> Result<T, AppError> {
    gloo_utils::document()
        .get_element_by_id(id)
        .ok_or(AppError::DomError(format!("Not Found id: {id}")))?
        .dyn_into::<T>()
        .map_err(|element| {
            AppError::DomError(format!(
                "Cannot convert from element: {element:?} into: {}",
                std::any::type_name::<T>()
            ))
        })
}

pub fn canvas(canvas_id: &str) -> Result<web_sys::HtmlCanvasElement, AppError> {
    get_element_by_id(canvas_id)
}

pub fn context2d(canvas_id: &str) -> Result<web_sys::CanvasRenderingContext2d, AppError> {
    canvas(canvas_id)?
        .get_context("2d")?
        .ok_or(AppError::DomError("Cannot get context2d".to_string()))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|object| {
            AppError::DomError(format!(
                "Cannot convert from element: {object:?}, into CanvasRenderingContext2d"
            ))
        })
}
