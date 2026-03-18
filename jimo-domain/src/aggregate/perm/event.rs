use ulid::Ulid;

use crate::{aggregate::perm::code::PermCode, common::aggregate::Event as AggregateEvent};

#[derive(Debug, Clone)]
pub enum Event {
    Created {
        id: Ulid,
        name: String,
        description: String,
        code: PermCode,
    },
    Deleted,
}

impl AggregateEvent for Event {
    fn kind(&self) -> &'static str {
        match self {
            Event::Created { .. } => "PERM.CREATED",
            Event::Deleted => "PERM.DELETED",
        }
    }
}
