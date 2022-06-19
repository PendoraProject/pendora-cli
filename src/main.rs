use std::env;

mod utils;
mod cmd;
use crate::cmd::init::create_project_structure;
use crate::cmd::create::create;
use crate::cmd::build::build;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "init" {
        create_project_structure(args);
    }
    else if args[1] == "create" {
        create(args);
    }
    else if args[1] == "build" {
        build(args);
    }
    else {
        panic!("Invalid arguments");
    }
}
