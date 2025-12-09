use std::{
    io::{self, Read, Write},
    path::Path,
};

use serde::{Serialize, de::DeserializeOwned};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Synchronously saves data to a JSON file.
///
/// This function serializes the provided data to JSON format and writes it to the specified file.
/// If the file already exists, it will be truncated.
///
/// # Arguments
///
/// * `path` - The file path where data will be saved
/// * `data` - The data to serialize and save (must implement `Serialize`)
///
/// # Returns
///
/// * `Ok(())` - If the operation succeeds
/// * `Err(io::Error)` - If file creation or writing fails, or if serialization fails
///
/// # Example
///
/// ```
/// use storage_service::save;
/// use serde::Serialize;
/// use tempfile::NamedTempFile;
///
/// #[derive(Serialize)]
/// struct User {
///     name: String,
///     age: u32,
/// }
///
/// let temp_file = NamedTempFile::new().unwrap();
/// let user = User {
///     name: "Alice".to_string(),
///     age: 30,
/// };
///
/// let result = save(temp_file.path(), &user);
/// assert!(result.is_ok());
/// ```
pub fn save<P, T>(path: P, data: T) -> Result<(), io::Error>
where
    P: AsRef<Path>,
    T: Serialize,
{
    let json_data = serde_json::to_string(&data)?;
    let mut file = std::fs::File::create(path)?;

    file.write_all(json_data.as_bytes())
}

/// Asynchronously saves data to a JSON file.
///
/// This async function serializes the provided data to JSON format and writes it to the specified
/// file using tokio's async file I/O. Serialization is performed on a blocking task to avoid
/// blocking the async runtime.
///
/// # Arguments
///
/// * `path` - The file path where data will be saved
/// * `data` - The data to serialize and save (must implement `Serialize`, `Send`, and `'static`)
///
/// # Returns
///
/// * `Ok(())` - If the operation succeeds
/// * `Err(io::Error)` - If file creation, writing fails, or if serialization fails
///
/// # Example
///
/// ```
/// use storage_service::save_async;
/// use serde::Serialize;
/// use tempfile::NamedTempFile;
///
/// #[derive(Serialize)]
/// struct Config {
///     host: String,
///     port: u16,
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let temp_file = NamedTempFile::new().unwrap();
///     let config = Config {
///         host: "localhost".to_string(),
///         port: 8080,
///     };
///
///     let result = save_async(temp_file.path(), config).await;
///     assert!(result.is_ok());
/// }
/// ```
pub async fn save_async<P, T>(path: P, data: T) -> Result<(), io::Error>
where
    P: AsRef<Path>,
    T: Serialize + Send + 'static,
{
    let json_data = tokio::task::spawn_blocking(move || serde_json::to_string(&data)).await??;
    let mut file = tokio::fs::File::create(path).await?;

    file.write_all(json_data.as_bytes()).await
}

/// Synchronously loads data from a JSON file.
///
/// This function reads a JSON file and deserializes its contents into the specified type.
///
/// # Arguments
///
/// * `path` - The file path to read from
///
/// # Returns
///
/// * `Ok(T)` - The deserialized data if successful
/// * `Err(io::Error)` - If file reading fails or if deserialization fails
///
/// # Example
///
/// ```
/// use storage_service::{save, load};
/// use serde::{Serialize, Deserialize};
/// use tempfile::NamedTempFile;
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let temp_file = NamedTempFile::new().unwrap();
/// let original = Person {
///     name: "Bob".to_string(),
///     age: 25,
/// };
///
/// save(temp_file.path(), &original).unwrap();
/// let loaded: Person = load(temp_file.path()).unwrap();
/// assert_eq!(loaded, original);
/// ```
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

/// Asynchronously loads data from a JSON file.
///
/// This async function reads a JSON file using tokio's async file I/O and deserializes
/// its contents into the specified type. Deserialization is performed on a blocking task
/// to avoid blocking the async runtime.
///
/// # Arguments
///
/// * `path` - The file path to read from
///
/// # Returns
///
/// * `Ok(T)` - The deserialized data if successful
/// * `Err(io::Error)` - If file reading fails or if deserialization fails
///
/// # Example
///
/// ```
/// use storage_service::{save_async, load_async};
/// use serde::{Serialize, Deserialize};
/// use tempfile::NamedTempFile;
///
/// #[derive(Serialize, Deserialize, Clone, Debug)]
/// struct Settings {
///     theme: String,
///     language: String,
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let temp_file = NamedTempFile::new().unwrap();
///     let original = Settings {
///         theme: "dark".to_string(),
///         language: "en".to_string(),
///     };
///
///     save_async(temp_file.path(), original.clone()).await.unwrap();
///     let loaded: Settings = load_async(temp_file.path()).await.unwrap();
///     assert_eq!(loaded.theme, "dark");
/// }
/// ```
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
