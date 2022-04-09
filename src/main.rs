use std::env;
use std::fs;
use clap::Parser;

mod args;
mod search;

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = args::ArgData::new(&args);
    let contents =
        fs::read_to_string(&data.filename).expect("Something went wrong reading the file");
    // TODO: Refactor error handling

    search::find(search::Options::Lowercase(&data.search), &contents);
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	// #[clap(short, long)]
	filename: String,
	#[clap(short)]
	uppercase: String,
	#[clap(short)]
	lowercase: String,
}
