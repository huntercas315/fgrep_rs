pub enum Options<'a> {
    Uppercase(&'a String),
    Lowercase(&'a String),
}

impl Options<'_> {
    fn extract(&self) -> &String {
        match self {
            Options::Uppercase(search) => return &search,
            Options::Lowercase(search) => return &search,
        }
    }
}

pub fn find(option: Options, contents: &String) {
    let lines: Vec<(String, i16)> = match option {
        Options::Uppercase(search) => find_uppercase(search, contents),
        Options::Lowercase(search) => find_lowercase(search, contents),
    };

    let search: &String = option.extract();

    if lines.len() != 0 {
        display_results(&lines, search)
    } else {
        println!("{} was not found in the chosen file", search);
    }
}

fn find_uppercase(search: &String, contents: &String) -> Vec<(String, i16)> {
    // panic!("The forbiden func\nhttps://youtu.be/wrj59JLxStM");
    let mut lines: Vec<(String, i16)> = Vec::new();
    let mut line_num: i16 = 0;

    for line in contents.lines() {
        line_num += 1;
        if line.contains(&search.as_str()) {
            lines.push((String::from(line), line_num));
        }
    }

    lines
}

fn find_lowercase(search: &String, contents: &String) -> Vec<(String, i16)> {
    let mut lines: Vec<(String, i16)> = Vec::new();
    let mut line_num: i16 = 0;

    for line in contents.lines() {
        line_num += 1;
        if line
            .to_lowercase()
            .contains(&search.as_str().to_lowercase())
        {
            lines.push((String::from(line), line_num));
        }
    }

    lines
}

fn display_results(lines: &Vec<(String, i16)>, search: &String) {
    println!("\"{}\" was found on lines:", &search);
    for i in 0..lines.len() {
        println!("Line {}:\n    {}", &lines[i].1, &lines[i].0);
    }
}
