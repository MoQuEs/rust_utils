use envmnt::load_file;
use std::env::{current_dir, var};
use std::fs::metadata;
use std::io::ErrorKind;
use std::path::Path;

pub fn load_dotenv_file(filename: &str) -> Result<(), std::io::Error> {
    let mut directory = current_dir()?;
    let filepath = Path::new(filename);

    let mut load_one = false;
    let mut go_down = true;
    while go_down {
        let candidate = directory.join(filepath);
        match metadata(&candidate) {
            Ok(candidate_metadata) if candidate_metadata.is_file() => {
                if let Some(candidate_filepath) = candidate.to_str() {
                    if load_file(candidate_filepath).is_ok() {
                        println!("Load .env file: {}", &candidate_filepath);
                        load_one = true;
                    }
                }
            }
            _ => {}
        }

        if let Some(parent) = directory.parent() {
            directory = parent.to_path_buf();
        } else {
            go_down = false;
        }
    }

    if !load_one {
        println!("Can't load .env file: {}", &filename);
        return Err(std::io::Error::new(ErrorKind::NotFound, format!("Can't load file: {}", &filename)));
    }

    Ok(())
}

pub fn get_var(env_var: impl AsRef<str>) -> Result<String, ()> {
    let mut variable = String::with_capacity(50);

    let mut load_one = false;
    for custom_env_var in [env_var.as_ref().to_uppercase(), env_var.as_ref().to_lowercase()].iter() {
        if let Ok(env_variable) = var(custom_env_var) {
            variable = env_variable;
            load_one = true;
            break;
        }
    }

    if !load_one {
        println!("Can't get environment value: {}", env_var.as_ref());
        return Err(());
    }

    Ok(variable)
}

pub fn configure_dotenv() {
    let env = get_var("env").unwrap();
    let docker = get_var("docker").unwrap_or_else(|_| String::from("0")) == "1";

    load_dotenv_file(".env").unwrap();
    let _ = load_dotenv_file(if docker { ".env.docker.local" } else { ".env.local" });
    let _ = load_dotenv_file(&format!(".env.{}", env));
    let _ = load_dotenv_file(&if docker {
        format!(".env.{}.docker.local", env)
    } else {
        format!(".env.{}.local", env)
    });

    println!("Env: {}", &env);
    println!("Docker: {}", docker);
}
