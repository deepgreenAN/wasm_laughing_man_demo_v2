use wasm_bindgen::JsValue;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("AppError::DomError: {0}")]
    DomError(String),

    #[error("AppError::JsError: {0}")]
    JsError(String),

    #[error("AppError::FetchError: {0}")]
    FetchError(String),

    #[error("AppError::OtherError: {0}")]
    OtherError(String),
}

impl From<JsValue> for AppError {
    fn from(value: JsValue) -> Self {
        AppError::JsError(format!("{value:?}"))
    }
}

impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        AppError::FetchError(value.to_string())
    }
}
