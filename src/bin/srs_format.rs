#[path = "../util/filereader.rs"]
mod filereader;
#[path = "../util/formater.rs"]
mod formater;

fn main() { // FIXME: args order plays role
    let arg: String = std::env::args()
        .nth(1)
        .expect("Please enter key or filename");
    let inplace: bool;
    let path: String;
    if arg == "--help" || arg == "-h" {
        print_help();
        std::process::exit(0);
    } else if arg == "-i" || arg == "--inplace" {
        inplace = true;
        path = std::env::args().nth(2).expect("Please enter filename");
    } else {
        inplace = false;
        path = arg;
    }
    let contents = filereader::read_string_from(path.clone());
    let formated_content = formater::rethink_newlines(contents);
    if inplace {
        std::fs::write(path, formated_content).unwrap();
    } else {
        println!("{}", formated_content);
    }
}

fn print_help() {
    println!(
        "{} {}",
        std::env::args().nth(0).unwrap(),
        "--help | [--inplace] <filename>"
    );
}
