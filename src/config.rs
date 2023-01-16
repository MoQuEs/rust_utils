use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
struct Config {}

pub fn from_env<T>() -> T {
    let mut cfg = config::Config::new();
    cfg.merge(config::Environment::new()).unwrap();
    cfg.try_into().unwrap()
}

pub fn from_toml<T>(file_name: &str) -> T {
    let mut cfg = config::Config::new();
    cfg.merge(config::FileSourceFile::new(PathBuf::from(file_name))).unwrap();
    cfg.try_into().unwrap()
}
