use std::{
    io::{self, Read, Write},
    path::Path,
};

use serde::{Serialize, de::DeserializeOwned};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub fn save<P, T>(path: P, data: T) -> Result<(), io::Error>
where
    P: AsRef<Path>,
    T: Serialize,
{
    let json_data = serde_json::to_string(&data)?;
    let mut file = std::fs::File::create(path)?;

    file.write_all(json_data.as_bytes())
}

pub async fn save_async<P, T>(path: P, data: T) -> Result<(), io::Error>
where
    P: AsRef<Path>,
    T: Serialize + Send + 'static,
{
    let json_data = tokio::task::spawn_blocking(move || serde_json::to_string(&data)).await??;
    let mut file = tokio::fs::File::create(path).await?;

    file.write_all(json_data.as_bytes()).await
}

pub fn load<'de, P, T>(path: P) -> Result<T, io::Error>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let mut file = std::fs::File::open(path)?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;

    let data: T = serde_json::from_str(&json_data)?;

    Ok(data)
}

pub async fn load_async<'de, P, T>(path: P) -> Result<T, io::Error>
where
    P: AsRef<Path>,
    T: DeserializeOwned + Send + 'static,
{
    let mut file = tokio::fs::File::open(path).await?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).await?;

    let data: T = tokio::task::spawn_blocking(move || serde_json::from_str(&json_data)).await??;

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[derive(Serialize, serde::Deserialize, Debug, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }

    #[test]
    fn save_test() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };

        let result = save(path, &data);
        assert!(result.is_ok());

        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("test"));
        assert!(content.contains("42"));
    }

    #[tokio::test]
    async fn save_async_test() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        let data = TestData {
            name: "async_test".to_string(),
            value: 100,
        };

        let result = save_async(path.clone(), data).await;
        assert!(result.is_ok());

        let content = tokio::fs::read_to_string(&path).await.unwrap();
        assert!(content.contains("async_test"));
        assert!(content.contains("100"));
    }

    #[test]
    fn load_test() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let original_data = TestData {
            name: "load_test".to_string(),
            value: 77,
        };

        save(path, &original_data).unwrap();

        let loaded_data: TestData = load(path).unwrap();
        assert_eq!(loaded_data, original_data);
    }

    #[tokio::test]
    async fn load_async_test() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        let original_data = TestData {
            name: "async_load_test".to_string(),
            value: 55,
        };

        save_async(path.clone(), original_data).await.unwrap();

        let loaded_data: TestData = load_async(&path).await.unwrap();
        assert_eq!(loaded_data.name, "async_load_test");
        assert_eq!(loaded_data.value, 55);
    }
}
