use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_str, to_string};

#[cfg(feature = "file-async")]
pub async fn load_json_async<T: DeserializeOwned>(path: impl AsRef<str>) -> std::io::Result<T> {
    let data = tokio::fs::read_to_string(path.as_ref()).await?;

    Ok(from_str::<T>(&data)?)
}

#[cfg(feature = "file-async")]
pub async fn save_json_async<T: Serialize>(path: impl AsRef<str>, data: T) -> std::io::Result<()> {
    let json = to_string(&data)?;
    tokio::fs::write(path.as_ref(), json).await
}

pub fn load_json<T: DeserializeOwned>(path: impl AsRef<str>) -> std::io::Result<T> {
    let data = std::fs::read_to_string(path.as_ref())?;

    Ok(from_str::<T>(&data)?)
}

pub fn save_json<T: Serialize>(path: impl AsRef<str>, data: T) -> std::io::Result<()> {
    let json = to_string(&data)?;
    std::fs::write(path.as_ref(), json)
}

pub fn load_file(path: impl AsRef<str>) -> std::io::Result<String> {
    std::fs::read_to_string(path.as_ref())
}

pub fn save_file(path: impl AsRef<str>, data: impl AsRef<[u8]>) -> std::io::Result<()> {
    std::fs::write(path.as_ref(), data)
}
