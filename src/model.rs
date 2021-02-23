use std::collections::HashMap;
use serde::{Deserialize, Serialize};
#[derive(Debug,  Serialize, Deserialize)]
pub struct DAG {
    pub version: Version,
}

#[derive(Debug,  Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub rule: serde_yaml::Value,
    pub display_name: String,
    pub description: String,
    pub required: bool,
    pub message: Option<HashMap<String, String>>,
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
