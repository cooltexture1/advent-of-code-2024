pub fn get_solution(input: String) {
    let lines: Vec<Vec<i32>> = parse_input(input);

    let mut safe_count: usize = 0;
    for line in lines {
        if check_matches_criteria(&line) {
            safe_count += 1;
        }
    }
    dbg!(safe_count);
}

fn is_asc_or_desc(inp: &Vec<i32>) -> bool {
    let mut inp_asc = inp.clone();
    inp_asc.sort();

    let mut inp_desc = inp.clone();
    inp_desc.sort();
    inp_desc.reverse();

    *inp == inp_asc || *inp == inp_desc
}

fn parse_input(input: String) -> Vec<Vec<i32>> {
    let mut lines: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        lines.push(
            line.split(' ')
                .filter(|s| *s != "")
                .map(|e| e.parse().unwrap())
                .collect(),
        );
    }
    return lines;
}

fn check_matches_criteria(line: &Vec<i32>) -> bool {
    if !is_asc_or_desc(&line) {
        return false;
    }
    for (i, num) in line.iter().enumerate() {
        match line.get(i + 1) {
            Some(next) => {
                // check if differs
                let diff = (num - next).abs();
                if !(diff >= 1 && diff <= 3) {
                    return false;
                }
            }
            None => return true,
        }
    }
    return false;
}

pub fn get_solution2(input: String) {
    let lines: Vec<Vec<i32>> = parse_input(input);

    let mut safe_count: usize = 0;
    'outer: for line in lines {
        let mut line_permutations = vec![];
        for n in 0..line.len() {
            let mut new_line = line.clone();
            new_line.remove(n);
            line_permutations.push(new_line);
        }
        for permutation in line_permutations {
            if check_matches_criteria(&permutation) {
                safe_count += 1;
                continue 'outer;
            }
        }
    }
    dbg!(safe_count);
}
