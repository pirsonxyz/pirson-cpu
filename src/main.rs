use std::fs;

fn main() {
    let source = "example";
    let file = fs::read_to_string(source).expect("File not found");
    let parts: Vec<&str> = file.split('_').collect();
    println!("{:#?}", parts);
}
