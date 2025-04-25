pub fn tokenize(input: &str) -> (Vec<Vec<String>>, Vec<String>) {
    let mut lines: Vec<Vec<String>> = Vec::new();
    let mut strings: Vec<String> = Vec::new();
    for line in extract_strings(input, &mut strings).trim().split('\n') {
        lines.push(
            line.replace("(", " ( ")
                .replace(")", " ) ")
                .trim()
                .split(' ')
                .map(|s| s.to_string())
                .filter(|s| s != "")
                .collect(),
        );
    }
    (lines, strings)
}

fn extract_strings(input: &str, storage: &mut Vec<String>) -> String {
    let mut new_str = String::new();
    let mut str_open: bool = false;
    let mut str_n: u8 = 0;
    for c in input.chars() {
        if c == '"' && !str_open {
            storage.push(String::new());
            str_open = true;
        } else if c == '"' && str_open {
            new_str.push('&');
            new_str += &str_n.to_string();
            str_open = false;
            str_n += 1;
        } else if str_open {
            storage[str_n as usize].push(c);
        } else {
            new_str.push(c);
        }
    }
    new_str
}