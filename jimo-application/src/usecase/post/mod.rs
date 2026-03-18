mod dto;
mod error;

use anyhow::anyhow;
use ulid::Ulid;

use jimo_domain::{
    aggregate::post::{Event as PostEvent, Post, PostKey},
    common::{
        Context, EventReader, EventWriter, QueryByKey, aggregate::EventExecutor,
        inline::InlineProjectionStore,
    },
    projection::PostViewStore,
};

use crate::usecase::post::{
    dto::{CreatePost, UpdatePost},
    error::PostError,
};

pub struct PostCase<S> {
    store: S,
}

impl<S> PostCase<S>
where
    S: EventReader<Post> + EventWriter<Post> + InlineProjectionStore<Post> + 'static,
{
    pub async fn create_post<Q>(&self, input: &CreatePost) -> Result<(), PostError>
    where
        Q: QueryByKey<PostKey, Option<Post>, S, PostError>,
    {
        Q::by_key(PostKey::Title(input.title.clone()))
            .execute(&self.store)
            .await?
            .is_none()
            .ok_or(anyhow!("title is already exists"))?;
        let id = Ulid::new();
        let event = PostEvent::Created {
            id,
            title: input.title.clone(),
            author: input.author,
            taxonomy: input.taxonomy.clone(),
            hero: input.hero.clone(),
            format: input.format.clone(),
            content: input.content.clone(),
        };
        let mut ctx = Context::<Post>::empty(id);
        ctx.apply(vec![event]).map_err(PostError::apply)?;
        ctx.commit().execute(&self.store).await?;
        Ok(())
    }

    pub async fn delete_post(&self, id: Ulid) -> Result<(), PostError> {
        let mut ctx = Context::<Post>::fetch(id).execute(&self.store).await?;
        ctx.apply(vec![PostEvent::Deleted])
            .map_err(PostError::apply)?;
        ctx.commit().execute(&self.store).await?;
        Ok(())
    }
    pub async fn user_delete_post(&self, id: Ulid, author: Ulid) -> Result<(), PostError> {
        let mut ctx = Context::<Post>::fetch(id).execute(&self.store).await?;
        ctx.apply(vec![PostEvent::DeletedByAuthor { author }])
            .map_err(PostError::apply)?;
        ctx.commit().execute(&self.store).await?;
        Ok(())
    }

    pub async fn update_post<Q>(&self, input: UpdatePost) -> Result<(), PostError>
    where
        Q: QueryByKey<PostKey, Option<Post>, S, PostError>,
    {
        let mut ctx = Context::<Post>::fetch(input.id)
            .execute(&self.store)
            .await?;
        let aggregate = ctx.as_aggregate().ok_or(anyhow!("post not exists"))?;
        (aggregate.title != input.title
            && Q::by_key(PostKey::Title(input.title.clone()))
                .execute(&self.store)
                .await?
                .is_some())
        .ok_or(anyhow!("title is already exists"))?;
        let event = PostEvent::Updated {
            title: input.title.to_string(),
            hero: input.hero.clone(),
            taxonomy: input.taxonomy.clone(),
            format: input.format.clone(),
            content: input.content.clone(),
        };
        ctx.apply(vec![event]).map_err(PostError::apply)?;
        ctx.commit().execute(&self.store).await?;
        Ok(())
    }
    pub async fn user_update_post(&self, input: UpdatePost, author: Ulid) -> Result<(), PostError> {
        let mut ctx = Context::<Post>::fetch(input.id)
            .execute(&self.store)
            .await?;
        let event = PostEvent::UpdatedByAuthor {
            author,
            title: input.title.to_string(),
            hero: input.hero.clone(),
            taxonomy: input.taxonomy.clone(),
            format: input.format.clone(),
            content: input.content.clone(),
        };
        ctx.apply(vec![event]).map_err(PostError::apply)?;
        ctx.commit().execute(&self.store).await?;
        Ok(())
    }
}

impl<S> PostCase<S>
where
    S: PostViewStore,
{
    pub async fn get(&self, _id: Ulid) -> Result<(), PostError> {
        todo!()
    }
}
