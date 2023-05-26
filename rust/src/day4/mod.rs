use crate::utils::get_file_lines;

pub fn day4_1(file_name: &str) -> String {
    let lines = get_file_lines(file_name);
    let ranges: Vec<_>  = lines.iter().map(| l | l.split(",").collect::<Vec<_>>()).collect();
    let ranges2: Vec<_>  = ranges.iter().map(| l | l.into_iter().map(| r | r.split("-").collect::<Vec<_>>()).collect::<Vec<_>>()).collect();
    let rangesNum: Vec<_> = ranges2.iter().map(|a| a.iter().map(|b| (b.first()..=b.last())))
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
