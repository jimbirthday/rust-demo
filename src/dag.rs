use serde::{de::Error, Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct DAG {
    pub details: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Detail {
    #[serde(alias = "type")]
    pub kind: String,
    #[serde(default = "default_display_name")]
    pub display_name: String,
    pub description: String,
    pub rule: serde_yaml::Value,
    pub required: bool,
    #[serde(default = "default_message")]
    pub message: HashMap<String, String>,
}

fn default_message() -> HashMap<String, String> {
    HashMap::new()
}

fn default_display_name() -> String {
    String::from("")
}

impl DAG {
    pub fn from_str(s: &str) -> Result<HashMap<serde_yaml::Value, Detail>, serde_yaml::Error> {
        let mut res = HashMap::new();
        let v: serde_yaml::Value = serde_yaml::from_str(s)?;
        match v {
            serde_yaml::Value::Mapping(v) => {
                for (key, value) in v {
                    println!("key  ------ : {:?}", key);
                    match &key {
                        serde_yaml::Value::String(_s) => {
                            let b: Detail = serde_yaml::from_value(value)?;

                            let r = &b.rule;
                            if r.is_null() {
                                serde_yaml::Error::custom(format!("{:?} rules is null", key));
                                println!("unkown type: {:?}", key);
                            }
                            match b.kind.as_str() {
                                "enum" => match r {
                                    serde_yaml::Value::Sequence(e) => {
                                        println!("enum type: {:?}", e);
                                    }
                                    _ => {
                                        serde_yaml::Error::custom(format!(
                                            "{:?} rules is null",
                                            key
                                        ));
                                    }
                                },
                                "string" => match r {
                                    serde_yaml::Value::Mapping(e) => {
                                        println!("enum string: {:?}", e);
                                    }
                                    _ => {
                                        serde_yaml::Error::custom(format!(
                                            "{:?} rules is null",
                                            key
                                        ));
                                    }
                                },
                                "regexp" => match r {
                                    serde_yaml::Value::Mapping(e) => {
                                        println!("regexp type: {:?}", e);
                                    }
                                    _ => {
                                        serde_yaml::Error::custom(format!(
                                            "{:?} rules is null",
                                            key
                                        ));
                                    }
                                },
                                "list" => match r {
                                    serde_yaml::Value::Mapping(e) => {
                                        println!("list type: {:?}", e);
                                    }
                                    _ => {
                                        serde_yaml::Error::custom(format!(
                                            "{:?} rules is null",
                                            key
                                        ));
                                    }
                                },
                                "mixed" => match r {
                                    serde_yaml::Value::Sequence(e) => {
                                        println!("mixed type: {:?}", e);
                                    }
                                    _ => {
                                        serde_yaml::Error::custom(format!(
                                            "{:?} rules is null",
                                            key
                                        ));
                                    }
                                },
                                _ => {
                                    serde_yaml::Error::custom(format!("unkown type: {:?}", key));
                                    println!("unkown type: {:?}", key);
                                }
                            }
                            res.insert(key, b);
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
