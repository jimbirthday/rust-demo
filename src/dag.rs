use serde::{Deserialize, Serialize};
use std::{collections::HashMap};


#[derive(Debug, Serialize, Deserialize)]
pub struct DAG {
    pub details: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Detail {
    #[serde(alias = "type")]
    pub kind : String,
    #[serde(default = "default_display_name")]
    pub display_name: String,
    pub description: String,
    pub rule: serde_yaml::Value,
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

