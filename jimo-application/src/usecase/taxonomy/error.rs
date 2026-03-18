use jimo_domain::common::{
    AggregateFetcherError, EventCommiterError, EventFetcherError, aggregate::ApplyError,
};

#[derive(Debug, thiserror::Error)]
pub enum TaxonomyError {
    #[error(transparent)]
    EventFetcher(#[from] EventFetcherError),
    #[error(transparent)]
    Commiter(#[from] EventCommiterError),
    #[error(transparent)]
    AggregateFetcher(#[from] AggregateFetcherError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Taxonomy Error: {0}")]
    Apply(Box<dyn ApplyError>),
}

impl TaxonomyError {
    pub fn apply(e: impl ApplyError) -> Self {
        Self::Apply(Box::new(e))
    }
}
