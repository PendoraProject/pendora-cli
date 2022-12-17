use std::fs::read_to_string;

use clap::{clap_derive::ArgEnum, Parser, Subcommand};
use eyre::Context;
use transpile::build_for;

use crate::project_creation::{create_method, create_object, new_project};
use pendora_base::parser::parse_project;
use pendorafile::Pendorafile;

mod pendorafile;
mod project_creation;
mod transpile;

fn main() -> eyre::Result<()> {
    let args = CLI::parse();
    let commmand = args.clone().command;
    match commmand {
        Commands::New { name } => {
            new_project(name);
        }
        Commands::Create { file_type, name } => match file_type {
            FileType::Object => {
                create_object(name);
            }
            FileType::Method => {
                create_method(name);
            }
        },
        Commands::Build { lang } => {
            let project =
                parse_project("src").wrap_err_with(|| format!("Failed to parsing source files"))?;
            let pendorafile: Pendorafile = toml::from_str(read_to_string("Pendorafile")?.as_str())
                .wrap_err_with(|| format!("Failed to parse Pendorafile"))?;

            match lang {
                Some(l) => {
                    build_for(l.as_str(), project.clone(), pendorafile.clone())
                        .wrap_err("Unable to build for language")?;
                }
                None => {
                    for i in &pendorafile.build.langs.clone() {
                        build_for(i, project.clone(), pendorafile.clone())
                            .wrap_err("Unable to build for language")?;
                    }
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug, Parser, Clone)]
#[clap(name = "pendora")]
#[clap(
    about = "The official command line tooling for the Pendora programming lanuage",
    version = "0.1.0",
    author = "ScratchCat458"
)]
pub struct CLI {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    #[clap(about = "Generate a new project based on the default layout")]
    New { name: Option<String> },
    #[clap(about = "Create a new Object or Method")]
    Create {
        #[clap(arg_enum)]
        file_type: FileType,
        name: Option<String>,
    },
    #[clap(about = "Build language artifacts based on your Pendora templates")]
    Build {
        #[clap(short = 'L', long = "lang")]
        lang: Option<String>,
    },
}

#[derive(Debug, Clone, ArgEnum)]
enum FileType {
    Object,
    Method,
}
