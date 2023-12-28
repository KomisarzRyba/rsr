use std::error::Error;

use crate::formats;

pub trait Reserializer {
    fn reserialize(&self, content: &str) -> Result<String, Box<dyn Error>>;
}

struct JsonYamlReserializer;

impl Reserializer for JsonYamlReserializer {
    fn reserialize(&self, content: &str) -> Result<String, Box<dyn Error>> {
        let deserialized: serde_json::Value = serde_json::from_str(content)?;
        serde_yaml::to_string(&deserialized).map_err(Into::into)
    }
}

struct YamlJsonReserializer;

impl Reserializer for YamlJsonReserializer {
    fn reserialize(&self, content: &str) -> Result<String, Box<dyn Error>> {
        let deserialized: serde_yaml::Value = serde_yaml::from_str(content)?;
        serde_json::to_string(&deserialized).map_err(Into::into)
    }
}

pub fn reserialize(
    content: &str,
    from_format: formats::Format,
    to_format: formats::Format,
) -> Result<String, Box<dyn Error>> {
    let reserializer: Box<dyn Reserializer> = match (from_format, to_format) {
        (formats::Format::Json, formats::Format::Yaml) => Box::new(JsonYamlReserializer),
        (formats::Format::Yaml, formats::Format::Json) => Box::new(YamlJsonReserializer),
        (formats::Format::Json, formats::Format::Json) => {
            panic!("file already matches the format provided")
        }
        (formats::Format::Yaml, formats::Format::Yaml) => {
            panic!("file already matches the format provided")
        }
    };
    reserializer.reserialize(content)
}
