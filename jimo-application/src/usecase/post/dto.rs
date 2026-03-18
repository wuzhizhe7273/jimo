use std::path::PathBuf;

use jimo_domain::projection::{PostAuthor, PostTag, PostTaxonomy, PostView};
use ulid::Ulid;
use url::Url;

pub struct CreatePost {
    pub title: String,
    pub author: Ulid,
    pub taxonomy: Vec<Ulid>,
    pub hero: Option<Url>,
    pub format: Option<String>,
    pub content: String,
}

pub struct UpdatePost {
    pub id: Ulid,
    pub title: String,
    pub taxonomy: Vec<Ulid>,
    pub hero: Option<Url>,
    pub format: Option<String>,
    pub content: String,
}

pub struct PostAuthorItem {
    pub id: Ulid,
    pub username: String,
    pub nickname: String,
}

impl From<PostAuthor> for PostAuthorItem {
    fn from(author: PostAuthor) -> Self {
        PostAuthorItem {
            id: author.id,
            username: author.username,
            nickname: author.nickname,
        }
    }
}

pub struct PostTagItem {
    pub id: Ulid,
    pub name: String,
}

impl From<PostTag> for PostTagItem {
    fn from(tag: PostTag) -> Self {
        PostTagItem {
            id: tag.id,
            name: tag.name,
        }
    }
}

pub struct PostTaxonomyItem {
    pub id: Ulid,
    pub name: String,
    pub path: PathBuf,
}

impl From<PostTaxonomy> for PostTaxonomyItem {
    fn from(taxonomy: PostTaxonomy) -> Self {
        PostTaxonomyItem {
            id: taxonomy.id,
            name: taxonomy.name,
            path: taxonomy.path,
        }
    }
}

pub struct PostItem {
    pub id: Ulid,
    pub title: String,
    pub author: PostAuthorItem,
    pub tags: Vec<PostTagItem>,
    pub taxonomies: Vec<PostTaxonomyItem>,
    pub hero: Option<Url>,
    pub format: Option<String>,
    pub content: String,
}

impl From<PostView> for PostItem {
    fn from(view: PostView) -> Self {
        PostItem {
            id: view.id,
            title: view.title,
            author: view.author.into(),
            tags: view.tags.into_iter().map(|t| t.into()).collect(),
            taxonomies: view.taxonomies.into_iter().map(|t| t.into()).collect(),
            hero: view.hero,
            format: view.format,
            content: view.content,
        }
    }
}
