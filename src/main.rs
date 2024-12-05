use std::fs::read_to_string;

mod day5;
use day5::{get_solution, get_solution2};

fn main() {
    let i: u8 = 5;
    let input: String = read_to_string(format!("inputs/day{}", i)).unwrap();

    get_solution(input.clone());
    get_solution2(input);
}
