use crate::options::{Options, OptionsArg};
use std::env;
use std::process;

mod args;
mod options;
mod search;

fn main() {
    let args: Vec<String> = env::args().collect();

    let data = args::ArgData::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let contents = args::file_string(&data.filename).unwrap_or_else(|err| {
        println!("Problem opening file: {}", err);
        process::exit(1);
    });
    let search_type = match data.search_type {
        OptionsArg::Uppercase => Options::Uppercase(&data.search),
        OptionsArg::Lowercase => Options::Lowercase(&data.search),
    };

    search::find(search_type, &contents);
}
