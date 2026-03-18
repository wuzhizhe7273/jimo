use jimo_domain::common::{AggregateFetcherError, EventCommiterError, aggregate::ApplyError};

#[derive(Debug, thiserror::Error)]
pub enum PostError {
    #[error(transparent)]
    Commiter(#[from] EventCommiterError),
    #[error(transparent)]
    Fetcher(#[from] AggregateFetcherError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Taxonomy Error: {0}")]
    Apply(Box<dyn ApplyError>),
}

impl PostError {
    pub fn apply(e: impl ApplyError) -> Self {
        PostError::Apply(Box::new(e))
    }
}
