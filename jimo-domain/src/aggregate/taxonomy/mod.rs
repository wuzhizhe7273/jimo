mod event;

use std::path::PathBuf;

use anyhow::{Context, anyhow};
use ulid::Ulid;

pub use event::Event;

use crate::common::aggregate::{Aggregate, Apply, ApplyError, Key};

pub enum TaxonomyKey {
    Name(String),
    Path(PathBuf),
}
impl Key for TaxonomyKey {
    fn unique(&self) -> bool {
        match self {
            Self::Name(..) => true,
            Self::Path(..) => true,
        }
    }
}

#[derive(Clone)]
pub struct Taxonomy {
    pub id: Ulid,
    pub name: String,
    pub description: String,
    pub path: PathBuf,
}

impl Aggregate for Taxonomy {
    type ID = Ulid;
    type Event = Event;
    type Key = TaxonomyKey;
    fn id(&self) -> &Self::ID {
        &self.id
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TaxonomyApplyError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl ApplyError for TaxonomyApplyError {
    fn aggregate_type(&self) -> &'static str {
        Taxonomy::ty()
    }
}

impl Apply for Taxonomy {
    type Error = TaxonomyApplyError;
    fn apply(aggregate: &mut Option<Self>, event: &Self::Event) -> Result<(), Self::Error> {
        match event {
            Event::Created {
                id,
                name,
                description,
                path,
            } => {
                aggregate
                    .is_none()
                    .ok_or(anyhow!("Taxonomy must be none"))?;
                let taxonomy = Taxonomy {
                    id: *id,
                    name: name.to_string(),
                    description: description.to_string(),
                    path: path.clone(),
                };
                aggregate.replace(taxonomy);
            }
            Event::Updated { name, description } => {
                let aggregate = aggregate.as_mut().context("Taxonomy can't be none")?;
                if let Some(name) = name {
                    aggregate.name = name.to_string();
                }
                if let Some(description) = description {
                    aggregate.description = description.to_string();
                }
            }
            Event::Deleted => {
                aggregate.take();
            }
        }
        Ok(())
    }
}
