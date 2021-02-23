extern crate serde_yaml;
extern crate yaml_rust;

use std::{collections::HashMap, result::Result};
mod model;

pub fn parse_yml(s: &str) -> Result<model::DAG, serde_yaml::Error> {
    let res: model::DAG = serde_yaml::from_str(s)?;
    Ok(res)
}

#[allow(dead_code)]
pub fn parse_yml_to_value(v: serde_yaml::Value) -> Result<model::Detail, serde_yaml::Error> {
    let res: model::Detail = serde_yaml::from_value(v)?;
    Ok(res)
}

fn value_to_detail(s: &str) -> Result<serde_yaml::Value, serde_yaml::Error> {
    let res: serde_yaml::Value = serde_yaml::from_str(s)?;
    Ok(res)
}

pub fn get_dag(s: &str) {
    let mut res = HashMap::new();
    let a = value_to_detail(s);
    match a {
        Err(e) => {
            println!("parse_yml_to_value err :{:?}", e);
        }
        Ok(r) => {
            println!("parse_yml_to_value ok : {:?}", r);
            match r {
                serde_yaml::Value::Mapping(v) => {
                    for (key, value) in v {
                        match key {
                            serde_yaml::Value::String(s) => {
                                res.insert(s, value);
                            }
                            _ => println!("unkown type: {:?}", key),
                        }
                    }
                }
                _ => println!("unkown type : {:?}", r),
            }
        }
    };
    println!("rules o:{:?}", res);
}

#[test]
fn test_yaml() {
    let s =  "# 结构概览\nversion:\n  type: enum   # string,enum,mixed\n  rule: [ \"1.0\" ]\n  displayName: 配置描述版本号\n  description: 配置描述文件版本号,目前只有 1.0\n  required: true\n\ntriggers:\n  type: enum\n  rule: [ 'commitMessage','push','pr' ]\n  displayName: 触发规则\n  description: 触发规则\n  required: true\n\nname:\n  type: string\n  rule:\n    length: '^\\d{1,10}$'\n  displayName: 流水线唯一ID\n  description: 流水线唯一ID的描述\n  required: true\n\ndisplayName:\n  type: string\n  rule:\n    length: '^\\d{1,10}$'\n  displayName: 流水线名称\n  description: 流水线名称的描述\n  required: true\n\njob:\n  type: regexp\n  rule:\n    length: '^\\d{1,10}$'\n  displayName: 配置描述版本号\n  description: 配置描述文件版本号,目前只有 1.0\n  required: true\n\njobs:\n  type: list\n  rule:\n    mix: 0\n    max: 100\n  displayName: 配置描述版本号\n  description: 配置描述文件版本号,目前只有 1.0\n  required: true\n  message: # 错误提示的文本设定\n    require: \"{$s} 必须 {$s}\"\n    length: 长度必须为 {$lenght}\n\nstages:\n  type: mixed\n  rule:\n    - string:\n        length: '^\\d{1,10}$'\n    - list:\n        mix: 0\n        max: 100\n  description: 流水线阶段的描述\n  required: true";
    match model::DAG::from_str(s) {
        Err(e) => {
            println!("err :{:?}", e);
        }
        Ok(t) => {
            println!("dag :{:?}", t);
        }
    };
}
