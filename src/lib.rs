use json::JsonValue;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn media_trope_str(media_name: &str, media_type: &str) -> Result<String, std::io::Error> {
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
    let fp = format!("{}_{}.json", media_type, media_name);
    let mut file = File::open(fp)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(String::from(contents))
}

fn media_trope_str_to_json(media_trope_str: &str) -> Result<JsonValue, json::Error> {
    let media_trope_json = json::parse(&media_trope_str)?;
    Ok(media_trope_json)
}

fn media_trope_json_to_vec(media_trope_json: JsonValue) -> HashSet<String> {
    let mut keys = HashSet::<String>::new();
    for (key, _) in media_trope_json.entries() {
        keys.insert(String::from(key));
        // values.push(value.as_u32().unwrap());
    }
    keys
}

pub fn get_tropes(media_name: &str, media_type: &str) -> HashSet<String> {
    let tropes_str = match media_trope_str(&media_name, &media_type) {
        Ok(result) => result,
        Err(_) => panic!(format!("Could not find file {}_{}.json!", media_type, media_name)),
    };
    let tropes_json = match media_trope_str_to_json(&tropes_str) {
        Ok(result) => result,
        Err(_) => panic!(format!("File {}_{}.json is not valid JSON.", media_type, media_name)),
    };
    let tropes = media_trope_json_to_vec(tropes_json);
    tropes
}

fn n_overlapping(m1: &HashSet<String>, m2: &HashSet<String>) -> u32 {
    let mut n: u32 = 0;
    for trope in m1 {
        if m2.contains(trope) {
            n += 1;
        }
    }
    n
}

pub fn similarity_score(m1_name: &str, m1_type: &str, m2_name: &str, m2_type: &str) -> f64 {
    // cf. formula at https://en.wikipedia.org/wiki/Cosine_similarity#Definition
    // This is radically simplified by the fact that trope scores are always binary {0, 1} values.
    let m1_tropes = &get_tropes(m1_name, m1_type);
    let m2_tropes = &get_tropes(m2_name, m2_type);

    let numerator = n_overlapping(m1_tropes, m2_tropes);
    let m1_magnitude = (m1_tropes.len() as f64).sqrt();
    let m2_magnitude = (m2_tropes.len() as f64).sqrt();
    (numerator as f64) / (m1_magnitude * m2_magnitude)
}