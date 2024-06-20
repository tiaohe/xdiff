use anyhow::Ok;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;

use crate::{ExtraArgs, RequestProfile};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffConfig {
    #[serde(flatten)]
    pub profiles: HashMap<String, DiffProfile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffProfile {
    pub req1: RequestProfile,
    pub req2: RequestProfile,
    pub res: ResponseProfile,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skup_body: Vec<String>,
}

impl DiffConfig {
    pub async fn load_yaml(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }
    pub fn from_yaml(content: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(content)?)
    }
    pub fn get_profile(&self, name: &str) -> Option<&DiffProfile> {
        self.profiles.get(name)
    }
}

impl DiffProfile {
    pub async fn diff(&self, args: ExtraArgs) -> Result<String> {
        // let res1 = req1.send(&arg).await?;
        // let res2 = req2.send(&arg).await?;

        // let text1 = res1.filter_text(&self.res).await?;
        // let text2 = res2.filter_text(&self.res).await?;

        // text_diff(&text1, &text2)
        println!("profile: {:?}", self);
        println!("args: {:?}", args);
        Ok("".to_string())
    }
}
