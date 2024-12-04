pub fn get_solution(input: String) {
    let matrix: Vec<Vec<char>> = read_matrix(input);

    let mut n_xmas: usize = 0;

    for s in left_rigth_iter(&matrix) {
        let str: String = s.collect();
        n_xmas += count_xmas(str);
    }
    for s in left_rigth_iter(&matrix) {
        let str: String = s.rev().collect();
        n_xmas += count_xmas(str);
    }

    for s in up_down_iter(&matrix) {
        let str: String = s.collect();
        n_xmas += count_xmas(str);
    }
    for s in up_down_iter(&matrix) {
        let str: String = s.rev().collect();
        n_xmas += count_xmas(str);
    }

    for s in diagonal_iter(&matrix) {
        let str: String = s.collect();
        n_xmas += count_xmas(str);
    }
    for s in diagonal_iter(&matrix) {
        let str: String = s.rev().collect();
        n_xmas += count_xmas(str);
    }

    let diagonal = diagonal_iter_negative(&matrix);
    for diag in diagonal {
        let str: String = diag.collect();
        // dbg!(str);
    }

    // for s in diagonal_iter_negative(&matrix) {
    //     let str: String = s.collect();
    //     n_xmas += count_xmas(str);
    // }
    // for s in diagonal_iter_negative(&matrix) {
    //     let str: String = s.rev().collect();
    //     n_xmas += count_xmas(str);

    dbg!(n_xmas);
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

pub fn get_solution2(input: String) {}

fn read_matrix(input: String) -> Vec<Vec<char>> {
    let mut v = vec![];

    for line in input.lines() {
        v.push(line.chars().collect());
    }

    return v;
}

fn left_rigth_iter(v: &Vec<Vec<char>>) -> Vec<impl DoubleEndedIterator<Item = &char>> {
    let mut new_vec = vec![];
    for line in v {
        new_vec.push(line.iter());
    }
    return new_vec;
}

fn up_down_iter(v: &Vec<Vec<char>>) -> Vec<impl DoubleEndedIterator<Item = &char>> {
    let mut new_vec = vec![];
    let y_len = v.len();
    let x_len = v.get(0).unwrap().len();

    for x in 0..x_len {
        let mut x_v: Vec<&char> = vec![];
        for y in 0..y_len {
            x_v.push(v.get(y).unwrap().get(x).unwrap());
        }
        new_vec.push(x_v.into_iter());
    }
    return new_vec;
}

fn diagonal_iter_negative(v: &Vec<Vec<char>>) -> Vec<impl DoubleEndedIterator<Item = &char>> {
    let mut new_vec = vec![];
    let y_len = v.len();
    let x_len = v.get(0).unwrap().len();

    let mut starting_coords: Vec<(usize, usize)> = vec![];
    for x in 0..x_len {
        starting_coords.push((x, y_len));
    }
    for y in 1..y_len {
        starting_coords.push((x_len, y));
    }
    dbg!(starting_coords.clone());

    for (x_start, y_start) in starting_coords {
        let mut step = 0;
        let mut diagonal: Vec<&char> = vec![];
        loop {
            let c = v
                .get(y_start - step)
                .map(|r| r.get(x_start - step))
                .flatten();
            if c.is_none() {
                break;
            }
            diagonal.push(c.unwrap());
            step += 1;
        }
        new_vec.push(diagonal.into_iter());
    }

    return new_vec;
}

fn diagonal_iter(v: &Vec<Vec<char>>) -> Vec<impl DoubleEndedIterator<Item = &char>> {
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
        let mut diagonal: Vec<&char> = vec![];
        loop {
            let c = v
                .get(y_start + step)
                .map(|r| r.get(x_start + step))
                .flatten();
            if c.is_none() {
                break;
            }
            diagonal.push(c.unwrap());
            step += 1;
        }
        new_vec.push(diagonal.into_iter());
    }

    return new_vec;
}
