use crate::utils::get_file_lines;

pub fn day4_1(file_name: &str) -> String {
    let lines = get_file_lines(file_name);
    let ranges: Vec<Vec<String>> = lines.iter().map(| l | l.split(",")).collect();
    "".to_string()
}

fn fully_contains(range1: &Vec<i64>, range2: &Vec<i64>) -> bool {
    if range1.first() >= range2.first() && range1.last() <= range2.last() {
        return true;
    }
    if range2.first() >= range1.first() && range2.last() <= range1.last() {
        return true;
    }
    false
}

pub fn day4_2(file_name: &str) -> String {
    "".to_string()
}
