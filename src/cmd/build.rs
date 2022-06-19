extern crate serde_json;

use std::fs::read_to_string;
use serde::Deserialize;
use crate::utils::files::make_dir;
use std::process::Command;

#[derive(Deserialize)]
struct PFile {
    buildkits: Vec<String>
}

pub fn build(args: Vec<String>){
    let pendorafile = read_to_string("Pendorafile".to_string()).expect("bugged");
    let json: PFile = serde_json::from_str(&pendorafile).expect("JSON was not well-formatted");

    if json.buildkits.len() == 0 {
        panic!("No BuildKits found. Add some in your Pendorafile.");
    }

    println!("BuildKits found:");
    for i in json.buildkits.iter(){
        println!("Loading {} buildkit", i);
        make_dir("build/".to_string() + i);
    }

    println!("Building artifacts:");
    for i in json.buildkits.iter(){
        compile(i.to_string());
    }
}

fn compile(i: String){
    Command::new("./build/".to_string() + &i + "/compile")
        .arg("./Pendorafile")
        .output()
        .expect("Failed to execute");
}