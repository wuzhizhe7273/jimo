use std::collections::HashSet;

use ulid::Ulid;

use crate::common::aggregate::Event as AggregateEvent;

#[derive(Clone)]
pub enum Event {
    Created {
        id: Ulid,
        name: String,
        description: String,
        parent: Option<Ulid>,
    },
    Deleted,
    AssignedPerms(HashSet<Ulid>),
    RemovedRoles(HashSet<Ulid>),
}

impl AggregateEvent for Event {
    fn kind(&self) -> &'static str {
        match self {
            Event::Created { .. } => "ROLE.CREATED",
            Event::Deleted => "ROLE.DELETED",
            Event::AssignedPerms(_) => "ROLE.ASSIGNED_PERMS",
            Event::RemovedRoles(_) => "ROLE.REMOVED_PERMS",
        }
    }
}
