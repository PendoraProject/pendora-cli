use std::fs::{File, create_dir_all};
use std::io::prelude::*;
use std::path::Path;

pub fn write_file(path_name: String, content: String){
    let path = Path::new(&path_name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(&content.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => println!("Successfully wrote to {}", display),
    }
}

pub fn make_dir(dir_name: String){
    match create_dir_all(dir_name) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }
}