use self::rust::rust_build;
use crate::pendorafile::Pendorafile;
use eyre::eyre;
use pendora_base::types::Project;

pub mod rust;

pub fn build_for(lang: &str, project: Project, pendorafile: Pendorafile) -> eyre::Result<()> {
    match lang {
        "rust" => rust_build(project, pendorafile),
        _ => return Err(eyre!("Support does not exist for language: {}", lang)),
    }

    Ok(())
}
