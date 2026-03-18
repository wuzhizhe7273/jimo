#[derive(Debug, thiserror::Error)]
pub enum PasswordHasherError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub trait PasswordHasher {
    async fn hash(&self, pwd: &str) -> Result<&str, PasswordHasherError>;
    async fn verify(&self, phc: &str, pwd: &str) -> Result<bool, PasswordHasherError>;
}

#[derive(Debug, thiserror::Error)]
pub enum TokenGeneratorError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub trait TokenGenerator {
    async fn gen_token(&self, payload: serde_json::Value) -> Result<String, TokenGeneratorError>;
}
