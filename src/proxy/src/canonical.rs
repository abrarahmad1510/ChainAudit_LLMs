use serde_json::{Value, to_vec};
use std::collections::BTreeMap;
/// Canonicalise a JSON object according to RFC 8785 (JCS).
/// Returns the canonical bytes.
pub fn canonicalise(value: &Value) -> Vec<u8> {
let sorted = sort_value(value);
// Serialise without extra whitespace.
to_vec(&sorted).expect("canonical serialisation failed")
}
fn sort_value(v: &Value) -> Value {
match v {
Value::Object(obj) => {
let sorted: BTreeMap<_, _> = obj.iter()
.map(|(k, v)| (k.clone(), sort_value(v)))
.collect();
Value::Object(sorted.into_iter().collect())
}
Value::Array(arr) => Value::Array(arr.iter().map(sort_value).collect(
)),
_ => v.clone(),
}
}
#[cfg(test)]
mod tests {
use super::*;
use serde_json::json;
#[test]
fn test_canonicalise() {
let obj = json!({
"b": 2,
"a": 1,
"c": {
"z": 3,
"y": 2
}
});
let expected = r#"{"a":1,"b":2,"c":{"y":2,"z":3}}"#;
let canonical = canonicalise(&obj);
assert_eq!(std::str::from_utf8(&canonical).unwrap(), expected);
}
}
