use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pipeline {
    #[serde(default = "default_string")]
    pub name: String,
    pub version: String,
    pub display_name: String,
    #[serde(default = "default_hashmap_string")]
    pub variables: HashMap<String, String>,
    pub triggers: HashMap<String, Trigger>,
    pub stages: Vec<Stage>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trigger {
    pub branches: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stage {
    pub name: String,
    pub display_name: String,
    pub stage: String,
    pub jobs: Vec<Job>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub job: String,
    #[serde(default = "default_string")]
    pub display_name: String,
    pub name: String,
    #[serde(default = "default_hashmap_string")]
    pub environments: HashMap<String, String>,
    pub commands: serde_yaml::Value,
    #[serde(default = "default_vec_string")]
    pub depends_on: Vec<String>,
    #[serde(default = "default_vec_artifact")]
    pub artifacts: Vec<Artifact>,
    #[serde(default = "default_vec_depend_artifacts")]
    pub depend_artifacts: Vec<DependArtifact>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
    pub name: String,
    pub scope: String,
    #[serde(default = "default_string")]
    pub path: String,
    #[serde(default = "default_string")]
    pub repository: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependArtifact {
    #[serde(default = "default_string")]
    pub name: String,
    #[serde(alias = "type")]
    pub kind: String,
    pub is_force: bool,
    #[serde(default = "default_string")]
    pub repository: String,
    #[serde(default = "default_string")]
    pub target: String,
    #[serde(default = "default_string")]
    pub source_stage: String,
    #[serde(default = "default_string")]
    pub source_job: String,
    #[serde(default = "default_string")]
    pub value: String,
}

fn default_string()->String{
    String::from("")
}
fn default_vec_string()->Vec<String>{
    Vec::new()
}
fn default_vec_artifact()->Vec<Artifact>{
    Vec::new()
}
fn default_vec_depend_artifacts()->Vec<DependArtifact>{
    Vec::new()
}
fn default_hashmap_string()->HashMap<String, String>{
    HashMap::new()
}

impl Pipeline {
    pub fn from_str(s: &str) -> Result<Pipeline, serde_yaml::Error> {
        let v: Pipeline= serde_yaml::from_str(s)?;
        Ok(v)
    }
}
