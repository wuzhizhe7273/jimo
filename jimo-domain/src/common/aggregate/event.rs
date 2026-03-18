use serde::{Deserialize, Serialize};

use crate::common::aggregate::Aggregate;

pub trait Event: Clone {
    fn kind(&self) -> &'static str;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Envelope<A>
where
    A: Aggregate,
{
    event: A::Event,
    version: u64,
}

impl<A> Envelope<A>
where
    A: Aggregate,
{
    pub fn new(event: A::Event, version: u64) -> Self {
        Self { event, version }
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn event(self) -> A::Event {
        self.event
    }
}
