use std::collections::HashSet;

use ulid::Ulid;

pub struct RegisterWithEmail {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct LoginWithEmail {
    pub email: String,
    pub password: String,
}

pub struct CreateRole {
    pub name: String,
    pub description: String,
    pub parent: Option<Ulid>,
}

pub struct DeleteRole {
    pub role_id: Ulid,
}

pub struct AssignPermsToRole {
    pub role_id: Ulid,
    pub perm_ids: HashSet<Ulid>,
}
pub struct RemovePermsFromRole {
    pub role_id: Ulid,
    pub perm_ids: HashSet<Ulid>,
}

pub struct CreatePerm {
    pub name: String,
    pub description: String,
    pub code: String,
}

pub struct DeletePerm {
    pub perm_id: Ulid,
}

pub struct UpdatePerm {
    pub perm_id: Ulid,
    pub name: String,
    pub description: String,
}
