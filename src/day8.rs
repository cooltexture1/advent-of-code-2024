use std::{
    collections::{HashMap, HashSet},
    vec,
};

use itertools::Itertools;

pub fn get_solution(input: String) {
    let game = read_input(&input);

    let unique_chars = get_unique_chars(&input);

    let mut antinode_spots: HashSet<(usize, usize)> = HashSet::new();
    for char in unique_chars {
        let antennas = game.get(char);

        for two in antennas.iter().permutations(2) {
            let ant_a = two.get(0).unwrap();
            let ant_b = two.get(1).unwrap();
            let antinode = get_antinodes(**ant_a, **ant_b);

            if is_in_bounds(antinode, &game) {
                antinode_spots.insert((antinode.0 as usize, antinode.1 as usize));
            }
        }
    }

    dbg!(antinode_spots.len());
}

pub fn get_solution2(input: String) {
    let game = read_input(&input);

    let unique_chars = get_unique_chars(&input);

    let mut antinode_spots: HashSet<(usize, usize)> = HashSet::new();
    for char in unique_chars {
        let antennas = game.get(char);

        for two in antennas.iter().permutations(2) {
            let ant_a = two.get(0).unwrap();
            let ant_b = two.get(1).unwrap();
            let antinode_vec = get_antinode_vec(**ant_a, **ant_b);

            for n in -130..130 {
                let multiplied_vec: (i64, i64) = (antinode_vec.0 * n, antinode_vec.1 * n);
                let potential_antinode = (
                    ant_a.0 as i64 + multiplied_vec.0,
                    ant_a.1 as i64 + multiplied_vec.1,
                );

                if is_in_bounds(potential_antinode, &game) {
                    antinode_spots
                        .insert((potential_antinode.0 as usize, potential_antinode.1 as usize));
                }
            }
        }
    }

    dbg!(antinode_spots.len());
}

fn is_in_bounds(antinode: (i64, i64), game: &Game) -> bool {
    !(antinode.0 >= game.max_x as i64
        || antinode.0 < 0
        || antinode.1 >= game.max_y as i64
        || antinode.1 < 0)
}

fn read_input(input: &str) -> Game {
    let max_y = input.lines().count();
    let max_x = input.lines().next().unwrap().chars().count();

    let mut antennas = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.insert((x, y), c);
            }
        }
    }

    return Game {
        antennas,
        max_x,
        max_y,
    };
}

fn get_unique_chars(input: &str) -> HashSet<char> {
    let mut set = HashSet::new();
    for line in input.lines() {
        for char in line.chars() {
            if char == '.' {
                continue;
            }
            set.insert(char);
        }
    }
    return set;
}

struct Game {
    antennas: HashMap<(usize, usize), char>,
    max_x: usize,
    max_y: usize,
}

impl Game {
    fn get(&self, c: char) -> Vec<(usize, usize)> {
        let mut v = vec![];
        for (coords, char) in &self.antennas {
            if *char == c {
                v.push(*coords);
            }
        }
        return v;
    }
}

fn get_antinode_vec(ant_a: (usize, usize), ant_b: (usize, usize)) -> (i64, i64) {
    let a_x = ant_a.0 as i64;
    let b_x = ant_b.0 as i64;
    let a_y = ant_a.1 as i64;
    let b_y = ant_b.1 as i64;
    let atob: (i64, i64) = (b_x - a_x, b_y - a_y);

    let a_to_antinode: (i64, i64) = (-atob.0, -atob.1);
    return a_to_antinode;
}

fn get_antinodes(ant_a: (usize, usize), ant_b: (usize, usize)) -> (i64, i64) {
    let a_x = ant_a.0 as i64;
    let b_x = ant_b.0 as i64;
    let a_y = ant_a.1 as i64;
    let b_y = ant_b.1 as i64;
    let atob: (i64, i64) = (b_x - a_x, b_y - a_y);

    let a_to_antinode: (i64, i64) = (-atob.0, -atob.1);

    let antinode: (i64, i64) = (a_x + a_to_antinode.0, a_y + a_to_antinode.1);

    return antinode;
}
