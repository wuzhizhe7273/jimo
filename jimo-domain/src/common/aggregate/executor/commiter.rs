use std::marker::PhantomData;

use crate::common::{
    EventCommiterError, EventWriter,
    aggregate::{self, executor::EventExecutor},
};

pub struct EventCommiter<A, S>
where
    A: aggregate::Aggregate,
    S: EventWriter<A>,
{
    context: aggregate::Context<A>,
    _marker: PhantomData<S>,
}

impl<A, S> EventCommiter<A, S>
where
    A: aggregate::Aggregate,
    S: EventWriter<A>,
{
    pub fn new(context: aggregate::Context<A>) -> Self {
        Self {
            context,
            _marker: PhantomData,
        }
    }
}

impl<'s, A, S> EventExecutor<'s> for EventCommiter<A, S>
where
    A: aggregate::Aggregate + aggregate::Apply + 'static,
    S: EventWriter<A> + 'static,
{
    type Ret = ();
    type Error = EventCommiterError;
    type Store = S;
    async fn execute(&'s mut self, store: &'s Self::Store) -> Result<Self::Ret, Self::Error> {
        let events = std::mem::take(&mut self.context.events);
        let id = self.context.id();
        store.save(id, events).await?;
        Ok(())
    }
}
