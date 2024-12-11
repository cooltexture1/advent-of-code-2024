use std::fs::read_to_string;

mod day11;
use day11::{get_solution, get_solution2};

fn main() {
    let i: u8 = 11;
    let input: String = read_to_string(format!("inputs/day{}", i)).unwrap();

    let test_input = "125 17".to_string();

    get_solution(input.clone());
    get_solution2(input);
}
