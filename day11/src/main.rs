use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: (&'static str, &'static str),
    divisor: usize,
    throw_to: (usize, usize),
}

fn main() {
    let mut inspections: HashMap<usize, usize> = HashMap::new();
    let mut monkeys: HashMap<usize, Monkey> = include_str!("puzzle-input.txt")
        .split("\n\n")
        .enumerate()
        .map(|(idx, monkey)| {
            let info = monkey.split("\n").collect::<Vec<&str>>();
            let items = info[1]
                .split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .filter_map(|num| num.parse::<usize>().ok())
                .collect::<VecDeque<usize>>();

            let operation = info[2]
                .split_once("= old ")
                .unwrap()
                .1
                .split_whitespace()
                .collect::<Vec<&str>>();
            let divisor = info[3]
                .split_whitespace()
                .filter_map(|num| num.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            let success = info[4]
                .split_whitespace()
                .filter_map(|num| num.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            let fail = info[5]
                .split_whitespace()
                .filter_map(|num| num.parse::<usize>().ok())
                .collect::<Vec<usize>>();

            inspections.insert(idx, 0);

            (
                idx,
                Monkey {
                    items,
                    operation: (operation[0], operation[1]),
                    divisor: divisor[0],
                    throw_to: (success[0], fail[0]),
                },
            )
        })
        .collect::<HashMap<usize, Monkey>>();

    let length = monkeys.len();

    let mut lcm = 1;
    for monkey in &monkeys {
        lcm *= monkey.1.divisor;
    }

    for _ in 0..10000 {
        for i in 0..length {
            let mut monkey = monkeys.remove(&i).unwrap();
            if monkey.items.len() > 0 {
                for _ in 0..monkey.items.len() {
                    *inspections.get_mut(&i).unwrap() += 1;
                    let mut item = monkey.items.pop_front().unwrap();
                    let op_val: usize;

                    if monkey.operation.1 == "old" {
                        op_val = item;
                    } else {
                        op_val = monkey.operation.1.parse::<usize>().unwrap();
                    }

                    match monkey.operation.0 {
                        "+" => {
                            item += op_val;
                        }
                        "-" => {
                            item -= op_val;
                        }
                        "*" => {
                            item *= op_val;
                        }
                        "/" => {
                            item /= op_val;
                        }
                        _ => unreachable!(),
                    }

                    item = item % lcm;
                    //item /= 3;

                    if item % monkey.divisor == 0 {
                        let target = monkey.throw_to.0;
                        monkeys.get_mut(&target).unwrap().items.push_back(item);
                    } else {
                        let target = monkey.throw_to.1;
                        monkeys.get_mut(&target).unwrap().items.push_back(item);
                    }
                }
            }

            monkeys.insert(i, monkey);
        }
    }

    let mut results: Vec<usize> = inspections.into_values().map(|num| num).collect();
    results.sort_by(|a, b| b.cmp(a));

    println!("Result: {:?}", results[0] * results[1]);
}
