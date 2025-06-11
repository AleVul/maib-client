pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Http(String),
    Json(String),
    Api(Vec<ApiError>),
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    error_code: String,
    error_message: String,
}

impl ApiError {
    pub fn code(&self) -> &str {
        return &self.error_code;
    }

    pub fn message(&self) -> &str {
        return &self.error_message;
    }
}
