use crate::common::aggregate::{Apply, ApplyError, Envelope};
use crate::common::projection::{Projection, ProjectionStore, Projector};
use crate::common::{Aggregate, Query};
use futures::StreamExt;
use std::fmt::Debug;

pub struct InlineProjection<A>
where
    A: crate::common::aggregate::Aggregate,
{
    aggregate: A,
}

#[derive(Debug, thiserror::Error)]
pub enum InlineProjectionStoreError {}

pub trait InlineProjectionStore<A>: ProjectionStore
where
    A: crate::common::aggregate::Aggregate,
{
    async fn get(&self, id: &A::ID) -> Result<Option<A>, InlineProjectionStoreError>;
    async fn save(&self, aggregate: A) -> Result<(), InlineProjectionStoreError>;
    async fn delete(&self, id: &A::ID) -> Result<(), InlineProjectionStoreError>;
}

impl<A> Projection for InlineProjection<A>
where
    A: crate::common::aggregate::Aggregate,
{
    type ID = A::ID;
    fn id(&self) -> &Self::ID {
        self.aggregate.id()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InlineProjectorError {
    #[error("Inline Projector Error:{0}")]
    Apply(Box<dyn ApplyError>),
    #[error("Inline Projector Error:{0}")]
    Internal(#[from] anyhow::Error),
    #[error("Inline Prjector Store Error:{0}")]
    Store(#[from] InlineProjectionStoreError),
}

impl<T> From<T> for InlineProjectorError
where
    T: ApplyError,
{
    fn from(value: T) -> Self {
        InlineProjectorError::Apply(Box::new(value))
    }
}

impl<A, S> Projector<A> for S
where
    A: crate::common::aggregate::Aggregate + Apply,
    S: InlineProjectionStore<A>,
{
    type Error = InlineProjectorError;
    async fn project<Stream>(&self, items: &mut Stream) -> Result<(), Self::Error>
    where
        Stream: futures::Stream<Item = (A::ID, Envelope<A>)> + Unpin,
    {
        while let Some((_id, envelope)) = items.next().await {
            let mut aggregate = self.get(&_id).await?;
            Apply::apply(&mut aggregate, &envelope.event())?;
            if let Some(aggregate) = aggregate {
                self.save(aggregate).await?;
            } else {
                self.delete(&_id).await?;
            }
        }
        Ok(())
    }
}
