use ulid::Ulid;
use url::Url;

use crate::common::aggregate::Event as AggregateEvent;

#[derive(Debug, Clone)]
pub enum Event {
    Created {
        id: Ulid,
        title: String,
        author: Ulid,
        taxonomy: Vec<Ulid>,
        hero: Option<Url>,
        format: Option<String>,
        content: String,
    },
    Deleted,
    Updated {
        title: String,
        hero: Option<Url>,
        taxonomy: Vec<Ulid>,
        format: Option<String>,
        content: String,
    },
    UpdatedByAuthor {
        author: Ulid,
        title: String,
        hero: Option<Url>,
        taxonomy: Vec<Ulid>,
        format: Option<String>,
        content: String,
    },
    DeletedByAuthor {
        author: Ulid,
    },
}

impl AggregateEvent for Event {
    fn kind(&self) -> &'static str {
        match self {
            Event::Created { .. } => "POST.CREATED",
            Event::Deleted => "POST.DELETED",
            Event::Updated { .. } => "POST.UPDATED",
            Event::UpdatedByAuthor { .. } => "POST.UPDATED_BY_AUTHOR",
            Event::DeletedByAuthor { .. } => "POST.DELETED_BY_AUTHOR",
        }
    }
}
