pub fn get_solution(input: String) {
    let mut sum = 0;
    for line in input.lines() {
        for n in 0..line.len() {
            let (_, section) = line.split_at(n);
            if let Some(num) = starts_with_mul(section) {
                sum += num;
            }
        }
    }

    println!("sum: {}", sum);
}

pub fn get_solution2(input: String) {
    let mut sum = 0;
    let mut is_do = true;
    for line in input.lines() {
        for n in 0..line.len() {
            let (_, section) = line.split_at(n);
            check_do(section, &mut is_do);
            if !is_do {
                continue;
            }
            if let Some(num) = starts_with_mul(section) {
                sum += num;
            }
        }
    }

    println!("sum2: {}", sum);
}

fn check_do(section: &str, is_do: &mut bool) {
    if section.starts_with("do()") {
        *is_do = true;
    }
    if section.starts_with("don't()") {
        *is_do = false;
    }
}

fn starts_with_mul(section: &str) -> Option<i32> {
    let mul_removed = section.strip_prefix("mul(")?;
    let numbers_section = mul_removed.split(')').next()?;

    if !numbers_section.chars().all(is_digit_or_comma) {
        return None;
    };

    let splits: Vec<&str> = numbers_section.split(',').collect();
    if splits.iter().count() != 2 {
        return None;
    }

    let (num1, num2): (i32, i32) = (
        splits.get(0)?.parse().unwrap(),
        splits.get(1)?.parse().unwrap(),
    );

    return Some(num1 * num2);
}

fn is_digit_or_comma(c: char) -> bool {
    c.is_ascii_digit() || c == ','
}
