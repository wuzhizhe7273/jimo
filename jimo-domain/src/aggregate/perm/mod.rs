mod code;
mod event;

use anyhow::anyhow;
use ulid::Ulid;

pub use code::PermCode;
pub use event::Event;

use crate::common::aggregate::{Aggregate, Apply, ApplyError, Key};

pub enum PermUniqueKey {
    Name(String),
    Code(PermCode),
}

impl Key for PermUniqueKey {
    fn unique(&self) -> bool {
        match self {
            Self::Name(..) => true,
            Self::Code(..) => true,
        }
    }
}

#[derive(Clone)]
pub struct Perm {
    id: Ulid,
    pub name: String,
    pub description: String,
    pub code: PermCode,
}

impl Default for Perm {
    fn default() -> Self {
        Self {
            id: Ulid::new(),
            name: String::new(),
            description: String::new(),
            code: PermCode::empty(),
        }
    }
}

impl Aggregate for Perm {
    type ID = Ulid;
    type Event = Event;
    type Key = PermUniqueKey;
    fn id(&self) -> &Self::ID {
        &self.id
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PermApplyError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl ApplyError for PermApplyError {
    fn aggregate_type(&self) -> &'static str {
        Perm::ty()
    }
}

impl Apply for Perm {
    type Error = PermApplyError;
    fn apply(aggregate: &mut Option<Self>, event: &Self::Event) -> Result<(), Self::Error> {
        match event {
            Event::Created {
                id,
                name,
                description,
                code,
            } => {
                aggregate.is_none().ok_or(anyhow!("Perm must be none"))?;
                let perm = Perm {
                    id: *id,
                    name: name.to_string(),
                    description: description.to_string(),
                    code: code.clone(),
                };
                aggregate.replace(perm);
            }
            Event::Deleted => {
                aggregate.take();
            }
        }
        Ok(())
    }
}
