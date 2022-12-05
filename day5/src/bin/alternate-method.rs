use anyhow::Result;
use std::{fs, str::FromStr};

struct Crane {
    stack: Vec<Vec<char>>,
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Crane {
    fn move_single_item(&mut self, m: &Move) {
        for _ in 0..m.count {
            let item = self.stack[m.from].pop().unwrap();
            self.stack[m.to].push(item);
        }
    }

    fn move_three_items(&mut self, m: &Move) {
        let out_items = (0..m.count)
            .filter_map(|_| self.stack[m.from].pop())
            .collect::<Vec<_>>();

        for item in out_items.into_iter().rev() {
            self.stack[m.to].push(item);
        }
    }

    fn print_top(&self) {
        for stack in &self.stack {
            print!("{}", stack.last().unwrap());
        }

        println!()
    }
}

impl FromStr for Crane {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();
        for line in s.lines().rev().skip(1) {
            let mut idx = 0;
            let mut crane_idx = 0;

            while idx < line.len() {
                if stack.len() <= crane_idx {
                    stack.push(Vec::new());
                }

                if line[idx..].starts_with("[") {
                    let c = line.chars().nth(idx + 1).unwrap();
                    stack[crane_idx].push(c);
                }

                idx += 4;
                crane_idx += 1;
            }
        }

        Ok(Crane { stack })
    }
}

impl FromIterator<usize> for Move {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let count = iter.next().unwrap();
        let from = iter.next().unwrap() - 1;
        let to = iter.next().unwrap() - 1;

        Move { count, from, to }
    }
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split_whitespace()
            .filter_map(|num| num.parse::<usize>().ok())
            .collect::<Move>())
    }
}

fn run_restack(file: String) -> Result<()> {
    let (crane, moves) = file.split_once("\n\n").unwrap();
    let crane = crane.parse::<Crane>()?;

    let moves = moves
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse::<Move>().ok())
        .collect::<Vec<Move>>();

    //part_one(moves, crane);
    part_two(moves, crane);

    Ok(())
}

fn part_one(moves: Vec<Move>, mut crane: Crane) {
    for m in moves {
        crane.move_single_item(&m);
    }

    crane.print_top();

}

fn part_two(moves: Vec<Move>, mut crane: Crane) {
    for m in moves {
        crane.move_three_items(&m);
    }

    crane.print_top();

}

fn main() -> Result<()> {
    let file = fs::read_to_string("puzzle-input.txt")?;
    run_restack(file)?;

    Ok(())
}
