use crate::day1::{day1_1, day1_2};
use crate::day2::{day2_1, day2_2};
use traits::PrintableResult;
use crate::day3::{day3_1, day3_2};

mod utils;
mod day1;
mod traits;
mod day2;
mod day3;

fn main() {
    day1_1("sample_day1.txt").printable_result("sample day 1.1:");
    day1_2("sample_day1.txt").printable_result("sample day 1.2:");
    day1_1("input_day1.txt").printable_result("day 1.1:");
    day1_2("input_day1.txt").printable_result("day 1.2:");

    println!();

    day2_1("sample_day2.txt").printable_result("sample day 2.1:");
    day2_2("sample_day2.txt").printable_result("sample day 2.2:");
    day2_1("input_day2.txt").printable_result("day 2.1:");
    day2_2("input_day2.txt").printable_result("day 2.2:");

    println!();

    day3_1("sample_day3.txt").printable_result("sample day 3.1:");
    day3_2("sample_day3.txt").printable_result("sample day 3.2:");
    day3_1("input_day3.txt").printable_result("day 3.1:");
    day3_2("input_day3.txt").printable_result("day 3.2:");

    println!();
}
