#[derive(Debug)]
pub struct ArgData {
    pub filename: String,
    pub search: String,
}

impl ArgData {
    pub fn new(args: &[String]) -> ArgData {
        if args.len() < 3 {
            panic!("insufficient arguments"); // Refactor errors
        }
        let filename = args[1].clone();
        let search = args[2].clone();

        ArgData { search, filename }
    }
}
