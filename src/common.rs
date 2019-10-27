use json::JsonValue;
use std::fs;
use std;
use simple_error::*;

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

pub fn as_string(thing: &JsonValue) -> Result<String, &'static str> {
    match thing {
        JsonValue::String(s) => Ok(String::from(s)),
        JsonValue::Short(s) => Ok(String::from(s as &str)),
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

pub fn as_range(value: &str) -> Result<std::ops::Range<usize>, Box<dyn std::error::Error>> {
    let parts = value.split('-').collect::<Vec<_>>();

    let lo = parts[0].parse::<usize>()?;
    let hi = parts[1].parse::<usize>()?;
    if lo > hi {
        bail!("Invalid range");
    }
    Ok(std::ops::Range{
        start: lo,
        end: hi+1
    })
}

#[cfg(test)]
#[test]
fn test_as_range() {
    let r = as_range("0-5").expect("Parse failed");
    assert_eq!(r.start, 0);
    assert_eq!(r.end, 6);

    let r2 = as_range("1-1").expect("Parse failed");
    assert_eq!(r2.start, 1);
    assert_eq!(r2.end, 2);
}

pub fn sha256_digest(path: &std::path::PathBuf) -> Result<std::string::String, std::io::Error> {
    let json = fs::read_to_string(path)?;
    let mut hasher = sha1::Sha1::new();
    hasher.update(json.as_ref());
    Ok(hasher.digest().to_string())
}