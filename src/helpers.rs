use std::fs;
pub fn read_and_split_file(file_path: &str) -> Vec<String> {
    let contents: String = fs::read_to_string(file_path).unwrap();
    let split_contents: Vec<String> = contents.trim().split("\n").map(|s| s.to_string()).collect();
    split_contents
}
