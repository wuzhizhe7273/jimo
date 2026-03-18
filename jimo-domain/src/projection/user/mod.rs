use ulid::Ulid;

use crate::{
    aggregate::{User, UserProfile},
    common::{
        Projection, ProjectionStore, Projector, aggregate::Key, projection::multi::MultiProjection,
    },
};

pub struct UserView {
    pub id: Ulid,
    pub username: String,
    pub email: Option<String>,
    pub nickname: String,
    pub avatar: Option<String>,
}

impl Projection for UserView {
    type ID = Ulid;
    fn id(&self) -> &Self::ID {
        &self.id
    }
}
pub enum UserViewKey {
    Username(String),
    Email(String),
    Nickname(String),
}

impl Key for UserViewKey {
    fn unique(&self) -> bool {
        match self {
            Self::Email(..) => true,
            Self::Username(..) => true,
            Self::Nickname(..) => false,
        }
    }
}
impl MultiProjection for UserView {
    type Key = UserViewKey;
}

pub trait UserViewStore: ProjectionStore + Projector<User> + Projector<UserProfile> {}
