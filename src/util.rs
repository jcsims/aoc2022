use std::fs;

pub fn slurp(filepath: &str) -> String {
    fs::read_to_string(filepath).expect("Error reading file from disk")
}
