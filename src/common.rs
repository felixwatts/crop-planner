use json::JsonValue;

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