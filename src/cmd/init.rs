use crate::utils::files::{write_file, make_dir};
use std::io::stdin;


pub fn create_project_structure(args: Vec<String>){
    println!("Welcome to the Pendora CLI!");
    let project_name = &args[2];
    
    let mut global_object_name = String::new();
    println!("What should the name of the global object be? ");
    stdin().read_line(&mut global_object_name).unwrap();
    
    let mut head_route = String::new();
    println!("Neat name! What is the head route to send requests to (don't forget the protocol e.g. https://) ");
    stdin().read_line(&mut head_route).unwrap();
    
    // create directories
    println!("Creating directories");
    make_dir(project_name.to_string() + "/src/methods/init");
    make_dir(project_name.to_string() + "/src/methods/mut");
    make_dir(project_name.to_string() + "/src/objects");
    
    // create src files
    println!("Creating global object");
    create_global_object(project_name.to_string(), global_object_name.to_string(), head_route.to_string());
    
    // create config
    println!("Creating Pendorafile config");
    create_config(project_name.to_string(), global_object_name.to_string());
}

fn create_config(project_name: String, global_object_name: String){
    let project_config: String = "{
    \"name\": \"_N\",
    \"version\": 1.0,
    \"global\": \"_G\",
    \"buildkits\": [
        
    ]
}".to_string();

    write_file(project_name.to_string() + "/Pendorafile", project_config.replace("_N", &project_name).replace("_G", &global_object_name.trim_end()));
}

fn create_global_object(project_name: String, global_object_name: String, head_route: String){
    let global_object = "Global _name {
    headRoute(\"_head\")
    values({

    })
    
    initialisers({

    })
    
    mutations({

    })
}".to_string();

    let path = project_name.to_string() + "/src/" + &global_object_name.trim_end() + ".pendora";
    let content = global_object.replace("_name", &global_object_name.trim_end()).replace("_head", &head_route.trim_end());
    write_file(path, content);
}