use std::collections::VecDeque;

pub fn get_solution(input: String) {
    let mut fs = read_input(&input);

    while swap(&mut fs) {}

    let mut sum = 0;
    for (i, block) in fs.iter().enumerate() {
        if let Some(num) = block {
            sum += i * num;
        } else {
            panic!("error2");
        }
    }
    println!("solution1: {}", sum);
}

fn swap(fs: &mut VecDeque<Option<usize>>) -> bool {
    loop {
        let elem = fs.pop_back().unwrap();
        if elem.is_some() {
            fs.push_back(elem);
            break;
        }
    }

    if let Some((i, _)) = fs.iter().enumerate().find(|(_, e)| e.is_none()) {
        let check = fs.swap_remove_back(i);
        if let Some(Some(_)) = check {
            panic!("error");
        }
        if check.is_none() {
            panic!("error1");
        }
        return true;
    } else {
        return false;
    }
}

fn read_input(input: &str) -> VecDeque<Option<usize>> {
    let mut fs: VecDeque<Option<usize>> = VecDeque::new();

    for (i, char) in input.chars().enumerate() {
        if char == '\n' {
            break;
        }
        let num: u8 = char.to_string().parse().unwrap();
        if i % 2 == 0 {
            for _ in 0..num {
                fs.push_back(Some(i / 2));
            }
        } else {
            for _ in 0..num {
                fs.push_back(None);
            }
        }
    }
    return fs;
}

pub fn get_solution2(input: String) {
    let mut fs = read_input2(&input);

    let max_id = fs.back().unwrap().id;
    // print_long_form(&fs);
    for n in (0..max_id + 1).rev() {
        move_n(&mut fs, n);
        // print_long_form(&fs);
    }

    let long_form = long_form(fs);

    let mut sum = 0;
    for (i, block) in long_form.iter().enumerate() {
        sum += i * block;
    }
    println!("solution2: {}", sum);
}

fn move_n(fs: &mut VecDeque<File>, n: usize) {
    let (i, ref_to_n) = fs.iter().enumerate().find(|(_, e)| e.id == n).unwrap();
    let file_n = ref_to_n.clone();

    if !fs
        .iter()
        .enumerate()
        .any(|(ind, e)| ind < i && e.space_after >= ref_to_n.size)
    {
        return;
    }
    if i == 0 {
        return;
    }
    let mut moving_file = fs.remove(i).unwrap();
    fs.get_mut(i - 1).unwrap().space_after += file_n.size + moving_file.space_after;

    let (index, before_file) = fs
        .iter_mut()
        .enumerate()
        .find(|(_, e)| e.space_after >= file_n.size)
        .unwrap();

    moving_file.space_after = before_file.space_after - file_n.size;
    before_file.space_after = 0;

    fs.insert(index + 1, moving_file);
}

fn print_long_form(fs: &VecDeque<File>) {
    let long_form = long_form(fs.clone());
    let mut s: String = "".to_string();
    for num in long_form {
        if num == 0 {
            s.push('.');
        } else {
            s.push_str(&num.to_string());
        }
    }
    println!("{}", s);
}

// fn move_a_file(fs: &mut VecDeque<File>) -> bool {
//     let Some((mut moving_file, index)) = get_last_to_change(fs) else {
//         return false;
//     };
//     dbg!(&moving_file);
//
//     let mut insert = None;
//     let mut space_left = 0;
//     for (i, file) in fs.iter_mut().enumerate() {
//         if file.space_after >= moving_file.size {
//             space_left = file.space_after - moving_file.size;
//             file.space_after = 0;
//             insert = Some(i);
//             break;
//         }
//     }
//     if let Some(i) = insert {
//         moving_file.space_after = space_left;
//         fs.insert(i + 1, moving_file);
//     } else {
//         fs.insert(fs.len() - index, moving_file);
//     }
//
//     fs.iter_mut().last().unwrap().space_after = 0;
//     return true;
// }

// fn get_last_to_change(fs: &mut VecDeque<File>) -> Option<(File, usize)> {
//     let (i, _) = fs.iter().rev().enumerate().find(|(_, e)| !e.to_ignore)?;
//     let moving_file = fs.remove(fs.len() - i - 1).unwrap();
//     return Some((moving_file, i));
// }

fn read_input2(input: &str) -> VecDeque<File> {
    let mut fs: VecDeque<File> = VecDeque::new();
    for (id, chars) in input.chars().collect::<Vec<char>>().chunks(2).enumerate() {
        fs.push_back(File {
            id,
            size: chars[0].to_string().parse().unwrap(),
            space_after: chars[1].to_string().parse().unwrap_or(0),
        });
    }

    return fs;
}

fn long_form(fs: VecDeque<File>) -> Vec<usize> {
    let mut v = vec![];
    for file in fs {
        for _ in 0..file.size {
            v.push(file.id);
        }
        for _ in 0..file.space_after {
            v.push(0);
        }
    }

    return v;
}

#[derive(Debug, Clone, Copy)]
struct File {
    id: usize,
    size: usize,
    space_after: usize,
}
