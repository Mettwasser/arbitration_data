#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    Json(#[from] serde_json::Error),
}
