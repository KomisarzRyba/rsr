use paste::paste;
use std::error::Error;

use crate::formats;

pub trait Reserializer {
    fn reserialize(&self, content: &str) -> Result<String, Box<dyn Error>>;
}

macro_rules! impl_reserializer {
    ($from:ident, $to:ident, $serde_from:ident, $serde_to:ident) => {
        paste! {
            pub struct [<$from:camel $to:camel Reserializer>];
            impl Reserializer for [<$from:camel $to:camel Reserializer>] {
                fn reserialize(&self, content: &str) -> Result<String, Box<dyn Error>> {
                    let deserialized: $serde_from::Value = $serde_from::from_str(content)?;
                    $serde_to::to_string(&deserialized).map_err(Into::into)
                }
            }
        }
    };
}

impl_reserializer!(Json, Yaml, serde_json, serde_yaml);
impl_reserializer!(Json, Toml, serde_json, toml);
impl_reserializer!(Yaml, Json, serde_yaml, serde_json);
impl_reserializer!(Yaml, Toml, serde_yaml, toml);
impl_reserializer!(Toml, Json, toml, serde_json);
impl_reserializer!(Toml, Yaml, toml, serde_yaml);

pub fn reserialize(
    content: &str,
    from_format: formats::Format,
    to_format: formats::Format,
) -> Result<String, Box<dyn Error>> {
    if from_format == to_format {
        panic!("file already matches the format provided")
    }
    let reserializer: Box<dyn Reserializer> = match (from_format, to_format) {
        (formats::Format::Json, formats::Format::Yaml) => Box::new(JsonYamlReserializer),
        (formats::Format::Json, formats::Format::Toml) => Box::new(JsonTomlReserializer),
        (formats::Format::Yaml, formats::Format::Json) => Box::new(YamlJsonReserializer),
        (formats::Format::Yaml, formats::Format::Toml) => Box::new(YamlTomlReserializer),
        (formats::Format::Toml, formats::Format::Json) => Box::new(TomlJsonReserializer),
        (formats::Format::Toml, formats::Format::Yaml) => Box::new(TomlYamlReserializer),
        _ => panic!("file already matches the format provided"),
    };
    reserializer.reserialize(content)
}
