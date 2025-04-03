pub mod srs_commands;
pub mod asm_commands;
use std::io::Read;

fn main() {
    let path = std::env::args().nth(1).expect("Please enter filename");
    let mut file = std::fs::File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unexpected error while reading file contents");
    print!("{contents}");
    println!("{:#02x}", asm_commands::Opcodes::Add as u16);
}
