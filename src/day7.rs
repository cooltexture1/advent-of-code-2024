use itertools::{repeat_n, Itertools};

pub fn get_solution(input: String) {
    let equations = read_input(&input);

    let mut total = 0;
    for equation in equations {
        let results = get_all_results(equation.values.as_slice(), &['+', '*']);

        if results.iter().any(|n| *n as u64 == equation.solution) {
            total += equation.solution;
        }
    }
    println!("solution1: {}", total);
}

pub fn get_solution2(input: String) {
    let equations = read_input(&input);

    let mut total = 0;
    for equation in equations {
        let results = get_all_results(equation.values.as_slice(), &['+', '*', '|']);

        if results.iter().any(|n| *n as u64 == equation.solution) {
            total += equation.solution;
        }
    }
    println!("solution2: {}", total);
}

fn get_all_results(values: &[usize], operators: &[char]) -> Vec<usize> {
    let combinations = repeat_n(operators, values.len() - 1).multi_cartesian_product();

    let mut v = vec![];
    for combination in combinations {
        let mut total: usize = *values.first().unwrap();

        for (i, operator) in combination.iter().enumerate() {
            match operator {
                '+' => total = total + values[i + 1],
                '*' => total = total * values[i + 1],
                '|' => total = concat_from_internet(total, values[i + 1]),
                _ => panic!("error"),
            }
        }

        v.push(total);
    }

    return v;
}

fn concat(a: usize, b: usize) -> usize {
    format!("{}{}", a, b).parse().unwrap()
}

fn concat_from_internet(a: usize, b: usize) -> usize {
    a as usize * 10usize.pow(b.ilog10() + 1) + b as usize
}

fn read_input(input: &str) -> Vec<Row> {
    let mut v = vec![];
    for line in input.lines() {
        let mut split = line.split(':');
        if split.clone().count() != 2 {
            panic!("error");
        }

        let solution: u64 = split.next().unwrap().parse().unwrap();
        let numbers: &str = split.next().unwrap();

        let mut values: Vec<usize> = vec![];
        for num in numbers.split(' ').filter(|s| *s != "") {
            values.push(num.parse().unwrap());
        }
        v.push(Row { solution, values });
    }
    return v;
}

#[derive(Debug)]
struct Row {
    solution: u64,
    values: Vec<usize>,
}
