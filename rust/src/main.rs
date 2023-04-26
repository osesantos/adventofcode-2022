use crate::day1::{day1_1, day1_2};
use traits::PrintableResult;

mod utils;
mod day1;
mod traits;

fn main() {
    day1_1("input_day1.txt").printable_result("day 1.1:");
    day1_2("input_day1.txt").printable_result("day 1.1:");
}
