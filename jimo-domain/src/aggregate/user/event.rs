use std::collections::HashSet;

use crate::common::aggregate::Event as AggregateEvent;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Clone, Serialize, Deserialize)]
pub enum Event {
    RegisteredByEmail {
        id: Ulid,
        username: String,
        email: String,
        phc: Option<String>,
    },
    Deleted,
    AssignedRoles(HashSet<Ulid>),
    RemovedRoles(HashSet<Ulid>),
}
impl AggregateEvent for Event {
    fn kind(&self) -> &'static str {
        match self {
            Event::RegisteredByEmail { .. } => "USER.REGISTERED_BY_EMAIL",
            Event::Deleted => "USER.DELETED",
            Event::AssignedRoles(_) => "USER.ASSIGNED_ROLES",
            Event::RemovedRoles(_) => "USER.REMOVED_ROLES",
        }
    }
}
