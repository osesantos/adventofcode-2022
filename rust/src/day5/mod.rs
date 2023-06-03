use std::collections::HashMap;
use crate::utils::get_file_lines;

struct Movement {
    quantity: i32,
    start_stack: i32,
    end_stack: i32,
}

pub fn day5_1(file_name: &str) -> String {
    let lines = get_file_lines(file_name);
    let movements: Vec<_> = lines.iter().map(|l| l.movement_parser().unwrap_or(Movement{
        quantity: 0,
        start_stack: 0,
        end_stack: 0,
    })).into_iter().filter(|m| m.quantity != 0).collect();



    "".to_string()
}

fn convert_to_list(lines: &Vec<String>) -> Vec<String> {
    let mut lists: Vec<String> = Vec::new();


    let trimmed_elem = lines.iter().map(|&e| e.trim()).collect();

}

trait MovementExtensions {
    fn movement_parser(&self) -> Option<Movement>;
}
impl MovementExtensions for &String {
    fn movement_parser(&self) -> Option<Movement> {
        if !self.starts_with("move") {
            return None;
        }
        let raw: Vec<_>= self.split(" ").collect();
        return Some(Movement {
            quantity: raw[1].parse::<i32>().unwrap(),
            start_stack: raw[3].parse::<i32>().unwrap(),
            end_stack: raw[5].parse::<i32>().unwrap(),
        });
    }
}

pub fn day5_2(file_name: &str) -> String {
    "".to_string()
}