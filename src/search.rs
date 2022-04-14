pub enum Options<'a> {
    Uppercase(&'a String),
    Lowercase(&'a String),
}

impl Options<'_> {
    fn unwrap(&self) -> &String {
        return match self {
            Options::Uppercase(search) => &search,
            Options::Lowercase(search) => &search,
        };
    }
}

pub fn find(option: Options, contents: &String) {
    let lines: Vec<(String, i16)> = match option {
        Options::Uppercase(search) => find_uppercase(search, contents),
        Options::Lowercase(search) => find_lowercase(search, contents),
    };

    let search: &String = option.unwrap();

    if lines.len() != 0 {
        display_results(&lines, search)
    } else {
        println!("\"{}\" was not found in the chosen file", search);
    }
}

fn find_uppercase(search: &String, contents: &String) -> Vec<(String, i16)> {
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
        println!("Line {}:", &lines[i].1);
        display_word(&lines[i].0, search);
    }
}

fn display_word(line: &String, search: &String) {
    let search_index: Vec<_> = line.match_indices(search).collect();
    let mut highlight_line = String::from(" ".repeat(line.len()));

    //
    for i in &search_index {
        println!("{:?}", i);
    }
    //

    for (index, _) in &search_index {
        println!("{}", index);
        highlight_line.replace_range(index..&(index + search.len()), "^");
    }

    println!("    {}", line);
    println!("	  {}", highlight_line);
}
