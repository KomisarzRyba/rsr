mod args;
mod formats;
mod reserializers;

use args::RsrArgs;
use clap::Parser;
use std::{error::Error, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = RsrArgs::parse();

    let content = fs::read_to_string(&cli.input_file)?;
    let from_format =
        formats::Format::from_extension(&cli.input_file).ok_or("file not supported")?;

    let result = reserializers::reserialize(&content, from_format, cli.format)?;
    let path = match cli.output {
        Some(ref o) => Path::new(o).to_owned(),
        None => Path::new(&cli.input_file).with_extension(cli.format.extension()),
    };

    match fs::write(&path, &result) {
        Ok(()) => println!("file saved at {}", path.to_str().unwrap()),
        Err(_) => eprintln!("could not save the file at {}", path.to_str().unwrap()),
    }

    Ok(())
}
