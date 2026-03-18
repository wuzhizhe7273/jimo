use ulid::Ulid;

use crate::common::aggregate::Event as AggregateEvent;

#[derive(Debug, Clone)]
pub enum Event {
    Created {
        id: Ulid,
        name: String,
        description: String,
    },
    Updated {
        name: String,
        description: String,
    },
    Deleted,
}

impl AggregateEvent for Event {
    fn kind(&self) -> &'static str {
        match self {
            Self::Created { .. } => "TAG.CREATED",
            Self::Updated { .. } => "TAG.UPDATED",
            Self::Deleted => "TAG.DELETED",
        }
    }
}
