mod store;
use crate::store::{load, save};

use serde_json::to_string;
use std::{env, path::Path};

// testing foo
fn main() {
    let data = to_string(&[69, 1200]).unwrap();

    let home_dir = env::home_dir().unwrap();
    let path = Path::join(&home_dir, "Documents");

    match save("test_data.txt", &path, data) {
        Ok(_) => println!("Saved data!"),
        Err(why) => println!("{:?}", why),
    }

    let path = Path::join(&path, "test_data.txt");

    match load(&path) {
        Ok(data) => println!("Data : {}", data),
        Err(why) => println!("{:?}", why),
    }
}
