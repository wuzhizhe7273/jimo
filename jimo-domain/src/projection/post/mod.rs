use std::path::PathBuf;

use ulid::Ulid;
use url::Url;

use crate::{
    aggregate::{Post, Taxonomy, UserProfile, tag::Tag},
    common::{self, aggregate::Key, projection::multi::MultiProjection},
};

pub struct PostAuthor {
    pub id: Ulid,
    pub username: String,
    pub nickname: String,
}

pub struct PostTag {
    pub id: Ulid,
    pub name: String,
}

pub struct PostTaxonomy {
    pub id: Ulid,
    pub name: String,
    pub path: PathBuf,
}

pub enum PostViewUniqueKey {
    Title(String),
}

pub struct PostView {
    pub id: Ulid,
    pub title: String,
    pub author: PostAuthor,
    pub tags: Vec<PostTag>,
    pub taxonomies: Vec<PostTaxonomy>,
    pub hero: Option<Url>,
    pub format: Option<String>,
    pub content: String,
}

impl Key for PostViewUniqueKey {
    fn unique(&self) -> bool {
        match self {
            Self::Title(..) => true,
        }
    }
}

impl common::projection::Projection for PostView {
    type ID = Ulid;
    fn id(&self) -> &Self::ID {
        &self.id
    }
}
impl MultiProjection for PostView {
    type Key = PostViewUniqueKey;
}

pub trait PostViewStore:
    common::projection::ProjectionStore
    + common::projection::Projector<Post>
    + common::projection::Projector<UserProfile>
    + common::projection::Projector<Taxonomy>
    + common::projection::Projector<Tag>
{
}
