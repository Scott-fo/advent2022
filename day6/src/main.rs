use std::collections::HashSet;

fn main() {
    let input = include_str!("puzzle-input.txt");
    let input: Vec<char> = input.chars().collect();

    let mut head: usize = 14;
    let mut tail: usize = 0;

    while head < input.len() {
        let result = check_for_repeat(&input[tail..head]);
        if result {
            break;
        } else {
            head += 1;
            tail += 1;
        }
    }

    println!("{:?}", head);
}

fn check_for_repeat(slice: &[char]) -> bool {
    let mut db = HashSet::new();
    for num in slice.iter() {
        if db.contains(num) {
            return false;
        } else {
            db.insert(num);
        }
    }

    true
}
