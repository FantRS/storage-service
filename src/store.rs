use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

/// Saves serialized string data
pub fn save(filename: &str, path: &Path, data: String) -> Result<(), io::Error> {
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Path is not exists or {:?} is not file.", path),
        ));
    }

    let filepath = Path::join(path, filename.trim_start_matches('/'));
    let mut file = File::create(&filepath)?;
    _ = file.write_all(data.as_bytes());

    Ok(())
}

/// Loads serialized string data
pub fn load(path: &Path) -> Result<String, io::Error> {
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Path is not exists or {:?} is not file.", path),
        ));
    }

    let mut file = File::open(&path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;

    Ok(result)
}
