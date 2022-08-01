pub enum Options<'a> {
    Uppercase(&'a String),
    Lowercase(&'a String),
}

impl Options<'_> {
    pub fn unwrap(&self) -> &String {
        use Options::*;
        match self {
            Uppercase(search) | Lowercase(search) => search,
        }
    }
}

#[derive(Debug)]
pub enum OptionsArg {
    Uppercase,
    Lowercase,
}

impl OptionsArg {
    pub fn translate<'a>(&self, search: &'a String) -> Options<'a> {
		return match self {
	        OptionsArg::Uppercase => Options::Uppercase(search),
	        OptionsArg::Lowercase => Options::Lowercase(search),
	    };
	}
}
