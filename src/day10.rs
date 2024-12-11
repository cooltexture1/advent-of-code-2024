use core::panic;

pub fn get_solution(input: String) {
    let map = read_input(&input);

    let mut total_score = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, num) in line.iter().enumerate() {
            if *num == 0 {
                total_score += calc_score((x, y), &map);
                // println!("score for ({x}, {y}) is {}", calc_score((x, y), &map));
            }
        }
    }
    println!("solution1: {}", total_score);
}

pub fn get_solution2(input: String) {
    let map = read_input(&input);

    let mut total_score = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, num) in line.iter().enumerate() {
            if *num == 0 {
                total_score += get_rating((x, y), &map);
            }
        }
    }
    println!("solution2: {}", total_score);
}

fn read_input(input: &str) -> Vec<Vec<usize>> {
    let mut v = vec![];
    for line in input.lines() {
        v.push(
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect(),
        );
    }
    return v;
}

fn get_rating((x, y): (usize, usize), map: &Vec<Vec<usize>>) -> usize {
    let mut found = vec![(x, y)];
    for n in 1..10 {
        found = get_surrounding_n(found, n, &map);
    }
    return found.len();
}

fn calc_score((x, y): (usize, usize), map: &Vec<Vec<usize>>) -> usize {
    let mut found = vec![(x, y)];
    for n in 1..10 {
        found = get_surrounding_n(found, n, &map);
        found.sort();
        found.dedup();
    }
    return found.len();
}

fn get_surrounding_n(
    start_points: Vec<(usize, usize)>,
    n: usize,
    map: &Vec<Vec<usize>>,
) -> Vec<(usize, usize)> {
    let mut v = vec![];
    for start_point in start_points {
        for i in 0..4 {
            let Some((x, y)) = get_in_dir(start_point, ADDS[i]) else {
                continue;
            };
            if map.get(y).map(|e| e.get(x)).flatten().is_none() {
                panic!("error");
            }
            if map
                .get(y)
                .map(|e| e.get(x))
                .flatten()
                .is_some_and(|e| *e == n)
            {
                v.push((x, y));
            }
        }
    }
    return v;
}

const ADDS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

const MAX: isize = 56;
// const MAX: isize = 7;

fn get_in_dir(start_point: (usize, usize), add: (isize, isize)) -> Option<(usize, usize)> {
    let x = (start_point.0 as isize).checked_add(add.0)?;
    let y = (start_point.1 as isize).checked_add(add.1)?;
    if x < 0 || x > MAX || y < 0 || y > MAX {
        return None;
    }
    return Some((x as usize, y as usize));
}
