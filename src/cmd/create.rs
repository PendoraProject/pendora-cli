use crate::utils::files::{write_file};
use std::io::stdin;

//TODO replace testing strings

pub fn create(args: Vec<String>){
    //let pendorafile = read_to_string("new/Pendorafile".to_string()).expect("bugged");
    //let json: serde_json::Value = serde_json::from_str(&pendorafile).expect("JSON was not well-formatted");

    let object_type = &args[2];

    if object_type == "object"{
        create_object(args[3].to_string());
    }
    else if object_type == "initmethod" {
        create_init_method(args[3].to_string());
    }
    else if object_type == "mutmethod" {
        create_mut_method(args[3].to_string());
    }
    else {
        panic!("Invalid command");
    }
}

fn create_object(object_name: String) {
    let object = "Object _ {
    shape({
        
    })
    
    initialisers({
        
    })
    
    mutations({
        
    })
}".to_string();

    write_file("src/objects/".to_string() + &object_name + ".pendora", object.replace("_", &object_name));
}

fn create_init_method(init_method_name: String){
    let init_method = "InitMethod _N(){
    route(\"_R\")
    
    request({
        
    })
    
    return<>({
    })
}".to_string();

    let mut route = String::new();
    println!("Route (extended from head route): ");
    stdin().read_line(&mut route).unwrap();

    write_file("src/methods/init/".to_string() + &init_method_name + ".pendora", init_method.replace("_N", &init_method_name).replace("_R", route.trim_end()))
}

fn create_mut_method(mut_method_name: String){
    let mut_method = "MutMethod _N(){
    route(\"_R\")
    
    request({
        
    })
    
    callback()
}".to_string();

    let mut route = String::new();
    println!("Route (extended from head route): ");
    stdin().read_line(&mut route).unwrap();

    write_file("src/methods/mut/".to_string() + &mut_method_name + ".pendora", mut_method.replace("_N", &mut_method_name).replace("_R", route.trim_end()));
}