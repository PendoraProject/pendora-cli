// Pendorafile uses TOML under the hood

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pendorafile {
    pub project: PFProject,
    pub build: PFBuild,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PFProject {
    pub name: String,
    pub author: Option<String>,
    pub version: String,
    // if None, src will be in ./src
    pub src: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PFBuild {
    pub langs: Vec<String>,
}
