use crate::options::OptionsArg;
use std::error::Error;
use std::fs;
use std::env;

#[derive(Debug)]
pub struct ArgData {
    pub filename: String,
    pub search: String,
    pub search_type: OptionsArg,
}

impl ArgData {
    pub fn new(args: &[String]) -> Result<ArgData, &'static str> {
        if args.len() < 3 {
            return Err("insufficient arguments");
        }
        let filename = args[1].clone();
        let search = args[2].clone();
        let mut search_type = OptionsArg::Lowercase;
        if args.len() > 3 {
            search_type = match args[3].as_str() {
                "-u" => OptionsArg::Uppercase,
                "-l" => OptionsArg::Lowercase,
                _ => OptionsArg::Lowercase,
            };
        }
        Ok(ArgData {
            filename,
            search,
            search_type,
        })
    }
}

pub fn file_string(filename: &String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    Ok(contents)
}

pub fn get_args() -> Vec<String> {
	env::args().collect()
}
