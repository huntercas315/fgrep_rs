#[derive(Debug)]
pub struct ArgData {
    pub filename: String,
    pub search: String,
    pub search_type: Type,
}

impl ArgData {
    pub fn new(args: &[String]) -> Result<ArgData, &'static str> {
        if args.len() < 3 {
            // panic!("insufficient arguments"); // TODO Refactor errors
            return Err("insufficient arguments");
        }
        let filename = args[1].clone();
        let search = args[2].clone();
        let mut search_type = Type::Lowercase;
        if args.len() > 3 {
            search_type = match args[3].as_str() {
                "-u" => Type::Uppercase,
                "-l" => Type::Lowercase,
                _ => Type::Lowercase,
            };
        }
        Ok(ArgData {
            filename,
            search,
            search_type,
        })
    }
}

#[derive(Debug)]
pub enum Type {
    Uppercase,
    Lowercase,
}
