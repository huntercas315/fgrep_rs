use std::env;
use std::fs;
use std::process;

mod args;
mod search;

fn main() {
    let args: Vec<String> = env::args().collect();

    let data = args::ArgData::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let contents =
        fs::read_to_string(&data.filename).expect("Something went wrong reading the file");
    // TODO: Refactor error handling
    let search_type = match data.search_type {
        args::Type::Uppercase => search::Options::Uppercase(&data.search),
        args::Type::Lowercase => search::Options::Lowercase(&data.search),
    };

    search::find(search_type, &contents);
}
