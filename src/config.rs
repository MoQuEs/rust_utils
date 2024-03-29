use config::File;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
struct Config {}

pub fn from_env() -> config::Config {
    config::Config::builder()
        .add_source(config::Environment::default())
        .build()
        .unwrap()
}

pub fn from_file(file_name: &str) -> config::Config {
    config::Config::builder()
        .add_source(File::from(Path::new(file_name)))
        .build()
        .unwrap()
}
