use std::fs;
use std::str::Lines;

pub fn get_file_lines(file_name: &str) -> Lines {
    let content = fs::read_to_string("../inputs/".to_string() + file_name).unwrap();
    let lines = content.lines();
    return lines.collect()
}

