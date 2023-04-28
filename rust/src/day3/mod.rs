use crate::utils::get_file_lines;

pub fn day3_1(file_name: &str) -> String {
    let lines = get_file_lines(file_name);
    let compartments: Vec<(&str, &str)> = lines.iter().map(|l| l.split_at(l.len() / 2)).collect();
    let dups: Vec<char> = compartments.iter().map(|c| find_dup(&c.0.chars().collect(), &c.1.chars().collect())).collect();
    total_score(&dups.iter().map(|c| calculate_priority(&c)).collect()).to_string()
}

fn total_score(scores: &Vec<i32>) -> i32 {
    scores.iter().sum()
}

fn find_dup(compartment_1: &Vec<char>, compartment_2: &Vec<char>) -> char {
    let mut dup: char = ' ';
    compartment_1.iter().for_each(|c| {
        if compartment_2.iter().find(|&&x| x == *c).is_some() {
            dup = *c;
        }
    });
    dup
}

/// Reduction factor to ASCII
/// uppercase -> 'A' is 65 while the score is 27 -> 38
/// lowercase -> 'a' is 97 while the score is 1 -> 96
///
fn calculate_priority(dup: &char) -> i32 {
    let dup_ascii = *dup as i32;
    if dup.is_ascii_uppercase() {
        dup_ascii - 38
    } else {
        dup_ascii - 96
    }
}

pub fn day3_2(file_name: &str) -> String {
    let lines = get_file_lines(file_name);
    "".to_string()
}
