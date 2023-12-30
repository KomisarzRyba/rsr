mod args;
mod formats;
mod reserializers;

use args::RsrArgs;
use clap::Parser;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = RsrArgs::parse();

    let content = fs::read_to_string(&cli.input_file).expect("Failed to read file contents");
    let from_format = formats::Format::from_extension(&cli.input_file)
        .expect("Extension not supported or could not read the extension");

    let result = reserializers::reserialize(&content, from_format, cli.format)?;
    let path = match cli.output {
        Some(ref o) => Path::new(o).to_owned(),
        None => {
            let input_path = Path::new(&cli.input_file);
            let new_path = change_path_extension(input_path, cli.format)
                .expect("invalid output path")
                .as_path()
                .to_owned();
            new_path
        }
    };

    match fs::write(&path, &result) {
        Ok(()) => println!("file saved at {}", path.to_str().unwrap()),
        Err(_) => eprintln!("could not save the file at {}", path.to_str().unwrap()),
    }

    Ok(())
}

fn change_path_extension(original_path: &Path, new_format: formats::Format) -> Option<PathBuf> {
    let mut buf = PathBuf::new();
    buf.push(original_path.parent().unwrap());
    buf.push(original_path.file_stem().unwrap());
    buf.set_extension(new_format.extension());

    Some(buf)
}
