#[path = "../translator/asm_commands.rs"]
mod asm_commands;
#[path = "../translator/ast_maker.rs"]
mod ast_maker;
#[path = "../util/filereader.rs"]
mod filereader;
#[path = "../translator/srs_commands.rs"]
mod srs_commands;
#[path = "../translator/tokenizer.rs"]
mod tokenizer;

fn main() {
    let path = std::env::args().nth(1).expect("Please enter filename or `--help`");
    if path == "--help" || path == "-h" {
        print_help();
        std::process::exit(0);
    }
    let contents = filereader::read_string_from(path);
    let tokens = tokenizer::tokenize(&contents);
    // TODO: remove everything below this line, it is for testing components
    println!("Tokens:");
    for line in tokens.0 {
        println!("{:?}", line);
    }
    println!("Strings to be replaced:");
    for string in tokens.1 {
        println!("{:?}", string);
    }
}

fn print_help() {
    println!("{} <filename>", std::env::args().nth(0).unwrap());
}
