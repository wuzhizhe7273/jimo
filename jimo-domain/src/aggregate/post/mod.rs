mod event;

use anyhow::anyhow;
use ulid::Ulid;
use url::Url;

pub use event::Event;

use crate::common::{
    self,
    aggregate::{Aggregate, Apply, ApplyError, Key},
};

#[derive(Debug, Clone, Default)]
pub struct Post {
    id: Ulid,
    pub title: String,
    pub author: Ulid,
    pub tag: Vec<Ulid>,
    pub taxonomy: Vec<Ulid>,
    pub hero: Option<Url>,
    pub format: Option<String>,
    pub content: String,
}

pub enum PostKey {
    Title(String),
}

impl Key for PostKey {
    fn unique(&self) -> bool {
        match self {
            PostKey::Title(..) => true,
        }
    }
}
impl Aggregate for Post {
    type ID = Ulid;
    type Event = Event;
    type Key = PostKey;
    fn id(&self) -> &Self::ID {
        &self.id
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PostApplyError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl ApplyError for PostApplyError {
    fn aggregate_type(&self) -> &'static str {
        Post::ty()
    }
}

impl Apply for Post {
    type Error = PostApplyError;
    fn apply(aggregate: &mut Option<Self>, event: &Self::Event) -> Result<(), Self::Error> {
        match event {
            Event::Created {
                id,
                title,
                author,
                taxonomy,
                hero,
                format,
                content,
            } => {
                aggregate.is_none().ok_or(anyhow!("Post must be none"))?;
                let post = Post {
                    id: *id,
                    title: title.to_string(),
                    author: *author,
                    hero: hero.clone(),
                    taxonomy: taxonomy.clone(),
                    format: format.clone(),
                    content: content.to_string(),
                    ..Default::default()
                };
                aggregate.replace(post);
            }
            Event::Deleted => {
                aggregate.take();
            }
            Event::Updated {
                title,
                hero,
                format,
                taxonomy,
                content,
            } => {
                if let Some(aggregate) = aggregate.as_mut() {
                    aggregate.title = title.into();
                    aggregate.hero = hero.clone();
                    aggregate.format = format.clone();
                    aggregate.content = content.into();
                    aggregate.taxonomy = taxonomy.clone();
                } else {
                    return Err(anyhow!("aggregate can't be none").into());
                }
            }
            Event::UpdatedByAuthor {
                author,
                title,
                hero,
                taxonomy,
                format,
                content,
            } => {
                if let Some(aggregate) = aggregate.as_mut() {
                    if !aggregate.author.eq(&author) {
                        return Err(anyhow!("author does not match").into());
                    }
                    aggregate.title = title.into();
                    aggregate.hero = hero.clone();
                    aggregate.format = format.clone();
                    aggregate.content = content.into();
                    aggregate.taxonomy = taxonomy.clone();
                } else {
                    return Err(anyhow!("aggregate can't be none").into());
                }
            }
            Event::DeletedByAuthor { author } => {
                if let Some(aggregate) = aggregate.as_mut() {
                    if !aggregate.author.eq(&author) {
                        return Err(anyhow!("author does not match").into());
                    }
                }
                aggregate.take();
            }
        }
        Ok(())
    }
}
