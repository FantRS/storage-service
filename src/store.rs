use serde::{de::DeserializeOwned, Serialize};
use serde_json::{to_string, from_str};
use std::{fs::{self, File}, io::{Read, Write}, path::Path};

pub fn save<T>(path: &str, filename: &str, data: T)
where
    T : Serialize
{
    if !Path::new(&path).exists() {
        _ = fs::create_dir(path);
    }

    let path = combine_path(path, filename);
    let json_data = to_string(&data).unwrap();
    
    let mut file = File::create(&path).unwrap();
    _ = file.write_all(json_data.as_bytes());
}

pub fn load<T>(path: &str) -> Option<T>
where 
    T : DeserializeOwned
{
    let file = File::open(path);

    if let Ok(mut file) = file {
        let mut json_data = String::new();
        _ = file.read_to_string(&mut json_data);

        let res = from_str::<T>(json_data.as_str());

        match res {
            Ok(data) => Some(data),
            Err(_) => None
        };
    }
    
    None
}

pub fn combine_path(path1: &str, path2: &str) -> String {
    format!("{}/{}", path1, path2)
}