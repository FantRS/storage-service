use std::env::home_dir;

mod store;

// test
fn main() {
    let home_dir = home_dir();

    if let Some(path) = home_dir {
        let path_str = path.to_str().unwrap();
        let doc_path = store::combine_path(path_str, "Documents");
        store::save(doc_path.as_str(), "test.txt", String::from("sd new"));

        let some: Option<String> = store::load(&(doc_path + "/test.txt"));
        println!("{:?}", some);
    }
}
