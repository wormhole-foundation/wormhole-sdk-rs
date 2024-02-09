use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// serde_json
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    /// reqwest
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    /// IO
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Api
    #[error("Internal API Error. Hint: this usually means a misformatted URL")]
    ApiError(#[from] ApiError),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    pub code: u32,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<HashMap<String, serde_json::Value>>,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiError")
            .field("code", &self.code)
            .field("message", &self.message)
            .field("details", &self.details)
            .finish()
    }
}

impl std::error::Error for ApiError {}
