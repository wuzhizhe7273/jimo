use std::marker::PhantomData;

use futures::StreamExt;

use crate::common::{
    AggregateFetcherError, Apply, Envelope, EventFetcherError, EventReader,
    aggregate::{self, SnapShotStore, error::SnapShotFetcherError, executor::EventExecutor},
};

pub struct EventFetcher<A, S>
where
    A: aggregate::Aggregate,
    S: EventReader<A>,
{
    id: A::ID,
    version: u64,
    _marker: PhantomData<S>,
}

impl<A, S> EventFetcher<A, S>
where
    A: aggregate::Aggregate,
    S: EventReader<A>,
{
    pub fn new(id: A::ID, version: u64) -> Self {
        Self {
            id,
            version,
            _marker: PhantomData,
        }
    }
}

impl<'s, A, S> EventExecutor<'s> for EventFetcher<A, S>
where
    A: aggregate::Aggregate + 'static,
    S: EventReader<A> + 'static,
{
    type Ret = futures::stream::BoxStream<'s, Envelope<A>>;
    type Error = EventFetcherError;
    type Store = S;
    async fn execute(&'s mut self, store: &'s Self::Store) -> Result<Self::Ret, Self::Error> {
        let stream = store.stream(&self.id, self.version)?.boxed();
        Ok(stream)
    }
}

pub struct AggregateFetcher<A, S>
where
    A: aggregate::Aggregate + aggregate::Apply,
{
    id: A::ID,
    _marker: PhantomData<S>,
}

impl<A, S> AggregateFetcher<A, S>
where
    A: aggregate::Aggregate + aggregate::Apply,
    S: EventReader<A>,
{
    pub fn new(id: A::ID) -> Self {
        Self {
            id,
            _marker: Default::default(),
        }
    }
}

impl<'s, A, S> EventExecutor<'s> for AggregateFetcher<A, S>
where
    A: aggregate::Aggregate + aggregate::Apply + 'static,
    S: EventReader<A> + 'static,
{
    type Ret = aggregate::Context<A>;
    type Error = AggregateFetcherError;
    type Store = S;
    async fn execute(&'s mut self, store: &'s Self::Store) -> Result<Self::Ret, Self::Error> {
        let stream = store.stream(&self.id, 0)?.boxed();
        let mut ctx = aggregate::Context::empty(self.id.clone());
        ctx.rehydrate(stream).await?;
        Ok(ctx)
    }
}

pub struct SnapShotFetcher<A, S>
where
    A: aggregate::Aggregate,
    S: SnapShotStore<A> + EventReader<A>,
{
    id: A::ID,
    _marker: PhantomData<S>,
}

impl<'s, A, S> EventExecutor<'s> for SnapShotFetcher<A, S>
where
    A: aggregate::Aggregate + Apply,
    S: SnapShotStore<A> + EventReader<A>,
{
    type Ret = aggregate::Context<A>;
    type Error = SnapShotFetcherError;
    type Store = S;
    async fn execute(&'s mut self, store: &'s Self::Store) -> Result<Self::Ret, Self::Error> {
        let snapshot = store.get(&self.id).await?;
        let mut ctx = if let Some(snap) = snapshot {
            aggregate::Context::<A>::from_snapshot(snap)
        } else {
            aggregate::Context::<A>::empty(self.id.clone())
        };
        let stream = store.stream(&self.id, ctx.version)?.boxed();
        ctx.rehydrate(stream).await?;
        Ok(ctx)
    }
}
