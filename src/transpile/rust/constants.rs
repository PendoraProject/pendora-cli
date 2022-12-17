pub const CARGO_TOML: &str = "[package]
name = \"|#pf_name|\"
version = \"|#pf_version|\"
edition = \"2021\"

[dependencies]
ureq = { version = \"*\", features = [\"json\"] }
serde_json = \"1.0\"
serde = { version = \"1.0\", features = [\"derive\"] }
";

pub const LIB: &str = "use ureq::*;
use serde::{Serialize, Deserialize};

|global|

|objects|
";

pub const GLOBAL: &str = "pub struct |name| {
    |field|
}

impl |name| {
    pub fn new(|argument|) -> Self {
        |name| {
            |field|
        }
    }

    |method|
}
";

pub const OBJECT: &str = "#[derive(Serialize, Deserialize)]
pub struct |name| {
    |field|
}

impl |name| {
    |method|
}

|objects|";

pub const OBJECT_FIELD: &str = "pub |field_name|: |field_type|,
|field|";

pub const ASSIGNMENT_FIELD: &str = "|arg_name|,
|field|";

pub const ARGUMENT: &str = "|argument_name|: |argument_type|, |argument|";

pub const GLOBAL_METHOD: &str =
    "pub fn |method_name|(&self, |argument|) -> Result<|return_type|, Error> {
    let resp: ureq::Response = ureq::request(|request_type|, |request_route|)
        .send_json(ureq::json!({
            |request_field|
        }))?;

    let content: |return_type| = resp.into_json()?;
    Ok(content)
}

|method|";

pub const OBJECT_METHOD: &str =
    "pub fn |method_name|(&self, global: |global_name|, |argument|) -> Result<|return_type|, Error> {
    let resp: ureq::Response = ureq::request(|request_type|, |request_route|)
        .send_json(ureq::json!({
            |request_field|
        }))?;

    let content: |return_type| = resp.into_json()?;
    Ok(content)
}

|method|";

pub const REQUEST_FIELD: &str = "\"|json_field_name|\": |json_field_content|,
|request_field|";
