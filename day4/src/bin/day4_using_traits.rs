use anyhow::Result;
use std::str::FromStr;

struct Assignment {
    start: usize,
    end: usize,
}

struct Assignments {
    left: Assignment,
    right: Assignment,
}

impl Assignment {
    pub fn fully_contains(&self, other: &Assignment) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }
}

impl FromStr for Assignment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once("-").expect("No separator found");
        return Ok(Assignment {
            start: a.parse()?,
            end: b.parse()?,
        });
    }
}

impl Assignments {
    pub fn one_assignment_contains_the_other(&self) -> bool {
        return self.left.fully_contains(&self.right) || self.right.fully_contains(&self.left);
    }
}

impl FromStr for Assignments {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(",").expect("No separator found");
        return Ok(Assignments {
            left: left.parse()?,
            right: right.parse()?,
        });
    }
}

fn main() -> Result<()> {
    println!(
        "{:?}",
        std::fs::read_to_string("./src/bin/puzzle-input.txt")?
            .lines()
            .flat_map(|line| line.parse::<Assignments>())
            .filter(|assignments| assignments.one_assignment_contains_the_other())
            .count()
    );

    Ok(())
}
