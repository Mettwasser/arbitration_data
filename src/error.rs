#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct InnerError {
    pub message: String,
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    Json(#[from] serde_json::Error),

    Csv(#[from] csv::Error),

    Initialization(#[from] InnerError),
}
