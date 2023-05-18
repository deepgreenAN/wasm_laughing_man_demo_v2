use wasm_bindgen::JsValue;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    /// DOMに関するエラー．
    #[error("AppError::DomError: {0}")]
    DomError(String),
    /// javascriptに関するエラー．
    #[error("AppError::JsError: {0}")]
    JsError(String),
    /// データのフェッチに関するエラー．
    #[error("AppError::FetchError: {0}")]
    FetchError(String),
    /// その他のエラー
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
