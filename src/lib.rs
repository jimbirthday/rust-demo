extern crate regex;
extern crate serde_yaml;
extern crate yaml_rust;

use regex::Regex;

use std::{collections::HashMap, result::Result};
mod dag;
mod yaml;

pub fn parse_yml(s: &str) -> Result<dag::DAG, serde_yaml::Error> {
    let res: dag::DAG = serde_yaml::from_str(s)?;
    Ok(res)
}

#[allow(dead_code)]
pub fn parse_yml_to_value(v: serde_yaml::Value) -> Result<dag::Detail, serde_yaml::Error> {
    let res: dag::Detail = serde_yaml::from_value(v)?;
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
    let d_c =  "# 结构概览\nversion:\n  type: enum   # string,enum,mixed\n  rule: [ \"1.0\" ]\n  displayName: 配置描述版本号\n  description: 配置描述文件版本号,目前只有 1.0\n  required: true\n\ntriggers:\n  type: enum\n  rule: [ 'commitMessage','push','pr' ]\n  displayName: 触发规则\n  description: 触发规则\n  required: true\n\nname:\n  type: string\n  rule:\n    length: '^\\d{1,10}$'\n  displayName: 流水线唯一ID\n  description: 流水线唯一ID的描述\n  required: true\n\ndisplayName:\n  type: string\n  rule:\n    length: '^\\d{1,10}$'\n  displayName: 流水线名称\n  description: 流水线名称的描述\n  required: true\n\njob:\n  type: regexp\n  rule:\n    length: '^\\d{1,10}$'\n  displayName: 配置描述版本号\n  description: 配置描述文件版本号,目前只有 1.0\n  required: true\n\njobs:\n  type: list\n  rule:\n    mix: 0\n    max: 100\n  displayName: 配置描述版本号\n  description: 配置描述文件版本号,目前只有 1.0\n  required: true\n  message: # 错误提示的文本设定\n    require: \"{$s} 必须 {$s}\"\n    length: 长度必须为 {$lenght}\n\nstages:\n  type: mixed\n  rule:\n    - string:\n        length: '^\\d{1,10}$'\n    - list:\n        mix: 0\n        max: 100\n  description: 流水线阶段的描述\n  required: true";
    let d = match dag::DAG::from_str(d_c) {
        Err(e) => {
            println!("err :{:?}", e);
            return;
        }
        Ok(t) => {
            // println!("dag :{:?}", t);
            t
        }
    };
    println!("");
    println!("");
    println!("");
    let y_c = 	"version: 1.0\nname: go-test-copy\ndisplayName:\ntriggers:\n  push:\n    branches:\n      - master\nvariables:\n  WORLD: world_ymlsd\nstages:\n  - stage: maven@1\n    displayName: 第一个stage\n    name: stage1\n    jobs:\n      - job:\n        displayName: 第一个job1\n        name: job1\n        environments:\n           WORLD: world\n           HELLO: helloaaa\n        commands:\n          - echo ${{HELLO}} &&\n            echo ${{WORLD}}\n          -\n            - echo group1  --- Test1\n            - echo group1  --- Test2\n            - echo group1  --- Test3\n          - group2:\n              - echo group2  --- Test1\n              - echo group2  --- Test2\n              - echo group2  --- Test3\n              - errGroup:\n                  -echo errGroup  --- Test1\n                  -echo errGroup  --- Test2\n          - echo Hello Wolrd 3\n          - sleep 10s\n        artifacts:\n          - name: mv1\n            scope: archive\n            repository: http://maven/last\n            path: /v1.0/java.jar\n          - name: zip1\n            scope: pipieline\n            path: /v.10/a.zip\n          - name: var1\n            scope: variable\n            value: a\n        dependArtifacts:\n          - type: archive\n            repository: http://maven/last\n            name: mv1\n            target: /data\n            isForce: true\n          - type: pipieline\n            sourceStage: stage1\n            sourceJob: job1\n            name: zip1\n            target: /data\n            isForce: true\n          - type: variable\n            value: a\n            isForce: true\n  - stage: maven@1\n    displayName: 第一个stage\n    name: stage1\n    jobs:\n      - job:\n        displayName: 第一个job1\n        name: job1\n        environments:\n           WORLD: world\n           HELLO: helloaaa\n        commands:\n          - echo ${{HELLO}} &&\n            echo ${{WORLD}}\n          -\n            - echo group1  --- Test1\n            - echo group1  --- Test2\n            - echo group1  --- Test3\n          - group2:\n              - echo group2  --- Test1\n              - echo group2  --- Test2\n              - echo group2  --- Test3\n              - errGroup:\n                  -echo errGroup  --- Test1\n                  -echo errGroup  --- Test2\n          - echo Hello Wolrd 3\n          - sleep 10s\n        artifacts:\n          - name: mv1\n            scope: archive\n            repository: http://maven/last\n            path: /v1.0/java.jar\n          - name: zip1\n            scope: pipieline\n            path: /v.10/a.zip\n          - name: var1\n            scope: variable\n            value: a\n        dependArtifacts:\n          - type: archive\n            repository: http://maven/last\n            name: mv1\n            target: /data\n            isForce: true\n          - type: pipieline\n            sourceStage: stage1\n            sourceJob: job1\n            name: zip1\n            target: /data\n            isForce: true\n          - type: variable\n            value: a\n            isForce: true";
    let p = match yaml::Pipeline::from_str(y_c) {
        Err(e) => {
            println!("err :{:?}", e);
            return;
        }
        Ok(t) => {
            // println!("dag :{:?}", t);
            t
        }
    };
    println!("");
    println!("");
    println!("");
    match p {
        serde_yaml::Value::Mapping(e) => {
            for (k, v) in e {
                match d.get(&k) {
                    Some(detail) => match detail.kind.as_str() {
                        "enum" => {
                            let yv: serde_yaml::Value = match serde_yaml::from_value(v) {
                                Err(e) => {
                                    println!("err :{:?}", e);
                                    return;
                                }
                                Ok(t) => t,
                            };

                            let yv = match yv {
                                serde_yaml::Value::String(s) => s,
                                serde_yaml::Value::Number(s) => format!("{:.1}", s),
                                _ => {
                                    println!("{:?} is not string", k);
                                    println!("1111111");
                                    continue;
                                }
                            };

                            let rules: serde_yaml::Sequence =
                                match serde_yaml::from_value(detail.rule.clone()) {
                                    Err(e) => {
                                        println!("err :{:?}", e);
                                        return;
                                    }
                                    Ok(t) => t,
                                };
                            let mut flag = false;
                            for r in rules {
                                let rv = match r {
                                    serde_yaml::Value::String(s) => s,
                                    serde_yaml::Value::Number(s) => format!("{:.1}", s),
                                    _ => {
                                        println!("{:?} is not string", k);
                                        continue;
                                    }
                                };

                                if rv == yv {
                                    flag = true;
                                    break;
                                }
                            }
                            if flag {
                                println!("ver ruc ---: {:?}", k);
                            } else {
                                println!("ver err ---: {:?}", k);
                            }
                        }
                        "string"| "regexp" => {
                            let yv: serde_yaml::Value = match serde_yaml::from_value(v) {
                                Err(e) => {
                                    println!("err :{:?}", e);
                                    return;
                                }
                                Ok(t) => t,
                            };

                            let yv = match yv {
                                serde_yaml::Value::String(s) => s,
                                serde_yaml::Value::Number(s) => format!("{:.1}", s),
                                _ => {
                                    println!("{:?} is not string", k);
                                    return;
                                }
                            };

                            let rules: serde_yaml::Mapping =
                                match serde_yaml::from_value(detail.rule.clone()) {
                                    Err(e) => {
                                        println!("err :{:?}", e);
                                        return;
                                    }
                                    Ok(t) => t,
                                };

                            for (rules_key, rules_value) in rules {
                                let rv: serde_yaml::Value =
                                    match serde_yaml::from_value(rules_value) {
                                        Err(e) => {
                                            println!("err :{:?}", e);
                                            return;
                                        }
                                        Ok(t) => t,
                                    };

                                let reg = match rv {
                                    serde_yaml::Value::String(s) => s,
                                    _ => {
                                        println!("err :{:?} is not regex", k);
                                        return;
                                    }
                                };

                                let r = match Regex::new(&reg) {
                                    Ok(t) => {
                                        println!("ok : {}", t);
                                        t
                                    }
                                    Err(e) => {
                                        println!("err : {}", e);
                                        return;
                                    }
                                };

                                if r.is_match(&yv) {
                                    println!("ok {}", yv);
                                } else {
                                    println!("{:?} is not match {:?}", k, yv);
                                }
                            }
                        }
                        "list" => {
                            println!("1list type ---: {:?}", v);
                        }
                        "mixed" => {
                            println!("mixed type ---: {:?}", v);
                        }
                        _ => {
                            println!("err  --- : {:?}", v);
                        }
                    },
                    _ => println!("unknown type err  ---- : {:?}", k),
                }
            }
            println!("");
        }
        _ => {
            println!("yaml pase failed");
            return;
        }
    }
}

#[test]
fn test1() {
    let tweet = "123456";
    let r = match Regex::new(r"^\d{1,10}$") {
        Ok(t) => {
            println!("ok : {}", t);
            t
        }
        Err(e) => {
            println!("err : {}", e);
            return;
        }
    };

    if r.is_match(tweet) {
        println!("ok {}", tweet);
    } else {
        println!("not match");
    }
}
