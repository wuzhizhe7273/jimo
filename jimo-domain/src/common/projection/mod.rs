pub mod inline;
pub mod multi;
mod query;

use crate::common::aggregate::{self, Key};

pub use inline::{
    InlineProjection, InlineProjectionStore, InlineProjectionStoreError, InlineProjectorError,
};
pub use query::{Query, QueryByKey};
pub trait Projection {
    type ID;
    fn id(&self) -> &Self::ID;
}

pub trait ProjectionStore: Sized {
    async fn execute<Q, Ret, Error>(&self, query: Q) -> Result<Ret, Error>
    where
        Error: std::error::Error + Send + Sync + 'static,
        Q: Query<Ret, Self, Error>,
    {
        query.execute(self).await
    }
}

pub trait Projector<A>: ProjectionStore
where
    A: aggregate::Aggregate,
{
    type Error: std::error::Error + Send + Sync + 'static;
    async fn project<Stream>(&self, items: &mut Stream) -> Result<(), Self::Error>
    where
        Stream: futures::Stream<Item = (A::ID, aggregate::Envelope<A>)> + Unpin;
}
