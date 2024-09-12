use json::JsonValue;

pub fn pretty_json(json_value: &JsonValue, indent: usize) -> String {
    let indent_str = " ".repeat(indent);
    let mut result = String::new();
    match json_value {
        JsonValue::Object(obj) => {
            result.push_str("{\n");
            let mut first = true;
            for (key, value) in obj.iter() {
                if !first {
                    result.push_str(",\n");
                }
                first = false;
                result.push_str(&format!("{}\"{}\": {}", indent_str, key, pretty_json(value, indent + 2)));
            }
            result.push_str(("\n".to_owned() + &indent_str[2..] + "}").as_str());
        }
        JsonValue::Array(arr) => {
            result.push_str("[\n");
            let mut first = true;
            for value in arr {
                if !first {
                    result.push_str(",\n");
                }
                first = false;
                result.push_str(&format!("{}{}", indent_str, pretty_json(value, indent + 2)));
            }
            result.push_str(("\n".to_string() + &indent_str[2..] + "]").as_str());
        }
        JsonValue::String(s) => result.push_str(&format!("\"{}\"", s)),
        JsonValue::Number(n) => result.push_str(&n.to_string()),
        JsonValue::Short(s) => result.push_str(&format!("\"{}\"", s)),
        JsonValue::Boolean(b) => result.push_str(&b.to_string()),
        JsonValue::Null => result.push_str("null"),
    }
    result
}