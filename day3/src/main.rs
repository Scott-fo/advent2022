use std::collections::HashSet;

fn main() {
    let input = include_str!("puzzle-input.txt");

    part_one(&input);
    part_two(&input);
}

fn get_priority(char: char) -> usize {
    if char.is_uppercase() {
        ((char as usize) % 32) + 26
    } else {
        (char as usize) % 32
    }
}

fn part_one(input: &str) {
    let sum: usize = input
        .split("\n")
        .map(|backpack| {
            let chars: Vec<char> = backpack.chars().collect();
            let pivot: usize = chars.len() / 2;

            if pivot != 0 {
                let first_compartment = &chars[0..pivot];
                let first_compartment_set: HashSet<char> =
                    HashSet::from_iter(first_compartment.iter().copied());

                let second_compartment = &chars[pivot..];

                for char in second_compartment {
                    if first_compartment_set.contains(char) {
                        return get_priority(*char) as usize;
                    }
                }

                0
            } else {
                0
            }
        })
        .sum();

    println!("Part 1: {:?}", sum);
}

fn part_two(input: &str) {
    let mut score: usize = 0;
    let backpack_array = input
        .split("\n")
        .flat_map(|backpack| backpack.split("\n"))
        .collect::<Vec<&str>>();

    let mut start_index = 0;

    while start_index <= backpack_array.len() - 2 {
        let first_backpack: Vec<char> = backpack_array[start_index].chars().collect();
        let second_backpack: Vec<char> = backpack_array[start_index + 1].chars().collect();
        let third_backpack: Vec<char> = backpack_array[start_index + 2].chars().collect();

        let first_backpack_set: HashSet<char> = HashSet::from_iter(first_backpack.iter().copied());
        let second_backpack_set: HashSet<char> =
            HashSet::from_iter(second_backpack.iter().copied());

        for char in third_backpack {
            if first_backpack_set.contains(&char) && second_backpack_set.contains(&char) {
                score += get_priority(char);
                break;
            }
        }

        start_index += 3;
    }

    println!("Part 2: {:?}", score);
}
