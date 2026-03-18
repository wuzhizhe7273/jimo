mod error;
mod event;
mod executor;
mod snapshot;
mod store;

pub use executor::{AggregateFetcher, EventCommiter, EventExecutor, EventFetcher};
pub use snapshot::{SnapShot, SnapShotStore};

pub use error::{
    AggregateAlreadyExists, AggregateFetcherError, AggregateNotFound, AggregateVersionConflict,
    ApplyError, EventCommiterError, EventFetcherError, EventReaderError, EventWriterError,
    RehydrateError, SnapShotError,
};
pub use event::{Envelope, Event};
use futures::StreamExt;
use std::fmt::Debug;
pub use store::{EventReader, EventWriter};
pub trait Key {
    fn unique(&self) -> bool;
}
impl Key for () {
    fn unique(&self) -> bool {
        false
    }
}
pub trait Aggregate: Clone {
    type ID: Clone + Debug + ToString + Send + Sync;
    type Event: Event;
    type Key: Key = ();

    fn id(&self) -> &Self::ID;

    fn ty() -> &'static str {
        std::any::type_name::<Self>()
    }
}

pub trait Apply: Aggregate {
    type Error: ApplyError;

    fn apply(aggregate: &mut Option<Self>, event: &Self::Event) -> Result<(), Self::Error>;
}

pub struct Context<A>
where
    A: Aggregate,
{
    stream: A::ID,
    aggregate: Option<A>,
    events: Vec<Envelope<A>>,
    version: u64,
}
impl<A> Context<A>
where
    A: Aggregate + Apply,
    A::Error: 'static,
{
    pub fn empty(id: A::ID) -> Context<A> {
        Context {
            stream: id,
            aggregate: None,
            events: vec![],
            version: 0,
        }
    }

    pub fn id(&self) -> &A::ID {
        &self.stream
    }

    pub fn as_aggregate(&self) -> Option<&A> {
        self.aggregate.as_ref()
    }

    pub fn to_aggregate(self) -> Option<A> {
        self.aggregate
    }

    pub fn from_snapshot(snapshot: SnapShot<A>) -> Self {
        snapshot.into()
    }

    pub fn apply(&mut self, events: Vec<A::Event>) -> Result<&mut Self, A::Error> {
        for event in events {
            A::apply(&mut self.aggregate, &event)?;
            let version = self.version + 1;
            let envelope = Envelope::new(event, version);
            self.version = version;
            self.events.push(envelope);
        }
        Ok(self)
    }

    pub fn commit<S>(self) -> EventCommiter<A, S>
    where
        S: EventWriter<A>,
    {
        EventCommiter::new(self)
    }

    pub fn events<S>(id: A::ID, version: u64) -> Result<EventFetcher<A, S>, EventFetcherError>
    where
        S: EventReader<A>,
    {
        Ok(EventFetcher::new(id, version))
    }

    pub fn fetch<S>(id: A::ID) -> AggregateFetcher<A, S>
    where
        S: EventReader<A>,
    {
        AggregateFetcher::new(id)
    }

    pub async fn rehydrate<Stream>(
        &mut self,
        mut stream: Stream,
    ) -> Result<&mut Self, RehydrateError>
    where
        Stream: futures::Stream<Item = Envelope<A>> + Unpin,
    {
        while let Some(envelope) = stream.next().await {
            let expected = self.version + 1;
            envelope
                .version()
                .eq(&expected)
                .ok_or(AggregateVersionConflict::new::<A>(expected, self.version))?;
            self.apply(vec![envelope.event()])?;
        }
        Ok(self)
    }
}

impl<A> From<SnapShot<A>> for Context<A>
where
    A: Aggregate,
{
    fn from(value: SnapShot<A>) -> Self {
        let stream = value.id().clone();
        let version = value.version();
        let aggregate = value.to_aggregate();

        Context {
            stream,
            aggregate: Some(aggregate),
            events: vec![],
            version,
        }
    }
}
