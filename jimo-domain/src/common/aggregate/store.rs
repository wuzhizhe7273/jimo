use crate::common::aggregate::{
    Aggregate,
    error::{EventReaderError, EventWriterError},
    event::Envelope,
};

pub trait EventReader<A: Aggregate> {
    fn stream(
        &self,
        id: &A::ID,
        version: u64,
    ) -> Result<impl futures::Stream<Item = Envelope<A>> + Send, EventReaderError>;
}

pub trait EventWriter<A: Aggregate> {
    async fn save(&self, id: &A::ID, events: Vec<Envelope<A>>) -> Result<(), EventWriterError>;
}
