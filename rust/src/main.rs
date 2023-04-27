use crate::day1::{day1_1, day1_2};
use crate::day2::{day2_1, day2_2};
use traits::PrintableResult;

mod utils;
mod day1;
mod traits;
mod day2;

fn main() {
    day1_1("input_day1.txt").printable_result("day 1.1:");
    day1_2("input_day1.txt").printable_result("day 1.2:");

    day2_1("input_day2.txt").printable_result("day 2.1:");
    day2_2("input_day2.txt").printable_result("day 2.2:");
}
