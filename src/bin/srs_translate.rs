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
    let mut verbose = false;
    let mut path = std::env::args()
        .nth(1)
        .expect("Please enter filename or `--help`");
    if path == "--help" || path == "-h" {
        print_help();
        std::process::exit(0);
    } else if path == "-v" {
        verbose = true;
        path = std::env::args()
            .nth(2)
            .expect("Please enter filename or `--help`");
    } else if std::env::args().count() == 3
        && std::env::args().nth(2).expect("Unknown argument") == "-v"
    {
        verbose = true;
    }
    let contents = filereader::read_string_from(path);
    let tokens = tokenizer::tokenize(&contents);
    let mut complete_tokens: Vec<Vec<srs_commands::CompleteToken>> = Vec::new();
    for line in &tokens.0 {
        complete_tokens.push(tokenizer::strings_to_enum(line));
    }
    if verbose {
        println!("Tokens:");
        for line in complete_tokens {
            println!("===  SEXPR  ===");
            for token in line {
                println!("{:?}", token);
            }
        }
        println!("Strings to be replaced:");
        for string in tokens.1 {
            println!("{:?}", string);
        }
    }
}

fn print_help() {
    println!("{} <filename> [ -v ]", std::env::args().nth(0).unwrap());
    println!("\t<filename> - file with source code");
    println!("\t-v - flag for verbose output")
}
