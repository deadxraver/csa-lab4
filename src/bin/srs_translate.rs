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
#[path = "../util/pretty_printer.rs"]
mod pretty_printer;

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
    let program = tokenizer::arrange_tokens(complete_tokens.clone(), tokens.1.clone());
    if verbose {
        println!("===  FINAL PROGRAM  ===");
        println!("Strings:");
        for string in program.strings {
            println!("{:?}", string);
        }
        println!("Functions:");
        for function in program.functions {
            pretty_printer::print_complete_tokens(function.clone());
        }
        println!("Constants:");
        for constant in program.constants {
            pretty_printer::print_complete_tokens(constant.clone());
        }
        println!("Code:");
        for code in program.code {
            pretty_printer::print_complete_tokens(code.clone());
        }
    }
}

fn print_help() {
    println!("{} <filename> [ -v ]", std::env::args().nth(0).unwrap());
    println!("\t<filename> - file with source code");
    println!("\t-v - flag for verbose output")
}
