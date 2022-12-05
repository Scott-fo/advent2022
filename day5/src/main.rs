use std::collections::VecDeque;

fn main() {
    let input = include_str!("puzzle-input.txt");

    part_one(&input);
    part_two(&input);
}

fn part_two(input: &str) {
    let input: Vec<&str> = input.split("\n").collect();

    let number_of_stacks = (input[0].len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); number_of_stacks];

    for element in &input {
        if element.contains("1") {
            break;
        }

        let bytes = element.as_bytes();
        let mut index = 1;

        for idx in 0..number_of_stacks {
            if bytes[index] != 32 {
                stacks[idx].push_back(bytes[index] as char);
            }
            index += 4
        }
    }

    for instruction in &input {
        if !instruction.contains("move") {
            continue;
        }

        let instruction: Vec<usize> = instruction
            .split(" ")
            .filter_map(|num| num.parse::<usize>().ok())
            .collect();

        let mut item_vec: VecDeque<char> = VecDeque::new();
        for _ in 0..instruction[0] {
            item_vec.push_back(stacks[instruction[1] - 1].pop_front().unwrap());
        }

        item_vec.append(& mut stacks[instruction[2] - 1]);
        stacks[instruction[2] - 1] = item_vec;
    }

    println!("{:?}", stacks);
}

fn part_one(input: &str) {
    let input: Vec<&str> = input.split("\n").collect();

    let number_of_stacks = (input[0].len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); number_of_stacks];

    for element in &input {
        if element.contains("1") {
            break;
        }

        let bytes = element.as_bytes();
        let mut index = 1;

        for idx in 0..number_of_stacks {
            if bytes[index] != 32 {
                stacks[idx].push_back(bytes[index] as char);
            }
            index += 4
        }
    }

    for instruction in &input {
        if !instruction.contains("move") {
            continue;
        }

        let instruction: Vec<usize> = instruction
            .split(" ")
            .filter_map(|num| num.parse::<usize>().ok())
            .collect();

        for _ in 0..instruction[0] {
            let item = stacks[instruction[1] - 1].pop_front().unwrap();
            stacks[instruction[2] - 1].push_front(item);
        }
    }

    println!("{:?}", stacks);
}
