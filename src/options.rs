pub enum Options<'a> {
    Uppercase(&'a String),
    Lowercase(&'a String),
}

#[derive(Debug)]
pub enum OptionsArg {
	Uppercase,
    Lowercase,
}

impl Options<'_> {
    pub fn unwrap(&self) -> &String {
        use Options::*;
        return match self {
            Uppercase(search) => &search,
            Lowercase(search) => &search,
        };
    }
}
