use std::fs;

use dialoguer::{Input, Select};

use crate::pendorafile::{PFBuild, PFProject, Pendorafile};

pub fn new_project(name: Option<String>) {
    let project_name = match name {
        Some(n) => n,
        None => Input::new()
            .with_prompt("Project name")
            .interact_text()
            .unwrap(),
    };

    let global_name: String = Input::new()
        .with_prompt("Global object name")
        .interact_text()
        .unwrap();

    let head_route: String = Input::new()
        .with_prompt("Head route")
        .with_initial_text("https://")
        .interact_text()
        .unwrap();

    let project_dir = format!("./{}/src", &project_name);
    let global_file = format!("{}/{}.pendora", &project_dir, &global_name);
    let data = format!(
        "Global {} {{
    headRoute(\"{}\")

    shape({{

    }})
    
    methods([
        
    ])
}};",
        global_name, head_route
    );

    fs::create_dir_all(&project_dir).expect("Unable to create directory");
    fs::write(global_file, data).expect("Unable to write file");

    let pendorafile = Pendorafile {
        project: PFProject {
            name: project_name.to_string(),
            version: "0.1.0".to_string(),
            author: None,
            src: None,
        },
        build: PFBuild { langs: Vec::new() },
    };
    let pf_dir = format!("./{}/Pendorafile", &project_name);
    fs::write(pf_dir, toml::to_string_pretty(&pendorafile).unwrap()).expect("Unable to write file");

    println!("Project template for {} generated", &project_name);
    println!("cd {} to get started", &project_name);
}

pub fn create_object(name: Option<String>) {
    let object_name = match name {
        Some(n) => n,
        None => Input::<String>::new()
            .with_prompt("Object name")
            .interact_text()
            .unwrap(),
    };

    let data = format!(
        "Object {} {{
    shape({{
        
    }})

    methods([
        
    ])
}};",
        &object_name
    );

    fs::create_dir_all("./src/objects").expect("Unable to create directory");
    let path = format!("./src/objects/{}.pendora", &object_name);
    fs::write(path, data).expect("Unable to write to file");
}

pub fn create_method(name: Option<String>) {
    let method_name = match name {
        Some(n) => n,
        None => Input::<String>::new()
            .with_prompt("Method name")
            .interact_text()
            .unwrap(),
    };

    let method_route = Input::<String>::new()
        .with_prompt("Method route")
        .interact_text()
        .unwrap();

    let request_types = ["GET", "POST", "UPDATE", "DELETE"];

    let request_type = request_types[Select::new()
        .items(&request_types)
        .with_prompt("Method request type")
        .interact()
        .unwrap()];

    let data = format!(
        "Method {}() {{
    route(\"{}\")
    
    request<{}>({{
        
    }})
    
    return<>({{
        
    }})
}};",
        &method_name, &method_route, &request_type
    );

    fs::create_dir_all("./src/methods").expect("Unable to create directory");
    let path = format!("./src/methods/{}.pendora", &method_name);
    fs::write(path, data).expect("Unable to write to file");
}
