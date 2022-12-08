fn main() {
    let map: Vec<Vec<u32>> = include_str!("puzzle-input.txt")
        .lines()
        .map(|line| {
            line.chars().flat_map(|num| num.to_digit(10)).collect::<Vec<u32>>()
        })
        .collect();

    walk(&map)
}

fn walk(map: &Vec<Vec<u32>>) {
    let mut trees_seen = 0;
    for y_idx in 0..map.len() {
        for x_idx in 0..map[0].len() {
            let up = check_up(&map, x_idx, y_idx, map[y_idx][x_idx]);
            if up == true {
                trees_seen += 1;
                continue;
            }

            let down = check_down(&map, x_idx, y_idx, map[y_idx][x_idx]);
            if down == true {
                trees_seen += 1;
                continue;
            }

            let left = check_left(&map, x_idx, y_idx, map[y_idx][x_idx]);
            if left == true {
                trees_seen += 1;
                continue;
            }

            let right = check_right(&map, x_idx, y_idx, map[y_idx][x_idx]);
            if right == true {
                trees_seen += 1;
                continue;
            }
        }
    }

    dbg!(trees_seen);
}

fn check_up(map: &Vec<Vec<u32>>, x: usize, y: usize, original_val: u32) -> bool {
    if x == 0 || y == 0 || x == map[0].len() - 1 || y == map.len() - 1 {
        return true;
    }

    if map[y - 1][x] < original_val {
        if y - 1 == 0 {
            return true;
        }

        else {
            check_up(map, x, y - 1, original_val)
        }
    } else {
        return false;
    }

}

fn check_down(map: &Vec<Vec<u32>>, x: usize, y: usize, original_val: u32) -> bool {
    if map[y + 1][x] < original_val {
        if y + 1 == map.len() - 1 {
            return true;
        }

        else {
            check_down(map, x, y + 1, original_val)
        }
    } else {
        return false;
    }
}

fn check_left(map: &Vec<Vec<u32>>, x: usize, y: usize, original_val: u32) -> bool {
    if map[y][x - 1] < original_val {
        if x - 1 == 0 {
            return true;
        }

        else {
            check_left(map, x - 1, y, original_val)
        }
    } else {
        return false;
    }
}

fn check_right(map: &Vec<Vec<u32>>, x: usize, y: usize, original_val: u32) -> bool {
    if map[y][x + 1] < original_val {
        if x + 1 == map[0].len() - 1 {
            return true;
        }

        else {
            check_right(map, x + 1, y, original_val)
        }
    } else {
        return false;
    }
}

