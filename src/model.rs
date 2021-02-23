use serde::{Deserialize, Serialize};
use std::{collections::HashMap};

#[derive(Debug, Serialize, Deserialize)]
pub struct DAG {
    pub details: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Detail {
    pub rule: serde_yaml::Value,
    #[serde(default = "default_display_name")]
    pub display_name: String,
    pub description: String,
    pub required: bool,
    #[serde(default = "default_message")]
    pub message: HashMap<String, String>,
}

fn default_message()->HashMap<String, String>{
    HashMap::new()
}

fn default_display_name()->String{
    String::from("")
}


impl DAG {
    pub fn from_str(s: &str) -> Result<HashMap<String, Detail>, serde_yaml::Error> {
        let mut res = HashMap::new();
        let v: serde_yaml::Value = serde_yaml::from_str(s)?;
        match v {
            serde_yaml::Value::Mapping(v) => {
                for (key, value) in v {
                    match key {
                        serde_yaml::Value::String(s) => {
                            let b: Detail = serde_yaml::from_value(value)?;
                            res.insert(s, b);
                        }
                        _ => println!("unkown type: {:?}", key),
                    }
                }
            }
            _ => println!("unkown type : {:?}", v),
        }
        Ok(res)
    }
}

// use std::time::*;
// #[derive(Debug)]
// pub struct Pipeline {
//     name: String,
//     version: String,
//     display_name: String,
//     triggers: HashMap<String, Trigger>,
//     variables: HashMap<String, String>,
//     stages: [Stage],
// }

// pub struct Trigger {
// 	branches: [String] ,

// pub struct Stage{
// 	Name        :String         ,
// 	DisplayName :String         ,
// 	Stage       :String         ,
// 	Status      :String         ,
// 	Error       :String         ,
// 	ExitCode    :i32            ,
// 	Started     :time::Da     ,
// 	Stopped     :time.Time      ,
// 	Finished    :time.Time      ,
// 	Created     :time.Time      ,
// 	Updated     :time.Time      ,
// 	Version     :String         ,
// 	OnSuccess   :bool           ,
// 	OnFailure   :bool           ,
// 	Labels      :HashMap<String, String>,
// 	Jobs        :[Job]          ,
// }

// type Job struct {
// 	Job             string            `yaml:"job" json:"job"`
// 	DisplayName     string            `yaml:"displayName" json:"displayName"`
// 	Name            string            `yaml:"name" json:"name"`
// 	Environments    map[string]string `yaml:"environments" json:"environments"`
// 	Commands        []interface{}     `yaml:"commands" json:"commands"`
// 	Number          int               `json:"number"`
// 	Status          string            `json:"status"`
// 	Error           string            `json:"error,omitempty"`
// 	ErrIgnore       bool              `json:"errignore,omitempty"`
// 	ExitCode        int               `json:"exit_code"`
// 	Started         time.Time         `json:"started,omitempty"`
// 	Stopped         time.Time         `json:"stopped,omitempty"`
// 	Finished        time.Time         `json:"finished"`
// 	Version         string            `json:"version"`
// 	DependsOn       []string          `yaml:"dependsOn" json:"dependsOn"`
// 	Image           string            `yaml:"image" json:"image"`
// 	Artifacts       []*Artifact       `yaml:"artifacts" json:"artifacts"`
// 	DependArtifacts []*DependArtifact `yaml:"dependArtifacts" json:"dependArtifacts"`
// }

// type Artifact struct {
// 	BuildName string `json:"buildName"`
// 	StageName string `json:"stageName"`
// 	JobName   string `json:"jobName"`

// 	Name  string `yaml:"name" json:"name"`
// 	Scope string `yaml:"scope" json:"scope"`
// 	Path  string `yaml:"path" json:"path"`

// 	Repository string `yaml:"repository" json:"repository"`

// 	Value string `yaml:"value" json:"value"`
// }

// type DependArtifact struct {
// 	Name       string `yaml:"name" json:"name"`
// 	Type       string `yaml:"type" json:"type"`
// 	IsForce    bool   `yaml:"isForce" json:"isForce"`
// 	Repository string `yaml:"repository" json:"repository"`
// 	Target     string `yaml:"target" json:"target"`

// 	SourceStage string `yaml:"sourceStage" json:"sourceStage"`
// 	SourceJob   string `yaml:"sourceJob" json:"sourceJob"`

// 	Value string `json:"value"`
// }
