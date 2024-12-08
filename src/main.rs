use std::fs::read_to_string;

mod day8;
use day8::{get_solution, get_solution2};

fn main() {
    let i: u8 = 8;
    let input: String = read_to_string(format!("inputs/day{}", i)).unwrap();

    let test_input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
        .to_string();

    get_solution(input.clone());
    get_solution2(input);
}
