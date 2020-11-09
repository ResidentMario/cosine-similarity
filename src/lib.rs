use json::JsonValue;
use std::fs::File;
use std::io::prelude::*;

// fn main() -> std::io::Result<()> {
//     let mut file = File::open("foo.txt")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     assert_eq!(contents, "Hello, world!");
//     Ok(())
// }

pub fn get_trope_file(media_name: String) -> Result<JsonValue, std::io::Error> {
    // Result::Ok(json::parse("{}").unwrap())
    let mut file = File::open("foo.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let out = json::parse(&contents);
    out
}

pub fn get_tropes(media_name: String) -> String {
    String::from("Hello World!")
}