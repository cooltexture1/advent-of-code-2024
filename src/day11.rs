pub fn get_solution(input: String) {
    let mut v: Vec<usize> = input
        .split(' ')
        .map(|e| e.trim().parse().unwrap())
        .collect();

    dbg!(&v);
    for n in 0..75 {
        v = next_step(v);
        dbg!(n);
    }

    // dbg!(&v);
    dbg!(v.len());
}

pub fn get_solution2(input: String) {}

fn next_step(v: Vec<usize>) -> Vec<usize> {
    let mut new_vec = vec![];

    for stone in v {
        // rule 1
        if stone == 0 {
            new_vec.push(1);
            continue;
        }
        // rule 2
        if get_base10_digits_len(stone) % 2 == 0 {
            let (n1, n2) = get_base10_digits_split(stone);
            new_vec.push(n1);
            new_vec.push(n2);
            continue;
        }
        // rule 3
        new_vec.push(stone * 2024);
    }

    return new_vec;
}

fn get_base10_digits_split(n: usize) -> (usize, usize) {
    let s = format!("{}", n);
    let (s1, s2) = s.split_at(s.len() / 2);
    return (s1.parse().unwrap(), s2.to_owned().parse().unwrap());
}

fn get_base10_digits_len(n: usize) -> u32 {
    return n.checked_ilog10().unwrap_or(0) + 1;
}
