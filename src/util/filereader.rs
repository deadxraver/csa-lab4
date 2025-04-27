use std::io::Read;

pub fn read_string_from(path: String) -> String {
    let mut file = std::fs::File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unexpected error while reading file contents");
    contents
}
