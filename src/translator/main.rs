pub mod srs_commands;
use std::io::Read;

fn main() {
    let path = std::env::args().nth(1).expect("Please enter filename");
    let mut file = std::fs::File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unexpected error while reading file contents");
    print!("{contents}");
}
