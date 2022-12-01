use std::fs;

fn main() {
    let input_file_path = "test.input";
    let contents = fs::read_to_string(input_file_path).unwrap();
    println!("{}", contents);
}