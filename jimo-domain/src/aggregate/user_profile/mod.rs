mod event;

use anyhow::anyhow;
use ulid::Ulid;
use url::Url;

pub use event::Event;

use crate::common::aggregate::{Aggregate, Apply, ApplyError, Key};

pub enum UserProfileKey {}

impl Key for UserProfileKey {
    fn unique(&self) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct UserProfile {
    uid: Ulid,
    pub nickname: String,
    pub avatar: Option<Url>,
}

impl Aggregate for UserProfile {
    type ID = Ulid;
    type Event = Event;
    type Key = UserProfileKey;
    fn id(&self) -> &Self::ID {
        &self.uid
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserProfileError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl ApplyError for UserProfileError {
    fn aggregate_type(&self) -> &'static str {
        UserProfile::ty()
    }
}

impl Apply for UserProfile {
    type Error = UserProfileError;
    fn apply(aggregate: &mut Option<Self>, event: &Self::Event) -> Result<(), Self::Error> {
        match event {
            Event::Created {
                uid,
                nickname,
                avatar,
            } => {
                aggregate
                    .is_none()
                    .ok_or(anyhow!("UserProfile must none"))?;
                let profile = UserProfile {
                    uid: *uid,
                    nickname: nickname.to_string(),
                    avatar: avatar.clone(),
                };
                aggregate.replace(profile);
            }
            Event::Deleted => {
                aggregate.take();
            }
        }
        Ok(())
    }
}
