use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct PluginConfig{
    path:String
}
impl PluginConfig{
    pub fn path(&self)->&str{
        &self.path
    }
}

impl jimo_conf::JomoConfItem for PluginConfig{
    fn name() -> &'static str {
        "plugin"
    }
}