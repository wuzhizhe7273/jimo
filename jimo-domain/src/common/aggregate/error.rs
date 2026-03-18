use crate::common::aggregate::Aggregate;

#[derive(Debug, thiserror::Error)]
#[error("aggregate {aggregate}(id:{id}) not found")]
pub struct AggregateNotFound {
    aggregate: String,
    id: String,
}

impl AggregateNotFound {
    pub fn new<A: Aggregate>(id: &A::ID) -> Self {
        Self {
            aggregate: A::ty().to_string(),
            id: id.to_string(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("aggregate {aggregate} {field}:{value} already exists")]
pub struct AggregateAlreadyExists {
    aggregate: String,
    field: String,
    value: String,
}

impl AggregateAlreadyExists {
    pub fn new<A: Aggregate>(field: &str, value: impl ToString) -> Self {
        Self {
            aggregate: A::ty().to_string(),
            field: field.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("aggregate version conflict, expected:{expected} actual:{version}")]
pub struct AggregateVersionConflict {
    aggregate: String,
    expected: u64,
    version: u64,
}

impl AggregateVersionConflict {
    pub fn new<A: Aggregate>(expected: u64, actual: u64) -> Self {
        Self {
            aggregate: A::ty().to_string(),
            expected,
            version: actual,
        }
    }
}

pub trait ApplyError: std::error::Error + Send + Sync + 'static {
    fn aggregate_type(&self) -> &'static str;
}

#[derive(Debug, thiserror::Error)]
pub enum RehydrateError {
    #[error(transparent)]
    Conflict(#[from] AggregateVersionConflict),
    #[error("{0}")]
    Apply(Box<dyn ApplyError>),
}

impl<T> From<T> for RehydrateError
where
    T: ApplyError + 'static,
{
    fn from(value: T) -> Self {
        RehydrateError::Apply(Box::new(value))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EventReaderError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum EventWriterError {
    #[error(transparent)]
    Conflict(#[from] AggregateVersionConflict),
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum EventFetcherError {
    #[error(transparent)]
    Reader(#[from] EventReaderError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum EventCommiterError {
    #[error(transparent)]
    Writer(#[from] EventWriterError),
    #[error("aggregate context is invalid")]
    ContextInvalid,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum AggregateFetcherError {
    #[error(transparent)]
    Reader(#[from] EventReaderError),
    #[error(transparent)]
    Rehydrate(#[from] RehydrateError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SnapShotError {
    #[error(transparent)]
    Internal(anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SnapShotFetcherError {
    #[error(transparent)]
    SnapShot(#[from] SnapShotError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error(transparent)]
    Reader(#[from] EventReaderError),
    #[error(transparent)]
    Rehydrate(#[from] RehydrateError),
}
