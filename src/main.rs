mod args;
mod formats;
mod reserializers;

use args::RsrArgs;
use clap::Parser;
use std::fs;

fn main() {
    let cli = RsrArgs::parse();

    let content = fs::read_to_string(&cli.input_file).expect("Failed to read file contents");
    let from_format = formats::Format::from_extension(&cli.input_file)
        .expect("Extension not supported or could not read the extension");

    let result = reserializers::reserialize(&content, from_format, cli.format);
    match result {
        Ok(r) => println!("{}", r),
        Err(e) => eprintln!("{}", e),
    }
}