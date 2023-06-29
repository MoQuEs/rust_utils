use envmnt::get_any;
use envmnt::load_file;
use std::env::current_dir;
use std::fs::metadata;
use std::io::ErrorKind;
use std::path::Path;

/// Load dotenv file by filename in the following order:
/// - filename
/// - ../filename
/// - ../../filename
/// - ...
///
/// # Arguments
/// * `filename` - The filename to load
///
/// # Example
/// ```
/// use rust_utils::dotenv::load_dotenv_file_with_parents;
///
/// if let Err(e) = load_dotenv_file_with_parents(".env") {
///    println!("Failed to load dotenv file: {}", e);
/// }
/// ```
///
pub fn load_dotenv_file_with_parents(filename: impl AsRef<Path>) -> Result<(), std::io::Error> {
    let mut directory = current_dir()?;

    loop {
        let candidate = directory.join(&filename);
        match metadata(&candidate) {
            Ok(candidate_metadata) if candidate_metadata.is_file() => {
                if let Some(candidate_filepath) = candidate.to_str() {
                    load_file(candidate_filepath).map_err(|e| {
                        std::io::Error::new(
                            ErrorKind::Other,
                            format!("Failed to load dotenv file: {}", e),
                        )
                    })?;
                }
            }
            _ => {}
        }

        if let Some(parent) = directory.parent() {
            directory = parent.to_path_buf();
        } else {
            break;
        }
    }

    Ok(())
}

/// Load dotenv files in the following order:
/// - .env + ../.env + ../../.env + ...
/// - .env.{env} + ../.env.{env} + ../../.env.{env} + ...
/// - .env.{dotenv_index} + ../.env.{dotenv_index} + ../../.env.{dotenv_index} + ...
/// - .env.local + ../.env.local + ../../.env.local + ...
/// - .env.{env}.local + ../.env.{env}.local + ../../.env.{env}.local + ...
/// - .env.{dotenv_index}.local + ../.env.{dotenv_index}.local + ../../.env.{dotenv_index}.local + ...
///
/// # Arguments
/// * `dotenv_indexes` - Additional dotenv indexes to load
///
/// # Examples
/// ```
/// use rust_utils::dotenv::load_all_dotenv_files;
///
/// load_all_dotenv_files(&["test"]);
/// ```
///
pub fn load_all_dotenv_files(dotenv_indexes: &[impl AsRef<str>]) {
    let env = get_any(&vec!["env", "ENV", "environment", "ENVIRONMENT"], "");

    let _ = load_dotenv_file_with_parents(".env");
    if !env.is_empty() {
        let _ = load_dotenv_file_with_parents(format!(".env.{env}"));
    }
    for additional_dotenv_file in dotenv_indexes {
        let _ = load_dotenv_file_with_parents(format!(".env.{}", additional_dotenv_file.as_ref()));
    }

    let _ = load_dotenv_file_with_parents(".env.local");
    if !env.is_empty() {
        let _ = load_dotenv_file_with_parents(format!(".env.{env}.local"));
    }
    for additional_dotenv_file in dotenv_indexes {
        let _ = load_dotenv_file_with_parents(format!(
            ".env.{}.local",
            additional_dotenv_file.as_ref()
        ));
    }
}
