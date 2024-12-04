pub fn get_solution(input: String) {
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];

    for line in input.lines() {
        let mut it = line.split(' ').filter(|s| *s != "");
        if !(it.clone().count() == 2) {
            panic!()
        }
        left_list.push(it.next().unwrap().parse().unwrap());
        right_list.push(it.next().unwrap().parse().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let mut sum: i32 = 0;
    for (elem_left, elem_right) in left_list.iter().zip(right_list) {
        sum += (elem_left - elem_right).abs();
    }

    println!("{}", sum);
}

pub fn get_solution2(input: String) {
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];

    for line in input.lines() {
        let mut it = line.split(' ').filter(|s| *s != "");
        if !(it.clone().count() == 2) {
            panic!()
        }
        left_list.push(it.next().unwrap().parse().unwrap());
        right_list.push(it.next().unwrap().parse().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let mut score: i32 = 0;

    for left_num in left_list {
        let amount: usize = right_list.iter().filter(|n| **n == left_num).count();

        score += left_num * amount as i32;
    }

    println!("{}", score);
}
