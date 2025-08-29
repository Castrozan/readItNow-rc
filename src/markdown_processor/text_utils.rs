use regex::Regex;

pub fn normalize_whitespace(text: &str) -> String {
    let space_regex = Regex::new(r" +").unwrap();
    let result = space_regex.replace_all(text, " ");

    let newline_regex = Regex::new(r"\n\s*\n\s*\n+").unwrap();
    let result = newline_regex.replace_all(&result, "\n\n");

    let trailing_space_regex = Regex::new(r" +\n").unwrap();
    trailing_space_regex.replace_all(&result, "\n").to_string()
}

pub fn wrap_lines(text: &str, max_length: usize) -> String {
    text.lines()
        .map(|line| {
            if line.len() <= max_length {
                line.to_string()
            } else {
                wrap_single_line(line, max_length)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn wrap_single_line(line: &str, max_length: usize) -> String {
    let words: Vec<&str> = line.split_whitespace().collect();
    let mut result = Vec::new();
    let mut current_line = String::new();

    for word in words {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + 1 + word.len() <= max_length {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            result.push(current_line);
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        result.push(current_line);
    }

    result.join("\n")
}
