#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Format {
    Json,
    Yaml,
    Toml,
    Csv,
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
            Some("csv") => Some(Format::Csv),
            _ => None,
        }
    }
}

impl ToString for Format {
    fn to_string(&self) -> String {
        match self {
            Format::Json => String::from("Json"),
            Format::Yaml => String::from("Yaml"),
            Format::Toml => String::from("Toml"),
            Format::Csv => String::from("Csv"),
        }
    }
}
