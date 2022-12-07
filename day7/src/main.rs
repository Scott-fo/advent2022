// Not my solution
// Worked on my own (in bin file)
// Got close but had to park it and take it as a learning experience
//
// Key Takeaways:
// The approach of making a node { value, children: node[] }
// Is the approach I originally wanted to take but was put off from reading around trees in rust
// Lesson learned, should have explored it first!
//
// Thank you to Amos for the excellent write up:
// https://fasterthanli.me/series/advent-of-code-2022/part-7
// Really enjoyed learning about nom as well, seems like it will be useful for the next days
//
// I would like to revisit this later on so that I can come up with an original approach
use camino::Utf8PathBuf;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );

    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

#[derive(Debug)]
struct FsEntry {
    path: Utf8PathBuf,
    size: u64,
    children: Vec<FsEntry>,
}

impl FsEntry {
    fn total_size(&self) -> u64 {
        self.size + self.children.iter().map(|c| c.total_size()).sum::<u64>()
    }

    fn all_dirs(&self) -> Box<dyn Iterator<Item = &FsEntry> + '_> {
        Box::new(
            std::iter::once(self).chain(
                self.children
                    .iter()
                    .filter(|c| !c.children.is_empty())
                    .flat_map(|c| c.all_dirs()),
            ),
        )
    }
}

fn main() {
    let lines = include_str!("puzzle-input.txt")
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1);

    let mut stack = vec![FsEntry {
        path: "/".into(),
        size: 0,
        children: vec![],
    }];

    for line in lines {
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {}
                Command::Cd(path) => match path.as_str() {
                    "/" => {}
                    ".." => {
                        let child = stack.pop();
                        stack.last_mut().unwrap().children.push(child.unwrap());
                    }
                    _ => {
                        let node = FsEntry {
                            path: path.clone(),
                            size: 0,
                            children: vec![],
                        };
                        stack.push(node);
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {}
                Entry::File(size, path) => {
                    let node = FsEntry {
                        size,
                        path,
                        children: vec![],
                    };

                    stack.last_mut().unwrap().children.push(node);
                }
            },
        }
    }

    let mut root = stack.pop().unwrap();
    while let Some(mut next) = stack.pop() {
        next.children.push(root);
        root = next;
    }

    dbg!(&root);

    // Part 1

    let sum = root
        .all_dirs()
        .map(|d| d.total_size())
        .filter(|&s| s <= 100000)
        .sum::<u64>();

    dbg!(sum);

    // Part 2

    let total_space: u64 = 70000000;
    let required_space: u64 = 30000000;
    let occupied_space: Vec<u64> = root.all_dirs().map(|d| d.total_size()).collect();

    let space_to_clear = required_space - (total_space - occupied_space[0]);
    let space_to_clear = root
        .all_dirs()
        .map(|d| d.total_size())
        .filter(|&s| s >= space_to_clear)
        .min()
        .unwrap();

    dbg!(space_to_clear);
}
