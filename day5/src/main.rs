use std::collections::VecDeque;

fn main() {
    let input = include_str!("puzzle-input.txt");

    part_one(&input);
    part_two(&input);
}

fn part_two(input: &str) {
    let number_of_stacks = (input.split_once("\n").unwrap().0.len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); number_of_stacks];

    for element in input.split("\n") {
        if element.contains("[") {
            let bytes = element.as_bytes();
            let mut index = 1;

            for idx in 0..number_of_stacks {
                if bytes[index] != 32 {
                    stacks[idx].push_back(bytes[index] as char);
                }
                index += 4
            }
        }

        if element.contains("move") {
            let instruction: Vec<usize> = element
                .split(" ")
                .filter_map(|num| num.parse::<usize>().ok())
                .collect();

            let mut item_vec: VecDeque<char> = VecDeque::new();
            for _ in 0..instruction[0] {
                item_vec.push_back(stacks[instruction[1] - 1].pop_front().unwrap());
            }

            item_vec.append(&mut stacks[instruction[2] - 1]);
            stacks[instruction[2] - 1] = item_vec;
        }
    }

    let mut result = VecDeque::new();
    for idx in 0..number_of_stacks {
        result.push_back(stacks[idx].pop_front().unwrap());
    }

    println!("Part 2: {:?}", result);
}

fn part_one(input: &str) {
    let number_of_stacks = (input.split_once("\n").unwrap().0.len() + 1) / 4;
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); number_of_stacks];

    for element in input.split("\n") {
        if element.contains("[") {
            let bytes = element.as_bytes();
            let mut index = 1;

            for idx in 0..number_of_stacks {
                if bytes[index] != 32 {
                    stacks[idx].push_back(bytes[index] as char);
                }
                index += 4
            }
        }

        if element.contains("move") {
            let instruction: Vec<usize> = element
                .split(" ")
                .filter_map(|num| num.parse::<usize>().ok())
                .collect();

            for _ in 0..instruction[0] {
                let item = stacks[instruction[1] - 1].pop_front().unwrap();
                stacks[instruction[2] - 1].push_front(item);
            }
        }
    }

    let mut result = VecDeque::new();
    for idx in 0..number_of_stacks {
        result.push_back(stacks[idx].pop_front().unwrap());
    }

    println!("Part 1: {:?}", result);
}
