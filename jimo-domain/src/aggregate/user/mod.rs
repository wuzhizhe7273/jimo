mod event;

use std::collections::HashSet;

use anyhow::{Context, anyhow};
use ulid::Ulid;

pub use event::Event;

use crate::common::aggregate::{Aggregate, Apply, ApplyError, Key};

#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: Ulid,
    pub username: String,
    pub email: Option<String>,
    pub phc: Option<String>,
    pub roles: HashSet<<crate::aggregate::role::Role as Aggregate>::ID>,
}

pub enum UserKey {
    Username(String),
    Email(String),
}

impl Key for UserKey {
    fn unique(&self) -> bool {
        match self {
            Self::Username(..) => true,
            Self::Email(..) => true,
        }
    }
}
impl Aggregate for User {
    type ID = Ulid;
    type Event = Event;
    type Key = UserKey;
    fn id(&self) -> &Self::ID {
        &self.id
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserApplyError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl ApplyError for UserApplyError {
    fn aggregate_type(&self) -> &'static str {
        User::ty()
    }
}

impl Apply for User {
    type Error = UserApplyError;
    fn apply(aggregate: &mut Option<Self>, event: &Self::Event) -> Result<(), Self::Error> {
        match event {
            Event::RegisteredByEmail {
                id,
                username,
                email,
                phc,
            } => {
                aggregate.is_none().ok_or(anyhow!("User must be none"))?;
                let user = User {
                    id: *id,
                    username: username.into(),
                    email: Some(email.into()),
                    phc: phc.clone(),
                    ..Default::default()
                };
                aggregate.replace(user);
            }
            Event::Deleted => {
                aggregate.take();
            }
            Event::AssignedRoles(roles) => {
                let aggregate = aggregate.as_mut().context("User can't be none")?;
                aggregate.roles.extend(roles);
            }
            Event::RemovedRoles(roles) => {
                let aggregate = aggregate.as_mut().context("User can't be none")?;
                aggregate.roles.retain(|r| !roles.contains(r));
            }
        }
        Ok(())
    }
}
