use config::File;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
struct Config {}

pub fn from_env() -> Result<config::Config, String> {
    config::Config::builder()
        .add_source(config::Environment::default())
        .build()
        .map_err(|e| format!("`Error on config building from env {e:?}"))
}

pub fn from_file(file_name: &str) -> Result<config::Config, String> {
    config::Config::builder()
        .add_source(File::from(Path::new(file_name)))
        .build()
        .map_err(|e| format!("`Error on config building from file {e:?}"))
}
