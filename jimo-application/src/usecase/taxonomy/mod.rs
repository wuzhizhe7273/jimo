mod dto;
mod error;

use anyhow::anyhow;
use ulid::Ulid;

use jimo_domain::{
    aggregate::{Taxonomy, taxonomy::TaxonomyKey},
    common::{
        EventReader, EventWriter, QueryByKey,
        aggregate::{Aggregate, Context, EventExecutor},
        inline::InlineProjectionStore,
    },
};

use crate::usecase::taxonomy::{dto::CreateTaxonomy, error::TaxonomyError};

pub struct TaxonomyCase<S>
where
    S: EventReader<Taxonomy> + EventWriter<Taxonomy> + InlineProjectionStore<Taxonomy> + 'static,
{
    store: S,
}

impl<S> TaxonomyCase<S>
where
    S: EventReader<Taxonomy> + EventWriter<Taxonomy> + InlineProjectionStore<Taxonomy> + 'static,
{
    pub async fn create_taxonomy<Q>(&self, input: CreateTaxonomy) -> Result<(), TaxonomyError>
    where
        Q: QueryByKey<TaxonomyKey, Option<Taxonomy>, S, TaxonomyError>,
    {
        Q::by_key(TaxonomyKey::Path(input.path.clone()))
            .execute(&self.store)
            .await?
            .is_none()
            .ok_or(anyhow!("alreadyexists"))?;
        let parent_path = input.path.parent().unwrap().to_path_buf();
        Q::by_key(TaxonomyKey::Path(parent_path))
            .execute(&self.store)
            .await?
            .ok_or(anyhow!("parent not exists"))?;
        let id = Ulid::new();
        let event = <Taxonomy as Aggregate>::Event::Created {
            id,
            name: input.name,
            description: input.description,
            path: input.path,
        };
        let mut ctx = Context::<Taxonomy>::empty(id);
        ctx.apply(vec![event]).map_err(TaxonomyError::apply)?;
        ctx.commit().execute(&self.store).await.unwrap();
        Ok(())
    }

    pub async fn delete_taxonomy(&self, id: Ulid) -> Result<(), TaxonomyError> {
        let mut ctx = Context::<Taxonomy>::fetch(id).execute(&self.store).await?;
        if ctx.as_aggregate().is_none() {
            return Err(anyhow::anyhow!("taxonomy not found").into());
        }
        let event = <Taxonomy as Aggregate>::Event::Deleted;
        ctx.apply(vec![event]).map_err(TaxonomyError::apply)?;
        ctx.commit().execute(&self.store).await.unwrap();
        Ok(())
    }
}
