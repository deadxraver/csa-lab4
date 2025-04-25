#[path = "../translator/asm_commands.rs"]
mod asm_commands;
#[path = "../translator/srs_commands.rs"]
mod srs_commands;
#[path = "../translator/tokenizer.rs"]
mod tokenizer;
use std::io::Read;

fn main() {
    let path = std::env::args().nth(1).expect("Please enter filename");
    let mut file = std::fs::File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unexpected error while reading file contents");
    // TODO: remove everything below this line, it is for testing components
    let tokens = tokenizer::tokenize(&contents);
    println!("Tokens:");
    for line in tokens.0 {
        println!("{:?}", line);
    }
    println!("Strings to be replaced:");
    for string in tokens.1 {
        println!("{:?}", string);
    }
}
