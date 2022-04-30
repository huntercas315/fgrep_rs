use std::process;

mod args;
mod options;
mod search;

fn main() {
    let args: Vec<String> = args::get_args();

    let data = args::ArgData::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let contents = args::file_string(&data.filename).unwrap_or_else(|err| {
        println!("Problem opening file: {}", err);
        process::exit(1);
    });
	let search_type = data.search_type.translate(&data.search);

    search::find(search_type, &contents);
}
