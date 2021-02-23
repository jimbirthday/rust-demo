extern crate serde_yaml;
extern crate yaml_rust;

use std::{hash::Hash, ops::Index, result::Result};

use std::collections::BTreeMap;
use yaml_rust::{YamlEmitter, YamlLoader};
mod model;

pub fn parse_yml(s: &str) -> Result<model::DAG, serde_yaml::Error> {
    let res: model::DAG = serde_yaml::from_str(s)?;
    Ok(res)
}

#[allow(dead_code)]
pub fn parse_yml1(s: &str) -> Result<BTreeMap<String, String>, serde_yaml::Error> {
    let docs = YamlLoader::load_from_str(	"# 结构概览\nversion:\n  type: enum   # string,enum,mixed\n  rule: [ \"1.0\" ]\n  displayName: 配置描述版本号\n  description: 配置描述文件版本号,目前只有 1.0\n  required: true\n\ntriggers:\n  type: enum\n  rule: [ 'commitMessage','push','pr' ]\n  displayName: 触发规则\n  description: 触发规则\n  required: true\n\nname:\n  type: string\n  rule:\n    length: '^\\d{1,10}$'\n  displayName: 流水线唯一ID\n  description: 流水线唯一ID的描述\n  required: true\n\ndisplayName:\n  type: string\n  rule:\n    length: '^\\d{1,10}$'\n  displayName: 流水线名称\n  description: 流水线名称的描述\n  required: true\n\njob:\n  type: regexp\n  rule:\n    length: '^\\d{1,10}$'\n  displayName: 配置描述版本号\n  description: 配置描述文件版本号,目前只有 1.0\n  required: true\n\njobs:\n  type: list\n  rule:\n    mix: 0\n    max: 100\n  displayName: 配置描述版本号\n  description: 配置描述文件版本号,目前只有 1.0\n  required: true\n  message: # 错误提示的文本设定\n    require: \"{$s} 必须 {$s}\"\n    length: 长度必须为 {$lenght}\n\nstages:\n  type: mixed\n  rule:\n    - string:\n        length: '^\\d{1,10}$'\n    - list:\n        mix: 0\n        max: 100\n  description: 流水线阶段的描述\n  required: true");
    assert_eq!(docs.is_ok(), true);
    let docs = docs.unwrap();
    for v in docs.iter() {
        println!("1");
        // println!("doc:------  {:?}", v);
        for av in v.as_hash() {
            for (_k, avv) in av{
                for avvvv in avv.as_hash() {
                    for (k,bv) in avvvv{
                        println!("k : ==== {:?}",k);
                        println!("bv : ==== {:?}",bv);
                    }
                }
            }
        }
        println!("2");
    }
    // let mut out_str = String::new();
    // let mut emitter = YamlEmitter::new(&mut out_str);
    // if let Err(e) = emitter.dump(doc) {
    //     println!("cccc ----- {:?}", e);
    // }
    let res: BTreeMap<String, String> = serde_yaml::from_str(s)?;
    Ok(res)
}

#[test]
fn test_yaml() {
    let s = "version:\n  type: enum   # string,enum,mixed\n  rule: [ \"1.0\" ]\n  displayName: 配置描述版本号\n  description: 配置描述文件版本号,目前只有 1.0\n  required: true";
    match parse_yml(s) {
        Err(e) => {
            println!("err:{:?}", e);
        }
        Ok(r) => {
            println!("dag ok: {:?}", r);
            match &r.version.rule{
                serde_yaml::Value::Sequence(v)=>{
                    for o in v{
                        println!("rules o:{:?}",o)
                    }
                },
                _=>println!("unkown type")
            }
        }
    };
}
