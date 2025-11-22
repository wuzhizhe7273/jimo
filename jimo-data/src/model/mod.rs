use toasty::{Model, stmt::Id};

#[derive(Debug, Model)]
pub struct User {
    #[key]
    #[auto]
    id: Id<Self>,
    #[unique]
    username: String,
    #[unique]
    email: Option<String>,
}

#[derive(Debug, Model)]
pub struct Role {
    #[key]
    #[auto]
    id: Id<Self>,
    #[unique]
    name: String,
    description: Option<String>,
    #[index]
    parent_id: Option<Id<Self>>,
    #[belongs_to(key=parent_id, references=id)]
    parent: toasty::BelongsTo<Option<Self>>,
    #[has_many(pair=parent)]
    chidren: toasty::HasMany<Self>,
}
