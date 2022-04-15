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
        display_results(&lines, search, &option);
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

fn display_results(lines: &Vec<(String, i16)>, search: &String, option: &Options) {
    println!("\"{}\" was found on lines:", &search);
    for i in 0..lines.len() {
        println!("Line {}:", &lines[i].1);
        display_word(&lines[i].0, search, option);
    }
}

fn display_word(line: &String, search: &String, option: &Options) {
    let mut line_temp;
	// send options with reference to a string that is upper or lower case
    let search_index: Vec<(usize, &str)> = match option {
        Options::Uppercase(_) => line.match_indices(search).collect(),
        Options::Lowercase(_) => {
            line_temp = line.to_lowercase();
            line_temp
                .match_indices(search.to_lowercase().as_str())
                .collect()
        }
    };
	
    let mut highlight_line = String::new();

    for (index, _) in search_index {
        if index == 0 {
            highlight_line.push_str("^".repeat(search.len()).as_str());
        } else {
            highlight_line.push_str(
                format!(
                    "{}{}",
                    " ".repeat(index - highlight_line.len()),
                    "^".repeat(search.len())
                )
                .as_str(),
            )
        }
    }

    println!("    {}\n    {}", line, highlight_line);
}
