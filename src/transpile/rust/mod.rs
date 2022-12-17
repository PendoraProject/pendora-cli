use std::fs::{create_dir_all, write};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use pendora_base::types::Project;

use crate::pendorafile::Pendorafile;

use self::{
    constants::{
        ARGUMENT, ASSIGNMENT_FIELD, CARGO_TOML, GLOBAL, GLOBAL_METHOD, LIB, OBJECT, OBJECT_FIELD,
        OBJECT_METHOD, REQUEST_FIELD,
    },
    conversions::{
        rust_request_content_conversion, rust_request_type_conversion, rust_surround_quotes,
        rust_type_conversion,
    },
};

pub mod constants;
pub mod conversions;

pub fn rust_build(project: Project, pendorafile: Pendorafile) {
    let mpb = MultiProgress::new();

    let pb = ProgressBar::new(4).with_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap(),
    );

    mpb.add(pb.clone());

    let mut output = LIB.to_string();

    pb.set_message("Building the Global Object");

    output = output.replace("|global|", rust_build_global(project.clone()).as_str());

    let pb2 = ProgressBar::new(project.objects.len() as u64).with_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap(),
    );

    pb.inc(1);
    mpb.add(pb2.clone());
    pb.set_message("Building all of the other objects and their methods");

    for (k, _) in &project.objects {
        pb2.set_message(format!("Building {}", &k));
        output = output.replace(
            "|objects|",
            rust_build_object(project.clone(), k.to_string()).as_str(),
        );
        pb2.inc(1);
    }

    output = output.replace("|objects|", "");

    pb.inc(1);
    mpb.remove(&pb2);
    pb.set_message("Writing lib.rs");

    create_dir_all("bin/rust/src").expect("Error creating directories");
    write("bin/rust/src/lib.rs", output).expect("Failed to write lib.rs");

    pb.inc(1);
    pb.set_message("Writing cargo.toml");

    write("bin/rust/Cargo.toml", rust_build_cargo_toml(pendorafile))
        .expect("Failed to write cargo.toml");

    pb.inc(1);
    pb.set_message("Done :O");
}

fn rust_build_cargo_toml(pendorafile: Pendorafile) -> String {
    let mut output = CARGO_TOML.to_string();

    output = output.replace("|#pf_name|", pendorafile.project.name.as_str());
    output = output.replace("|#pf_version|", pendorafile.project.version.as_str());

    output
}

fn rust_build_global(project: Project) -> String {
    let mut output = GLOBAL.to_string();

    output = output.replace("|name|", &project.global.name.as_str());

    for (k, v) in &project.global.shape {
        output = output.replacen(
            "|field|",
            OBJECT_FIELD
                .replace("|field_name|", &k)
                .replace("|field_type|", rust_type_conversion(v.to_owned()).as_str())
                .as_str(),
            1,
        );
    }

    output = output.replacen("|field|", "", 1);

    for (k, v) in &project.global.shape {
        output = output.replace(
            "|argument|",
            ARGUMENT
                .replace("|argument_name|", &k)
                .replace(
                    "|argument_type|",
                    rust_type_conversion(v.to_owned()).as_str(),
                )
                .as_str(),
        )
    }

    output = output.replace("|argument|", "");

    for (k, _) in &project.global.shape {
        output = output.replacen(
            "|field|",
            ASSIGNMENT_FIELD.replace("|arg_name|", &k).as_str(),
            1,
        );
    }

    output = output.replace("|field|", "");

    for i in &project.global.methods {
        output = output.replace(
            "|method|",
            rust_build_global_method(project.clone(), i.to_owned()).as_str(),
        );
    }

    output = output.replace("|method|", "");

    output
}

fn rust_build_global_method(project: Project, method_name: String) -> String {
    let mut output = GLOBAL_METHOD.to_string();
    let method = project.methods.get(method_name.as_str()).unwrap();

    output = output.replace("|method_name|", &method.name.as_str());
    output = output.replace("|return_type|", &method.return_object.as_str());
    output = output.replace(
        "|request_type|",
        rust_request_type_conversion((&method.request_type).to_owned()).as_str(),
    );
    output = output.replace(
        "|request_route|",
        rust_surround_quotes(format!("{}{}", &project.global.head_route, &method.route)).as_str(),
    );

    for (k, v) in &method.arguments {
        output = output.replace("|argument|", ARGUMENT);
        output = output.replace("|argument_name|", k.as_str());
        output = output.replace(
            "|argument_type|",
            rust_type_conversion(v.to_owned()).as_str(),
        );
    }

    output = output.replace("|argument|", "");

    for (k, v) in &method.request_shape {
        output = output.replace("|request_field|", REQUEST_FIELD);
        output = output.replace("|json_field_name|", k.as_str());
        output = output.replace(
            "|json_field_content|",
            rust_request_content_conversion(v.to_owned(), true).as_str(),
        );
    }

    output = output.replace("|request_field|", "");

    output
}

fn rust_build_object(project: Project, object_name: String) -> String {
    let mut output = OBJECT.to_string();
    let object = project.objects.get(object_name.as_str()).unwrap();

    output = output.replace("|name|", &object.name.as_str());

    // TODO single replace fields
    for (k, v) in &object.shape {
        output = output.replace(
            "|field|",
            OBJECT_FIELD
                .replace("|field_name|", &k)
                .replace("|field_type|", rust_type_conversion(v.to_owned()).as_str())
                .as_str(),
        );
    }

    output = output.replace("|field|", "");

    for i in &object.methods {
        output = output.replace(
            "|method|",
            rust_build_object_method(project.clone(), i.to_owned()).as_str(),
        );
    }

    output = output.replace("|method|", "");
    output
}

fn rust_build_object_method(project: Project, method_name: String) -> String {
    let mut output = OBJECT_METHOD.to_string();
    let method = project.methods.get(method_name.as_str()).unwrap();

    output = output.replace("|global_name|", &project.global.name.as_str());
    output = output.replace("|method_name|", &method.name.as_str());
    output = output.replace("|return_type|", &method.return_object.as_str());
    output = output.replace(
        "|request_type|",
        rust_request_type_conversion((&method.request_type).to_owned()).as_str(),
    );
    output = output.replace(
        "|request_route|",
        rust_surround_quotes(format!("{}{}", project.global.head_route, method.route)).as_str(),
    );

    for (k, v) in &method.arguments {
        output = output.replace("|argument|", ARGUMENT);
        output = output.replace("|argument_name|", k.as_str());
        output = output.replace(
            "|argument_type|",
            rust_type_conversion(v.to_owned()).as_str(),
        );
    }

    output = output.replace("|argument|", "");

    for (k, v) in &method.request_shape {
        output = output.replace("|request_field|", REQUEST_FIELD);
        output = output.replace("|json_field_name|", k.as_str());
        output = output.replace(
            "|json_field_content|",
            rust_request_content_conversion(v.to_owned(), false).as_str(),
        );
    }

    output = output.replace("|request_field|", "");

    output
}
