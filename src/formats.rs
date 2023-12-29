#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Format {
    Json,
    Yaml,
    Toml,
    Csv,
}

impl Format {
    pub fn from_extension(file: &str) -> Option<Format> {
        std::path::Path::new(file)
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| match ext {
                "json" => Some(Format::Json),
                "yml" => Some(Format::Yaml),
                "toml" => Some(Format::Toml),
                "csv" => Some(Format::Csv),
                &_ => None,
            })
    }
}
