use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::common::aggregate::Event as AggregateEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Created {
        id: Ulid,
        name: String,
        description: String,
        path: PathBuf,
    },
    Updated {
        name: Option<String>,
        description: Option<String>,
    },
    Deleted,
}

impl AggregateEvent for Event {
    fn kind(&self) -> &'static str {
        match self {
            Event::Created { .. } => "TAXONOMY.CREATED",
            Event::Updated { .. } => "TAXONOMY.UPDATED",
            Event::Deleted => "TAXONOMY.DELETED",
        }
    }
}
