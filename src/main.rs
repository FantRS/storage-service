use std::env::home_dir;
use serde::{Deserialize, Serialize};

mod store;

// test
fn main() {

    let data = Person {
        name: String::from("Sanya"),
        age: 19,
        is_male: true
    };

    let home_dir = home_dir();
    if let Some(path) = home_dir {
        let path_str = path.to_str().unwrap();
        let doc_path = store::combine_path(path_str, "Documents");
        store::save(doc_path.as_str(), "test.txt", data);

        let some: Person = store::load(&(doc_path + "/test.txt")).unwrap();
        println!("{:?}", some);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: i8,
    is_male: bool
}