// This successfully builds the tree
// But I struggled to get it to actually sum the values correctly.
// I had it adding the file sizes to the directory, but not bubbling those values up to the
// wrapping directories
use anyhow::Result;

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    dir_name: T,
    parent: Option<usize>,
    children: Vec<usize>,
    file_size: Option<usize>,
}

#[derive(Debug, Default)]
struct ArenaWrapper<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, dir_name: T) -> Self {
        Self {
            idx,
            dir_name,
            parent: None,
            children: Vec::new(),
            file_size: None,
        }
    }
}

impl<T> ArenaWrapper<T>
where
    T: PartialEq,
{
    fn get_or_add_index(&mut self, dir_name: T) -> usize {
        for node in &self.arena {
            if node.dir_name == dir_name {
                return node.idx;
            }
        }

        let idx = self.arena.len();
        self.arena.push(Node::new(idx, dir_name));
        idx
    }

    fn insert(&mut self, new_dir: T, parent_dir: T, file_size: usize) {
        let parent_idx = self.get_or_add_index(parent_dir);
        let child_idx = self.get_or_add_index(new_dir);

        self.arena[child_idx].parent = Some(parent_idx);
        self.arena[child_idx].file_size = Some(file_size);
        self.arena[parent_idx].children.push(child_idx);
    }
}

fn main() -> Result<()> {
    let mut tree: ArenaWrapper<&str> = ArenaWrapper::default();
    let mut parent_dir = vec!["/"];
    let input = std::fs::read_to_string("./src/puzzle-input.txt")?;
    input.lines().skip(1).for_each(|line| {
        if line.starts_with("$") {
            let instruction = line.split_whitespace().collect::<Vec<&str>>();
            let command = instruction[1];

            match command {
                "cd" => match instruction[2] {
                    ".." => {
                        parent_dir.pop();
                    }
                    _ => {
                        parent_dir.push(instruction[2]);
                    }
                },
                "ls" => {}
                _ => unreachable!(),
            }
        } else {
            let files = line.split_whitespace().collect::<Vec<&str>>();
            if files[0].contains("dir") {
                tree.insert(files[1], parent_dir.last().unwrap(), 0);
            } else {
                tree.insert(
                    files[1],
                    parent_dir.last().unwrap(),
                    files[0].parse::<usize>().unwrap(),
                );
            }
        }
    });

    println!("{:?}", tree);

    Ok(())
}

