use std::collections::HashMap;

pub fn get_solution(input: String) {
    let mut game_map: GameMap = read_map(&input);

    let max_y = input.lines().count();
    let max_x = input.lines().next().unwrap().chars().count();

    while step(&mut game_map, max_x, max_y) == Ok(()) {}
    dbg!(game_map.count_visited());
}

fn step(game_map: &mut GameMap, max_x: usize, max_y: usize) -> Result<(), ()> {
    let ((guard_x, guard_y), guard_cell) = game_map.get_guard();

    let guard_x: i32 = *guard_x as i32;
    let guard_y: i32 = *guard_y as i32;

    let (forward_x, forward_y) = match guard_cell {
        Cell::Guard(Guard::Up) => (guard_x, guard_y - 1),
        Cell::Guard(Guard::Down) => (guard_x, guard_y + 1),
        Cell::Guard(Guard::Left) => (guard_x - 1, guard_y),
        Cell::Guard(Guard::Right) => (guard_x + 1, guard_y),
        _ => panic!("error"),
    };

    if forward_x < 0
        || forward_x > max_x as i32 - 1
        || forward_y < 0
        || forward_y > max_y as i32 - 1
    {
        *guard_cell = Cell::Visited;
        return Err(());
    }

    let guard_cell: Cell = guard_cell.clone();
    let forward_cell = game_map.get((forward_x as usize, forward_y as usize));

    if *forward_cell == Cell::Obstacle {
        if let Cell::Guard(g) = game_map.get((guard_x as usize, guard_y as usize)) {
            g.turn();
            return Ok(());
        } else {
            panic!("error");
        }
    }

    *forward_cell = guard_cell;
    *game_map.get((guard_x as usize, guard_y as usize)) = Cell::Visited;
    game_map.guard_pos = (forward_x as usize, forward_y as usize);

    return Ok(());
}

fn read_map(input: &str) -> GameMap {
    let mut game_map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '.' => game_map.insert((x, y), Cell::Empty),
                '#' => game_map.insert((x, y), Cell::Obstacle),
                '^' => game_map.insert((x, y), Cell::Guard(Guard::Up)),
                _ => panic!("error unsupported symbol"),
            };
        }
    }
    let guard_pos = *game_map.iter_mut().find(|(_, v)| v.is_guard()).unwrap().0;
    return GameMap {
        map: game_map,
        guard_pos,
    };
}

pub fn get_solution2(input: String) {
    let mut game_map: GameMap = read_map(&input);

    let max_y = input.lines().count();
    let max_x = input.lines().next().unwrap().chars().count();

    while step(&mut game_map, max_x, max_y) == Ok(()) {}

    let mut loop_counter = 0;
    for ((x, y), _) in game_map.map.iter().filter(|(_, v)| **v == Cell::Visited) {
        let mut game_map_tmp: GameMap = read_map(&input);
        if *game_map_tmp.get_guard().0 == (*x, *y) {
            continue;
        }
        *game_map_tmp.get((*x, *y)) = Cell::Obstacle;

        let mut i = 0;
        while step(&mut game_map_tmp, max_x, max_y) == Ok(()) {
            i += 1;
            if i > 20000 {
                loop_counter += 1;
                if loop_counter % 100 == 0 {
                    dbg!(loop_counter);
                }
                break;
            }
        }
    }
    dbg!(loop_counter);
}

struct GameMap {
    map: HashMap<(usize, usize), Cell>,
    guard_pos: (usize, usize),
}

impl GameMap {
    fn get_guard(&mut self) -> (&(usize, usize), &mut Cell) {
        return (
            &self.guard_pos,
            (self.map.get_mut(&self.guard_pos).unwrap()),
        );
    }

    fn get(&mut self, (x, y): (usize, usize)) -> &mut Cell {
        return self.map.get_mut(&(x, y)).unwrap();
    }

    fn count_visited(&self) -> usize {
        let mut visited = 0;
        for (_, v) in self.map.iter() {
            if *v == Cell::Visited {
                visited += 1;
            }
        }

        return visited;
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Guard {
    Up,
    Down,
    Left,
    Right,
}

impl Guard {
    fn turn(&mut self) {
        *self = match *self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Obstacle,
    Guard(Guard),
    Visited,
    Empty,
}

impl Cell {
    fn is_guard(&self) -> bool {
        match self {
            Self::Guard(_) => return true,
            _ => return false,
        }
    }
}
