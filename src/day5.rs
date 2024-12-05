pub fn get_solution(input: String) {
    let rules = load_rules(&input);
    let sequences = load_sequences(&input);

    let mut sum = 0;
    for sequence in &sequences {
        if valid_sequence(&sequence, &rules) {
            sum += sequence.get(sequence.len() / 2).unwrap();
        }
    }
    println!("solution1: {}", sum);
}

fn load_rules(input: &str) -> Vec<(usize, usize)> {
    let mut v = vec![];
    for line in input.lines() {
        if line == "" {
            break;
        }
        let mut split = line.split('|');
        let num1 = split.next().unwrap().parse().unwrap();
        let num2 = split.next().unwrap().parse().unwrap();
        v.push((num1, num2));
    }

    return v;
}

fn load_sequences(input: &str) -> Vec<Vec<usize>> {
    let mut v = vec![];

    for line in input.lines() {
        if !(line.contains(',')) {
            continue;
        }

        let mut line_vec = vec![];
        for n in line.split(',') {
            line_vec.push(n.parse().unwrap());
        }
        v.push(line_vec);
    }

    return v;
}

fn valid_sequence(sequence: &Vec<usize>, rules: &Vec<(usize, usize)>) -> bool {
    for (i, num) in sequence.iter().enumerate() {
        let (before_list, after_list) = sequence.split_at(i);
        for before in before_list {
            if rules.iter().any(|(b, a)| b == num && a == before) {
                return false;
            }
        }
        for after in after_list {
            if rules.iter().any(|(b, a)| b == after && a == num) {
                return false;
            }
        }
    }
    return true;
}

pub fn get_solution2(input: String) {
    let rules = load_rules(&input);
    let sequences = load_sequences(&input);

    let mut sum = 0;
    for sequence in &sequences {
        if valid_sequence(&sequence, &rules) {
            continue;
        }
        let mut ordered_seq = sequence.clone();
        loop {
            ordered_seq = order_sequence(&ordered_seq, &rules);
            if valid_sequence(&ordered_seq, &rules) {
                sum += ordered_seq.get(sequence.len() / 2).unwrap();
                break;
            }
        }
    }
    println!("{}", sum);
}

fn order_sequence(sequence: &Vec<usize>, rules: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut new_vec: Vec<usize> = sequence.clone();

    for (i, num) in new_vec.clone().iter().enumerate() {
        let tmp = new_vec.clone();
        let (before_list, _) = tmp.split_at(i);
        for (j, before) in before_list.iter().enumerate() {
            if rules.iter().any(|(b, a)| b == num && a == before) {
                new_vec.swap(i, j);
            }
        }
        let tmp = new_vec.clone();
        let (_, after_list) = tmp.split_at(i);
        for (j, after) in after_list.iter().enumerate() {
            if rules.iter().any(|(b, a)| b == after && a == num) {
                new_vec.swap(i, j);
            }
        }
    }

    return new_vec;
}
