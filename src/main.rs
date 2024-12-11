use std::fs::read_to_string;

mod day10;
use day10::{get_solution, get_solution2};

fn main() {
    let i: u8 = 10;
    let input: String = read_to_string(format!("inputs/day{}", i)).unwrap();

    let _test_input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
        .to_string();

    get_solution(input.clone());
    get_solution2(input);
}
