use jimo_domain::common::{
    AggregateFetcherError, EventCommiterError, EventFetcherError, aggregate::ApplyError,
};

use crate::usecase::iam::interface::PasswordHasherError;

#[derive(Debug, thiserror::Error)]
pub enum AuthenticateError {
    #[error("invalid crendials")]
    InvalidCredentials,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum IAMError {
    #[error("IAM Error:{0}")]
    Hasher(#[from] PasswordHasherError),
    #[error(transparent)]
    EventFetcher(#[from] EventFetcherError),
    #[error(transparent)]
    Commiter(#[from] EventCommiterError),
    #[error(transparent)]
    AggregateFetcher(#[from] AggregateFetcherError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("IAM Error: {0}")]
    Apply(Box<dyn ApplyError>),
    #[error(transparent)]
    Authenticate(#[from] AuthenticateError),
}

impl IAMError {
    pub fn apply(e: impl ApplyError) -> Self {
        Self::Apply(Box::new(e))
    }
}
