use anyhow::anyhow;
use figment::{self, providers::Format};
use crate::config::JimoConfStorage;
use crate::error::JimoConfStorageError;

pub struct JimoFigmentStorage {
    inner: figment::Figment,
}

#[async_trait::async_trait]
impl JimoConfStorage for JimoFigmentStorage {
    async fn get(
        &self,
        name: &str,
    ) -> Result<Option<serde_json::Value>, JimoConfStorageError> {
        let v = self
            .inner
            .extract_inner_lossy(name)
            .map_err(|e| anyhow!(e))?;
        Ok(Some(v))
    }
    async fn set(
        &self,
        _name: &str,
        _value: &serde_json::Value,
    ) -> Result<(), JimoConfStorageError> {
        Err(JimoConfStorageError::Any(anyhow!(
            "figment storage not support set"
        )))
    }
}
impl JimoFigmentStorage {
    pub fn builder() -> JimoConfStorageBuilder {
        JimoConfStorageBuilder::default()
    }
}

#[derive(Default)]
pub struct JimoConfStorageBuilder {
    path: Option<String>,
}

impl JimoConfStorageBuilder {
    pub fn path(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self
    }
    pub fn build(self) -> Result<JimoFigmentStorage, JimoConfStorageError> {
        let path = self
            .path
            .ok_or(JimoConfStorageError::Any(anyhow!("path is required")))?;
        let figment = figment::Figment::from(figment::providers::Toml::file(path));
        Ok(JimoFigmentStorage { inner: figment })
    }
}
