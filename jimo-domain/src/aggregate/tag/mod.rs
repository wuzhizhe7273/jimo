mod event;

use anyhow::{Context, anyhow};
use ulid::Ulid;

pub use event::Event;

use crate::common::aggregate::{Aggregate, Apply, ApplyError, Key};

pub enum TagKey {
    Name(String),
}
impl Key for TagKey {
    fn unique(&self) -> bool {
        match self {
            Self::Name(..) => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: Ulid,
    pub name: String,
    pub description: String,
}

impl Aggregate for Tag {
    type ID = Ulid;
    type Event = event::Event;
    type Key = TagKey;
    fn id(&self) -> &Self::ID {
        &self.id
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TagApplyError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl ApplyError for TagApplyError {
    fn aggregate_type(&self) -> &'static str {
        Tag::ty()
    }
}

impl Apply for Tag {
    type Error = TagApplyError;
    fn apply(aggregate: &mut Option<Self>, event: &Self::Event) -> Result<(), Self::Error> {
        match event {
            Event::Created {
                id,
                name,
                description,
            } => {
                aggregate.is_none().ok_or(anyhow!("Tag must be none"))?;
                let tag = Tag {
                    id: *id,
                    name: name.clone(),
                    description: description.clone(),
                };
                aggregate.replace(tag);
            }
            Event::Updated { name, description } => {
                let aggregate = aggregate.as_mut().context("Tag can't be none")?;
                aggregate.name = name.clone();
                aggregate.description = description.clone();
            }
            Event::Deleted => {
                aggregate.take();
            }
        }
        Ok(())
    }
}
