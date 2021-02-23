use hello_rust::parse_yml;

//Hello World
// fn main(){
//    println!("Hello World")
// }

//println
// #[derive(Debug)]
// struct Structure(i32);

// #[derive(Debug)]
// struct Deep(Structure);

// fn main() {
//    println!("{} is sha diao", "mao guo rui");

//    println!("{0} {1} {2} {1} ", "mao", "is", "sha");

//    println!("{name} is {some}", name = "mao guo rui ", some = "sha diao");

//    println!(
//       "{} of {:b} people know binary, the other half doesn't",
//       1, 10
//    );

//    println!("{number:>width$}", number = "abc", width = 100);

//    println!("Now {:?} will print!", Deep(Structure(7)));
// }

//Primitives
// fn main() {
//    let _logical: bool = true;

//    let _float: f64 = 1.0;

//    let inferred_type;

//    inferred_type = 123;

//    println!("{:?}", inferred_type);

//    let a = 1;
//    let b = false;

//    let res = reverse((a, b));

//    println!("{:?}", res);

//    let xs: [i32; 5] = [123321, 2, 3, 4, 5];
//    analyze_slice(&xs);

//    let xss: [i32;10] = [5; 10];
//    analyze_slice(&xss);

//    analyze_slice(&xs[2..4]);
// }

// fn reverse(pair: (i32, bool)) -> (bool, i32) {
//    let (interger, boolean) = pair;

//    (boolean, interger)
// }

// fn analyze_slice(slice: &[i32]) {
//    println!("first element of the slice: {}", slice[0]);
//    println!("second element of the slice: {}", slice[1]);
//    println!("the slice has {} elements", slice.len());
// }

//Custom Types
// #[derive(Debug)]
// struct Persion {
//    name: String,
//    age: i32,
// }

// struct Unit;

// struct Pair(i32, f32);

// // A struct with two fields
// #[derive(Debug)]
// struct Point {
//    x: f32,
//    y: f32,
// }

// #[allow(dead_code)]
// struct Rectangle {
//    top_left: Point,
//    buttom: Point,
// }

// fn main() {
//    let name = String::from("jim");
//    let age = 21;
//    let peter = Persion { name, age };
//    println!("{:?}", peter);

//    let point = Point { x: 1.0, y: 3.5 };
//    println!("{:?}", point);

//    let bottom_right = Point { ..point };

//    println!("second point: ({:?}, {:?})", bottom_right.x, bottom_right.y);

//    // Instantiate a unit struct
//    let _unit = Unit;

//    // Instantiate a tuple struct
//    let pair = Pair(1, 0.1);

//    // Access the fields of a tuple struct
//    println!("pair contains {:?} and {:?}", pair.0, pair.1);

//    // Destructure a tuple struct
//    let Pair(integer, f32) = pair;

//    println!("pair contains {:?} and {:?}", integer, f32);
// }

// extern crate yaml_rust;
// use serde::{Deserialize, Serialize};
// use yaml_rust::{YamlEmitter, YamlLoader};

// fn main() {
//    let docs = YamlLoader::load_from_str(	"# 结构概览\nversion:\n  type: enum   # string,enum,mixed\n  rule: [ \"1.0\" ]\n  displayName: 配置描述版本号\n  description: 配置描述文件版本号,目前只有 1.0\n  required: true\n\ntriggers:\n  type: enum\n  rule: [ 'commitMessage','push','pr' ]\n  displayName: 触发规则\n  description: 触发规则\n  required: true\n\nname:\n  type: string\n  rule:\n    length: '^\\d{1,10}$'\n  displayName: 流水线唯一ID\n  description: 流水线唯一ID的描述\n  required: true\n\ndisplayName:\n  type: string\n  rule:\n    length: '^\\d{1,10}$'\n  displayName: 流水线名称\n  description: 流水线名称的描述\n  required: true\n\njob:\n  type: regexp\n  rule:\n    length: '^\\d{1,10}$'\n  displayName: 配置描述版本号\n  description: 配置描述文件版本号,目前只有 1.0\n  required: true\n\njobs:\n  type: list\n  rule:\n    mix: 0\n    max: 100\n  displayName: 配置描述版本号\n  description: 配置描述文件版本号,目前只有 1.0\n  required: true\n  message: # 错误提示的文本设定\n    require: \"{$s} 必须 {$s}\"\n    length: 长度必须为 {$lenght}\n\nstages:\n  type: mixed\n  rule:\n    - string:\n        length: '^\\d{1,10}$'\n    - list:\n        mix: 0\n        max: 100\n  description: 流水线阶段的描述\n  required: true");
//    assert_eq!(docs.is_ok(), true);
//    let docs = docs.unwrap();
//    println!("cccc ----- ");
//    println!("docs {:?}", docs);
//    let doc = &docs[0];

//    let mut out_str = String::new();
//    let mut emitter = YamlEmitter::new(&mut out_str);
//    if let Err(_e) =emitter.dump(doc)  {
//       return;
//    }
//    println!("cccc ----- ");
//    println!("doc {:?}", out_str);
// }
// use std::collections::HashMap;
// #[derive(Debug, PartialEq, Serialize, Deserialize)]
// pub struct Pipeline {
//     version: String,
// }

// fn main() -> Result<(), serde_yaml::Error> {
//    let out_str = 	"version:\n 1.0";
//    let deserialized_point: Pipeline = serde_yaml::from_str(out_str)?;
//    println!("res {:?}", deserialized_point);
//    Ok(())
// }

fn main() {
    // let s = "displayName: 配置描述版本号\ndescription: 配置描述文件版本号,目前只有 1.0\nrequired: true";
    let s = "version:\n  type: enum   # string,enum,mixed\n  rule: [ \"1.0\" ]\n  displayName: 配置描述版本号\n  description: 配置描述文件版本号,目前只有 1.0\n  required: true";
    match parse_yml(s) {
        Err(e) => {
            println!("err:{:?}", e);
        }
        Ok(r) => {
            println!("dag : {:?}", r);
        }
    };
}
