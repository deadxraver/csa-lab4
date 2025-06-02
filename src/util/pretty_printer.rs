use crate::srs_commands::CompleteToken;

pub fn print_complete_token(token: CompleteToken) {
    println!("type: {:?}, token: {:?}, numeric_rep: {}, string_rep: '{}'", token.token_type, token.token, token.numeric_rep, token.string_rep);
}

pub fn print_complete_tokens(tokens: Vec<CompleteToken>) {
    println!("[");
    for token in tokens {
        print!("\t");
        print_complete_token(token);
    }
    println!("]");
}