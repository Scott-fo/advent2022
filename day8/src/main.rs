#[derive(Debug)]
struct ScenicScore {
    up: usize,
    down: usize,
    left: usize,
    right: usize,
}

impl ScenicScore {
    fn get_product(&self) -> usize {
        self.up * self.down * self.left * self.right
    }
}

fn main() {
    let map: Vec<Vec<u32>> = include_str!("puzzle-input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|num| num.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect();

    walk(&map)
}

fn walk(map: &Vec<Vec<u32>>) {
    let mut trees_seen = 0;
    let mut scenic_score: Vec<ScenicScore> = vec![];
    for y_idx in 0..map.len() {
        for x_idx in 0..map[0].len() {
            scenic_score.push(ScenicScore {
                up: 0,
                down: 0,
                left: 0,
                right: 0,
            });

            let (found, _score) = check_edge(&map, x_idx, y_idx);
            if found == true {
                trees_seen += 1;
                continue
            }

            let (found, score) = check_up(&map, x_idx, y_idx, map[y_idx][x_idx], y_idx);
            scenic_score.last_mut().unwrap().up = score;
            if found == true {
                trees_seen += 1;
                //continue;
            }

            let (found, score) = check_down(&map, x_idx, y_idx, map[y_idx][x_idx], y_idx);
            scenic_score.last_mut().unwrap().down = score;
            if found == true {
                trees_seen += 1;
                //continue;
            }

            let (found, score) = check_left(&map, x_idx, y_idx, map[y_idx][x_idx], x_idx);
            scenic_score.last_mut().unwrap().left = score;
            if found == true {
                trees_seen += 1;
                //continue;
            }

            let (found, score) = check_right(&map, x_idx, y_idx, map[y_idx][x_idx], x_idx);
            scenic_score.last_mut().unwrap().right = score;
            if found == true {
                trees_seen += 1;
                //continue;
            }
        }
    }

    //dbg!(trees_seen);
    println!("Best score {:?}", scenic_score.iter().map(|score| score.get_product()).max().unwrap())
}

fn check_edge(
    map: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
) -> (bool, usize) {
    if x == 0 || y == 0 || x == map[0].len() - 1 || y == map.len() - 1 {
        return (true, 0);
    }

    (false, 0)
}

fn check_up(
    map: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    original_val: u32,
    original_y: usize,
) -> (bool, usize) {
    if map[y - 1][x] < original_val {
        if y - 1 == 0 {
            return (true, original_y);
        } else {
            check_up(map, x, y - 1, original_val, original_y)
        }
    } else {
        return (false, original_y - (y - 1));
    }
}

fn check_down(
    map: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    original_val: u32,
    original_y: usize,
) -> (bool, usize) {
    if map[y + 1][x] < original_val {
        if y + 1 == map.len() - 1 {
            return (true, map.len() - original_y - 1);
        } else {
            check_down(map, x, y + 1, original_val, original_y)
        }
    } else {
        return (false, y - (original_y - 1));
    }
}

fn check_left(
    map: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    original_val: u32,
    original_x: usize,
) -> (bool, usize) {
    if map[y][x - 1] < original_val {
        if x - 1 == 0 {
            return (true, original_x);
        } else {
            check_left(map, x - 1, y, original_val, original_x)
        }
    } else {
        return (false, original_x - (x - 1));
    }
}

fn check_right(
    map: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    original_val: u32,
    original_x: usize,
) -> (bool, usize) {
    if map[y][x + 1] < original_val {
        if x + 1 == map[0].len() - 1 {
            return (true, map[0].len() - original_x - 1);
        } else {
            check_right(map, x + 1, y, original_val, original_x)
        }
    } else {
        return (false, x - (original_x - 1));
    }
}
