pub fn rethink_newlines(input: String) -> String {
    let mut new_str = String::new();
    let mut scopes: u8 = 0;
    for c in input.chars() {
        if c != '\n' {
            new_str.push(c);
            if c == '(' {
                scopes += 1;
            } else if c == ')' {
                scopes -= 1;
                if scopes == 0 {
                    new_str.push('\n');
                }
            }
        }
    }
    let spl = new_str.split("\n");
    let mut ret_str = String::new();
    for c in spl {
        ret_str += c.trim();
        ret_str.push('\n');
    }
    ret_str
}
#[allow(dead_code)]
pub fn extract_strings(input: &str, storage: &mut Vec<String>) -> String {
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
