mod event;

use anyhow::{Context, anyhow};
use std::collections::HashSet;
use ulid::Ulid;

pub use event::Event;

use crate::common::aggregate::{Aggregate, Apply, ApplyError, Key};

pub enum RoleKey {
    Name(String),
}

impl Key for RoleKey {
    fn unique(&self) -> bool {
        match self {
            Self::Name(..) => true,
        }
    }
}

#[derive(Clone, Default)]
pub struct Role {
    id: Ulid,
    pub name: String,
    pub description: String,
    pub perms: HashSet<Ulid>,
    pub parent: Option<Ulid>,
}

impl Aggregate for Role {
    type ID = Ulid;
    type Event = Event;
    type Key = RoleKey;
    fn id(&self) -> &Self::ID {
        &self.id
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RoleApplyError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl ApplyError for RoleApplyError {
    fn aggregate_type(&self) -> &'static str {
        Role::ty()
    }
}

impl Apply for Role {
    type Error = RoleApplyError;
    fn apply(aggregate: &mut Option<Self>, event: &Self::Event) -> Result<(), Self::Error> {
        match event {
            Event::Created {
                id,
                name,
                description,
                parent,
            } => {
                aggregate.is_none().ok_or(anyhow!("Role must be none"))?;
                let role = Role {
                    id: *id,
                    name: name.to_string(),
                    description: description.to_string(),
                    parent: *parent,
                    ..Default::default()
                };
                aggregate.replace(role);
            }
            Event::Deleted => {
                aggregate.take();
            }
            Event::AssignedPerms(perms) => {
                let aggregate = aggregate.as_mut().context("Role can't be none")?;
                aggregate.perms.extend(perms);
            }
            Event::RemovedRoles(perms) => {
                let aggregate = aggregate.as_mut().context("Role can't be none")?;
                aggregate.perms.retain(|p| !perms.contains(p));
            }
        }
        Ok(())
    }
}
