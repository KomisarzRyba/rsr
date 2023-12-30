#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Format {
    Json,
    Yaml,
    Toml,
}

impl Format {
    pub fn from_extension(file: &str) -> Option<Format> {
        match std::path::Path::new(file)
            .extension()
            .and_then(|ext| ext.to_str())
        {
            Some("json") => Some(Format::Json),
            Some("yml") => Some(Format::Yaml),
            Some("toml") => Some(Format::Toml),
            _ => None,
        }
    }
}

impl Format {
    pub fn to_extension(&self) -> &str {
        match self {
            Format::Json => "json",
            Format::Yaml => "yml",
            Format::Toml => "toml",
        }
    }
}

impl ToString for Format {
    fn to_string(&self) -> String {
        match self {
            Format::Json => String::from("Json"),
            Format::Yaml => String::from("Yaml"),
            Format::Toml => String::from("Toml"),
        }
    }
}
