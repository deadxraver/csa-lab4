#[path = "../util/formater.rs"]
mod formater;

pub fn tokenize(input: &str) -> (Vec<Vec<String>>, Vec<String>) {
    let mut lines: Vec<Vec<String>> = Vec::new();
    let mut strings: Vec<String> = Vec::new();
    for line in formater::rethink_newlines(formater::extract_strings(input, &mut strings))
        .trim()
        .split('\n')
    {
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
