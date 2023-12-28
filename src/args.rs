use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RsrArgs {
    /// Path to the file you want to convert
    pub input_file: String,
    /// Format of the output file
    pub format: crate::formats::Format,
}
