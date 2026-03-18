use ulid::Ulid;
use url::Url;

use crate::common::aggregate::Event as AggregateEvent;

#[derive(Debug, Clone)]
pub enum Event {
    Created {
        uid: Ulid,
        nickname: String,
        avatar: Option<Url>,
    },
    Deleted,
}

impl AggregateEvent for Event {
    fn kind(&self) -> &'static str {
        match self {
            Event::Created { .. } => "USER_PROFILE.CREATED",
            Event::Deleted => "USER_PROFILE.DELETED",
        }
    }
}
