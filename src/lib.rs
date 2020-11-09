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

pub fn media_trope_str(media_name: String, media_type: String) -> Result<String, std::io::Error> {
    // Result::Ok(json::parse("{}").unwrap())
    // Q: why not use ? operator here?
    //
    // A: the ? operator forwards the error type returned by the operand. If this is not the
    // same concrete type, then it will (somehow?) cast the error to the concrete type in the
    // function header, e.g. std::io:Error in this case. This is "one of the big points of ?".
    //
    // However, this doesn't work in this specific case. From rustlang community:
    // 
    // > but your error types have to implement From for it to happen via ?
    // > there's no implementation of From<json::Error> for io::Error
    //
    // I don't understand that condition, exactly, but I do understand its upside: no ? for me,
    // except if I split the method into two, which is what I've done.
    let mut fp = String::from(&media_type);
    fp.push_str("_");
    fp.push_str(&media_name);
    fp.push_str(".json");
    let mut file = File::open(fp)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(String::from(contents))
}

pub fn media_trope_str_to_json(media_trope_str: String) -> Result<JsonValue, json::Error> {
    let media_trope_json = json::parse(&media_trope_str)?;
    Ok(media_trope_json)
}

pub fn media_trope_json_to_vec(media_trope_json: JsonValue) -> Vec<String> {
    for (key, value) in media_trope_json.entries() {
        println!("{:?}, {:?}", key, value)
    }
    vec![]
}
pub fn get_tropes(media_name: String, media_type: String) -> Vec<String> {
    let tropes = match media_trope_str(media_name, media_type) {
        Ok(result) => result,
        Err(_) => panic!("Could not find file media file!"),
    };
    let tropes = match media_trope_str_to_json(tropes) {
        Ok(result) => result,
        Err(_) => panic!("Could not find file media file!"),
    };
    media_trope_json_to_vec(tropes)
}