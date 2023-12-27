use serde_json::Value;
use serde_yaml::Error as YamlError;

pub fn json_to_yaml(json_string: &str) -> Result<String, YamlError> {
    let deserialized: Value = serde_json::from_str(json_string).unwrap();
    serde_yaml::to_string(&deserialized)
}
