mod args;
mod serializer;

use args::RsrArgs;
use clap::Parser;
use core::panic;
use std::fs::read_to_string;

fn main() {
    let cli = RsrArgs::parse();

    println!("{}", cli.input_file);
    println!("{:#?}", cli.format);

    let input_string = read_to_string(cli.input_file).unwrap();
    let output_string = serializer::json_to_yaml(input_string.as_str());
    match output_string {
        Ok(output) => println!("{}", output),
        Err(e) => panic!("{}", e),
    }
}
