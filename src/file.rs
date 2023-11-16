use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_str, to_string};

#[cfg(feature = "file-async")]
pub async fn load_json_async<T: DeserializeOwned>(path: impl AsRef<str>) -> T {
    let data = tokio::fs::read_to_string(path.as_ref())
        .await
        .unwrap_or_else(|_| panic!("Can't read {}", path.as_ref()));

    from_str::<T>(&data).unwrap_or_else(|_| panic!("Can't parse {}", path.as_ref()))
}

#[cfg(feature = "file-async")]
pub async fn save_json_async<T: Serialize>(path: impl AsRef<str>, data: T) {
    let json = to_string(&data).unwrap_or_else(|_| panic!("Can't serialize {}", path.as_ref()));
    tokio::fs::write(path.as_ref(), json)
        .await
        .unwrap_or_else(|_| panic!("Can't write {}", path.as_ref()));
}

fn load_json<T: DeserializeOwned>(path: impl AsRef<str>) -> T {
    let data = std::fs::read_to_string(path.as_ref())
        .unwrap_or_else(|_| panic!("Can't read {}", path.as_ref()));

    from_str::<T>(&data).unwrap_or_else(|_| panic!("Can't parse {}", path.as_ref()))
}

fn save_json<T: Serialize>(path: impl AsRef<str>, data: T) {
    let json = to_string(&data).unwrap_or_else(|_| panic!("Can't serialize {}", path.as_ref()));
    std::fs::write(path.as_ref(), json).unwrap_or_else(|_| panic!("Can't write {}", path.as_ref()));
}
