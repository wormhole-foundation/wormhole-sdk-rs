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
}
