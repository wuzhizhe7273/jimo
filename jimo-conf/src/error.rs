use thiserror::Error;

#[derive(Debug, Error)]
pub enum JimoConfError {
    #[error(transparent)]
    Storage(#[from] JimoConfStorageError),
    #[error("item: '{0}' not found")]
    ItemNotFound(String),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error("config manager not found")]
    MgrNotFound,
}

impl JimoConfError {
    pub fn item_not_found(name: &str) -> Self {
        JimoConfError::ItemNotFound(name.to_string())
    }
}

#[derive(Debug, Error)]

pub enum JimoConfStorageError {
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}
