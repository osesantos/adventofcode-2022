use crate::utils::get_file_lines;

pub fn day1_1(file_name: &str) {
    let lines = get_file_lines(file_name);
    for line in lines.into_iter() {
        println!("{}", line)
    }
}