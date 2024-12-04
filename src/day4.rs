// solution < 2613 < 4378

pub fn get_solution(input: String) {
    let matrix: Vec<Vec<char>> = read_matrix(input);

    let mut n_xmas: usize = 0;

    for s in left_rigth_iter(&matrix) {
        let str: String = s.map(|(c, _, _)| c).collect();
        n_xmas += count_xmas(str);
    }
    for s in left_rigth_iter(&matrix) {
        let str: String = s.map(|(c, _, _)| c).rev().collect();
        n_xmas += count_xmas(str);
    }

    for s in up_down_iter(&matrix) {
        let str: String = s.map(|(c, _, _)| c).collect();
        n_xmas += count_xmas(str);
    }
    for s in up_down_iter(&matrix) {
        let str: String = s.map(|(c, _, _)| c).rev().collect();
        n_xmas += count_xmas(str);
    }

    for s in diagonal_iter(&matrix) {
        let str: String = s.map(|(c, _, _)| c).collect();
        n_xmas += count_xmas(str);
    }
    for s in diagonal_iter(&matrix) {
        let str: String = s.map(|(c, _, _)| c).rev().collect();
        n_xmas += count_xmas(str);
    }

    for s in diagonal_iter_negative(&matrix) {
        let str: String = s.map(|(c, _, _)| c).collect();
        n_xmas += count_xmas(str);
    }
    for s in diagonal_iter_negative(&matrix) {
        let str: String = s.map(|(c, _, _)| c).rev().collect();
        n_xmas += count_xmas(str);
    }

    println!("solution1: {}", n_xmas);
}

fn count_xmas(str: String) -> usize {
    let mut num = 0;
    for n in 0..str.len() {
        let (_, section) = str.split_at(n);
        if section.starts_with("XMAS") {
            num += 1;
        }
    }
    return num;
}

pub fn get_solution2(input: String) {
    let matrix: Vec<Vec<char>> = read_matrix(input);

    let mut a_list: Vec<(usize, usize)> = vec![];

    for s in diagonal_iter(&matrix) {
        for found_a in find_mas(s.collect()) {
            a_list.push(found_a);
        }
    }
    for s in diagonal_iter(&matrix) {
        for found_a in find_mas(s.rev().collect()) {
            a_list.push(found_a);
        }
    }

    for s in diagonal_iter_negative(&matrix) {
        for found_a in find_mas(s.collect()) {
            a_list.push(found_a);
        }
    }
    for s in diagonal_iter_negative(&matrix) {
        for found_a in find_mas(s.rev().collect()) {
            a_list.push(found_a);
        }
    }

    for a in &a_list {
        if !(*matrix.get(a.1).unwrap().get(a.0).unwrap() == 'A') {
            panic!("found a not a");
        }
    }
    dbg!(a_list.len());

    let mut already_seen_list: Vec<(usize, usize)> = vec![];
    let mut seen_twice: Vec<(usize, usize)> = vec![];

    for a in a_list {
        if seen_twice.contains(&a) {
            continue;
        }
        if already_seen_list.contains(&a) {
            seen_twice.push(a);
            continue;
        }
        already_seen_list.push(a);
    }

    // for n in 0..50 {
    //     dbg!(seen_twice.get(n).unwrap());
    // }

    dbg!(seen_twice.len());
}

fn find_mas(input: Vec<(&char, usize, usize)>) -> Vec<(usize, usize)> {
    let mut a_list = vec![];
    for n in 0..input.len() {
        let (_, section) = input.split_at(n);
        if section.len() < 3 {
            continue;
        }
        if *section.get(0).unwrap().0 == 'M'
            && *section.get(1).unwrap().0 == 'A'
            && *section.get(2).unwrap().0 == 'S'
        {
            a_list.push((section.get(1).unwrap().1, section.get(1).unwrap().2));
        }
    }
    return a_list;
}

fn read_matrix(input: String) -> Vec<Vec<char>> {
    let mut v = vec![];

    for line in input.lines() {
        v.push(line.chars().collect());
    }

    return v;
}

fn left_rigth_iter(
    v: &Vec<Vec<char>>,
) -> Vec<impl DoubleEndedIterator<Item = (&char, usize, usize)>> {
    let mut new_vec = vec![];
    for (y, line) in v.iter().enumerate() {
        new_vec.push(line.iter().enumerate().map(move |(x, c)| (c, x, y)));
    }
    return new_vec;
}

fn up_down_iter(v: &Vec<Vec<char>>) -> Vec<impl DoubleEndedIterator<Item = (&char, usize, usize)>> {
    let mut new_vec = vec![];
    let y_len = v.len();
    let x_len = v.get(0).unwrap().len();

    for x in 0..x_len {
        let mut x_v: Vec<(&char, usize)> = vec![];
        for y in 0..y_len {
            x_v.push((v.get(y).unwrap().get(x).unwrap(), y));
        }
        new_vec.push(x_v.into_iter().map(move |(c, y)| (c, x, y)));
    }
    return new_vec;
}

fn diagonal_iter_negative(
    v: &Vec<Vec<char>>,
) -> Vec<impl DoubleEndedIterator<Item = (&char, usize, usize)>> {
    let mut new_vec = vec![];
    let y_len = v.len();
    let x_len = v.get(0).unwrap().len();

    let mut starting_coords: Vec<(usize, usize)> = vec![];
    for x in 0..x_len {
        starting_coords.push((x, 0));
    }
    for y in 1..y_len {
        starting_coords.push((x_len - 1, y));
    }

    for (x_start, y_start) in starting_coords {
        let mut step = 0;
        let mut diagonal: Vec<(&char, usize, usize)> = vec![];
        loop {
            if step > x_start {
                break;
            }
            let c = v
                .get(y_start + step)
                .map(|r| r.get(x_start - step))
                .flatten();
            if c.is_none() {
                break;
            }
            diagonal.push((c.unwrap(), x_start - step, y_start + step));
            step += 1;
        }
        new_vec.push(diagonal.into_iter());
    }

    return new_vec;
}

fn diagonal_iter(
    v: &Vec<Vec<char>>,
) -> Vec<impl DoubleEndedIterator<Item = (&char, usize, usize)>> {
    let mut new_vec = vec![];
    let y_len = v.len();
    let x_len = v.get(0).unwrap().len();

    let mut starting_coords: Vec<(usize, usize)> = vec![];
    for x in 0..x_len {
        starting_coords.push((x, 0));
    }
    for y in 1..y_len {
        starting_coords.push((0, y));
    }

    for (x_start, y_start) in starting_coords {
        let mut step = 0;
        let mut diagonal: Vec<(&char, usize, usize)> = vec![];
        loop {
            let c = v
                .get(y_start + step)
                .map(|r| r.get(x_start + step))
                .flatten();
            if c.is_none() {
                break;
            }
            diagonal.push((c.unwrap(), x_start + step, y_start + step));
            step += 1;
        }
        new_vec.push(diagonal.into_iter());
    }

    return new_vec;
}
