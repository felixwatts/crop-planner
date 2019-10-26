use json::JsonValue;
use std::fs;
use std;

pub fn as_object(thing: &JsonValue) -> Result<&json::object::Object, &'static str> {
    match thing {
        JsonValue::Object(obj) => Ok(obj),
        _ => Err("Expected JSON object")
    }
}

pub fn as_array(thing: &JsonValue) -> Result<&Vec<JsonValue>, &'static str> {
    match thing {
        JsonValue::Array(arr) => Ok(arr),
        _ => Err("Expected JSON array")
    }
}

pub fn as_string(thing: &JsonValue) -> Result<&str, &'static str> {
    match thing {
        JsonValue::String(s) => Ok(s),
        JsonValue::Short(s) => Ok(s),
        _ => Err("Expected JSON string")
    }
}

pub fn as_int(thing: &JsonValue) -> Result<i32, &'static str> {
    match thing {
        JsonValue::Number(n) => Ok((*n).into()),
        _ => Err("Expected JSON string")
    }
}

pub fn as_usize(thing: &JsonValue) -> Result<usize, &'static str> {
    match thing {
        JsonValue::Number(n) => Ok((*n).into()),
        _ => Err("Expected JSON string")
    }
}

pub fn sha256_digest(path: &std::path::PathBuf) -> Result<std::string::String, std::io::Error> {
    let json = fs::read_to_string(path)?;
    let mut hasher = sha1::Sha1::new();
    hasher.update(json.as_ref());
    Ok(hasher.digest().to_string())
}