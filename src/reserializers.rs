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

struct JsonTomlReserializer;

impl Reserializer for JsonTomlReserializer {
    fn reserialize(&self, content: &str) -> Result<String, Box<dyn Error>> {
        let deserialized: serde_json::Value = serde_json::from_str(content)?;
        toml::to_string(&deserialized).map_err(Into::into)
    }
}

struct YamlJsonReserializer;

impl Reserializer for YamlJsonReserializer {
    fn reserialize(&self, content: &str) -> Result<String, Box<dyn Error>> {
        let deserialized: serde_yaml::Value = serde_yaml::from_str(content)?;
        serde_json::to_string(&deserialized).map_err(Into::into)
    }
}

struct YamlTomlReserializer;

impl Reserializer for YamlTomlReserializer {
    fn reserialize(&self, content: &str) -> Result<String, Box<dyn Error>> {
        let deserialized: serde_yaml::Value = serde_yaml::from_str(content)?;
        toml::to_string(&deserialized).map_err(Into::into)
    }
}

struct TomlJsonReserializer;

impl Reserializer for TomlJsonReserializer {
    fn reserialize(&self, content: &str) -> Result<String, Box<dyn Error>> {
        let deserialized: toml::Value = toml::from_str(content)?;
        serde_json::to_string(&deserialized).map_err(Into::into)
    }
}

struct TomlYamlReserializer;

impl Reserializer for TomlYamlReserializer {
    fn reserialize(&self, content: &str) -> Result<String, Box<dyn Error>> {
        let deserialized: toml::Value = toml::from_str(content)?;
        serde_yaml::to_string(&deserialized).map_err(Into::into)
    }
}

pub fn reserialize(
    content: &str,
    from_format: formats::Format,
    to_format: formats::Format,
) -> Result<String, Box<dyn Error>> {
    let reserializer: Box<dyn Reserializer> = match (from_format, to_format) {
        (formats::Format::Json, formats::Format::Yaml) => Box::new(JsonYamlReserializer),
        (formats::Format::Json, formats::Format::Toml) => Box::new(JsonTomlReserializer),
        (formats::Format::Yaml, formats::Format::Json) => Box::new(YamlJsonReserializer),
        (formats::Format::Yaml, formats::Format::Toml) => Box::new(YamlTomlReserializer),
        (formats::Format::Toml, formats::Format::Json) => Box::new(TomlJsonReserializer),
        (formats::Format::Toml, formats::Format::Yaml) => Box::new(TomlYamlReserializer),
        (formats::Format::Json, formats::Format::Json)
        | (formats::Format::Yaml, formats::Format::Yaml)
        | (formats::Format::Toml, formats::Format::Toml)
        | (formats::Format::Csv, formats::Format::Csv) => {
            panic!("file already matches the format provided")
        }
        (formats::Format::Json, formats::Format::Csv) => todo!(),
        (formats::Format::Yaml, formats::Format::Csv) => todo!(),
        (formats::Format::Toml, formats::Format::Csv) => todo!(),
        (formats::Format::Csv, formats::Format::Json) => todo!(),
        (formats::Format::Csv, formats::Format::Yaml) => todo!(),
        (formats::Format::Csv, formats::Format::Toml) => todo!(),
    };
    reserializer.reserialize(content)
}
