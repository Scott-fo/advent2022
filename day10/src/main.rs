use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<isize> = VecDeque::new();
    let mut x: HashMap<isize, isize> = HashMap::new();

    let mut running_total = 1;
    let mut target_idx: isize = 20;

    include_str!("puzzle-input.txt").lines().for_each(|line| {
        let instruction: Vec<&str> = line.split_whitespace().collect();
        match instruction[0] {
            "addx" => {
                let val = instruction[1].parse::<isize>().unwrap();
                queue.push_back(val);
            }
            "noop" => {
                queue.push_back(0);
            }
            _ => unreachable!(),
        }
    });

    let mut cycle = 1;
    while queue.len() > 0 {
        let val = queue.pop_front().unwrap();
        if val != 0 {
            cycle += 1;
        }

        if cycle == target_idx.try_into().unwrap() {
            x.insert(target_idx, running_total);
            target_idx += 40;
        }

        running_total += val;
        cycle += 1;

        if cycle == target_idx.try_into().unwrap() {
            x.insert(target_idx, running_total);
            target_idx += 40;
        }
    }

    let sum: isize = x.iter().map(|cycle| cycle.0 * cycle.1).sum();

    dbg!(&sum);
}
