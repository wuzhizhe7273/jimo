mod dto;
mod error;
mod interface;

use anyhow::{Context as AnyhowContext, anyhow};
use ulid::Ulid;

use jimo_domain::{
    aggregate::{
        Perm, Role, User,
        perm::{Event as PermEvent, PermCode},
        role::Event as RoleEvent,
        user::{Event as UserEvent, UserKey},
    },
    common::{
        EventReader, EventWriter, QueryByKey,
        aggregate::{Context, EventExecutor},
        inline::InlineProjectionStore,
    },
};

use crate::usecase::iam::{
    dto::{
        AssignPermsToRole, CreatePerm, CreateRole, DeletePerm, DeleteRole, LoginWithEmail,
        RemovePermsFromRole,
    },
    error::{AuthenticateError, IAMError},
    interface::{PasswordHasher, TokenGenerator},
};

pub struct IAMCase<S, Hasher, Tokener>
where
    S: EventReader<User>
        + EventWriter<User>
        + EventReader<Role>
        + EventWriter<Role>
        + EventReader<Perm>
        + EventWriter<Perm>
        + InlineProjectionStore<User>
        + InlineProjectionStore<Role>
        + InlineProjectionStore<Perm>
        + 'static,
    Hasher: PasswordHasher,
    Tokener: TokenGenerator,
{
    store: S,
    hasher: Hasher,
    tonker: Tokener,
}

impl<S, Hasher, Tokener> IAMCase<S, Hasher, Tokener>
where
    S: EventWriter<User>
        + EventReader<User>
        + InlineProjectionStore<User>
        + EventWriter<Role>
        + EventReader<Role>
        + InlineProjectionStore<Role>
        + EventWriter<Perm>
        + EventReader<Perm>
        + InlineProjectionStore<Perm>
        + 'static,
    Hasher: PasswordHasher,
    Tokener: TokenGenerator,
{
    pub async fn register<Q>(&self, input: &dto::RegisterWithEmail) -> Result<(), IAMError>
    where
        Q: QueryByKey<UserKey, Option<User>, S, IAMError>,
    {
        let phc = self.hasher.hash(&input.password).await?;
        Q::by_key(UserKey::Username(input.username.clone()))
            .execute(&self.store)
            .await?
            .is_none()
            .ok_or(anyhow!("user is already exists"))?;
        Q::by_key(UserKey::Email(input.email.clone()))
            .execute(&self.store)
            .await?
            .is_none()
            .ok_or(anyhow!("email is already exists"))?;
        let id = Ulid::new();
        let event = UserEvent::RegisteredByEmail {
            id,
            username: input.username.to_string(),
            email: input.email.to_string(),
            phc: Some(phc.to_string()),
        };
        let mut user = Context::<User>::empty(id);
        user.apply(vec![event]).map_err(IAMError::apply)?;
        user.commit().execute(&self.store).await.unwrap();
        Ok(())
    }

    pub async fn login_with_email<Q>(&self, input: LoginWithEmail) -> Result<String, IAMError>
    where
        Q: QueryByKey<UserKey, Option<User>, S, IAMError>,
    {
        let user = Q::by_key(UserKey::Email(input.email))
            .execute(&self.store)
            .await?
            .context("User not exists")?;
        let phc = user.phc.context("phc not exists")?;
        self.hasher
            .verify(&phc, &input.password)
            .await?
            .ok_or(anyhow!("password conflict"))?;
        let payload = (user.id, user.roles);
        let token = self
            .tonker
            .gen_token(serde_json::json!(payload))
            .await
            .context("gen token failed")?;
        Ok(token)
    }
}

impl<S, Hasher, Tokener> IAMCase<S, Hasher, Tokener>
where
    S: EventWriter<User>
        + EventReader<User>
        + InlineProjectionStore<User>
        + EventWriter<Role>
        + EventReader<Role>
        + InlineProjectionStore<Role>
        + EventWriter<Perm>
        + EventReader<Perm>
        + InlineProjectionStore<Perm>
        + 'static,
    Hasher: PasswordHasher,
    Tokener: TokenGenerator,
{
    pub async fn delete_user(&self, id: Ulid) -> Result<(), IAMError> {
        let mut ctx = Context::<User>::fetch(id).execute(&self.store).await?;
        ctx.apply(vec![UserEvent::Deleted])
            .map_err(IAMError::apply)?;
        ctx.commit().execute(&self.store).await?;
        Ok(())
    }
}

impl<S, Hasher, Tokener> IAMCase<S, Hasher, Tokener>
where
    S: EventWriter<User>
        + EventReader<User>
        + InlineProjectionStore<User>
        + EventWriter<Role>
        + EventReader<Role>
        + InlineProjectionStore<Role>
        + EventWriter<Perm>
        + EventReader<Perm>
        + InlineProjectionStore<Perm>
        + 'static,
    Hasher: PasswordHasher,
    Tokener: TokenGenerator,
{
    pub async fn check_perm(&self, id: Ulid, code: &str) -> Result<bool, IAMError> {
        let user = Context::<User>::fetch(id)
            .execute(&self.store)
            .await?
            .to_aggregate()
            .ok_or(AuthenticateError::InvalidCredentials)?;
        let mut roles = vec![];
        for role in user.roles {
            let role = Context::<Role>::fetch(role)
                .execute(&self.store)
                .await?
                .to_aggregate();
            if let Some(role) = role {
                roles.push(role);
            }
        }
        let mut perms = vec![];
        for perm in roles.iter().flat_map(|r| r.perms.clone()) {
            let perm = Context::<Perm>::fetch(perm)
                .execute(&self.store)
                .await?
                .to_aggregate();
            if let Some(perm) = perm {
                perms.push(perm);
            }
        }
        let code =
            PermCode::try_from_str(code).map_err(|_| AuthenticateError::InvalidCredentials)?;
        Ok(perms.into_iter().map(|p| p.code).any(|c| c == code))
    }
}

impl<S, Hasher, Tokener> IAMCase<S, Hasher, Tokener>
where
    S: EventWriter<User>
        + EventReader<User>
        + InlineProjectionStore<User>
        + EventWriter<Role>
        + EventReader<Role>
        + InlineProjectionStore<Role>
        + EventWriter<Perm>
        + EventReader<Perm>
        + InlineProjectionStore<Perm>
        + 'static,
    Hasher: PasswordHasher,
    Tokener: TokenGenerator,
{
    pub async fn create_role(&self, input: CreateRole) -> Result<(), IAMError> {
        let id = Ulid::new();
        let event = RoleEvent::Created {
            id,
            name: input.name,
            description: input.description,
            parent: input.parent,
        };
        let mut role = Context::<Role>::empty(id);
        role.apply(vec![event]).map_err(IAMError::apply)?;
        role.commit().execute(&self.store).await.unwrap();
        Ok(())
    }

    pub async fn delete_role(&self, input: DeleteRole) -> Result<(), IAMError> {
        let mut role = Context::<Role>::fetch(input.role_id)
            .execute(&self.store)
            .await?;
        if role.as_aggregate().is_none() {
            return Err(anyhow!("role not found").into());
        }
        let event = RoleEvent::Deleted;
        role.apply(vec![event]).map_err(IAMError::apply)?;
        role.commit().execute(&self.store).await.unwrap();
        Ok(())
    }

    pub async fn assign_perms_to_role(&self, input: AssignPermsToRole) -> Result<(), IAMError> {
        let mut role = Context::<Role>::fetch(input.role_id)
            .execute(&self.store)
            .await?;
        if role.as_aggregate().is_none() {
            return Err(anyhow!("role not found").into());
        }
        let event = RoleEvent::AssignedPerms(input.perm_ids);
        role.apply(vec![event]).map_err(IAMError::apply)?;
        role.commit().execute(&self.store).await.unwrap();
        Ok(())
    }

    pub async fn remove_perms_from_role(&self, input: RemovePermsFromRole) -> Result<(), IAMError> {
        let mut role = Context::<Role>::fetch(input.role_id)
            .execute(&self.store)
            .await?;
        if role.as_aggregate().is_none() {
            return Err(anyhow!("role not found").into());
        }
        let event = RoleEvent::RemovedRoles(input.perm_ids);
        role.apply(vec![event]).map_err(IAMError::apply)?;
        role.commit().execute(&self.store).await.unwrap();
        Ok(())
    }
}

impl<S, Hasher, Tokener> IAMCase<S, Hasher, Tokener>
where
    S: EventWriter<User>
        + EventReader<User>
        + InlineProjectionStore<User>
        + EventWriter<Role>
        + EventReader<Role>
        + InlineProjectionStore<Role>
        + EventWriter<Perm>
        + EventReader<Perm>
        + InlineProjectionStore<Perm>
        + 'static,
    Hasher: PasswordHasher,
    Tokener: TokenGenerator,
{
    pub async fn create_perm(&self, input: CreatePerm) -> Result<(), IAMError> {
        let code =
            PermCode::try_from_str(&input.code).map_err(|e| anyhow!("invalid perm code: {}", e))?;
        let id = Ulid::new();
        let event = PermEvent::Created {
            id,
            name: input.name,
            description: input.description,
            code,
        };
        let mut perm = Context::<Perm>::empty(id);
        perm.apply(vec![event]).map_err(IAMError::apply)?;
        perm.commit().execute(&self.store).await.unwrap();
        Ok(())
    }

    pub async fn delete_perm(&self, input: DeletePerm) -> Result<(), IAMError> {
        let mut perm = Context::<Perm>::fetch(input.perm_id)
            .execute(&self.store)
            .await?;
        if perm.as_aggregate().is_none() {
            return Err(anyhow!("perm not found").into());
        }
        let event = PermEvent::Deleted;
        perm.apply(vec![event]).map_err(IAMError::apply)?;
        perm.commit().execute(&self.store).await.unwrap();
        Ok(())
    }
}
