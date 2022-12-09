use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

fn main() {
    let lines = include_str!("puzzle-input.txt").lines();
    let mut seen: HashMap<isize, HashSet<isize>> = HashMap::new();

    let mut t = Point { x: 0, y: 0 };
    let mut h = Point { x: 0, y: 0 };

    seen.insert(t.y, HashSet::new());
    seen.get_mut(&t.y).unwrap().insert(t.x);

    for instruction in lines {
        let instruction: Vec<&str> = instruction.split_whitespace().collect();
        let distance = instruction[1].parse::<isize>().unwrap();

        match_move(instruction[0], distance, &mut t, &mut h, &mut seen)
    }

    println!(
        "Positions visited: {:?}",
        seen.iter().map(|set| { set.1.len() }).sum::<usize>()
    );
}

fn match_move(
    direction: &str,
    distance: isize,
    t: &mut Point,
    h: &mut Point,
    seen: &mut HashMap<isize, HashSet<isize>>,
) {
    match direction {
        "R" => {
            (0..distance).for_each(|_| {
                h.x += 1;
                if h.x.abs_diff(t.x) > 1 && h.y.abs_diff(t.y) == 1 {
                    t.y = h.y;
                    t.x = h.x - 1;
                    if !seen.contains_key(&t.y) {
                        seen.insert(t.y, HashSet::new());
                    }
                    seen.get_mut(&t.y).unwrap().insert(t.x);
                } else if h.x.abs_diff(t.x) > 1 {
                    t.x += 1;
                    seen.get_mut(&t.y).unwrap().insert(t.x);
                }
            });
        }
        "L" => {
            (0..distance).for_each(|_| {
                h.x -= 1;
                if h.x.abs_diff(t.x) > 1 && h.y.abs_diff(t.y) == 1 {
                    t.y = h.y;
                    t.x = h.x + 1;
                    if !seen.contains_key(&t.y) {
                        seen.insert(t.y, HashSet::new());
                    }
                    seen.get_mut(&(t.y as isize)).unwrap().insert(t.x);
                } else if t.x.abs_diff(h.x) > 1 {
                    t.x -= 1;
                    seen.get_mut(&t.y).unwrap().insert(t.x);
                }
            });
        }
        "U" => {
            (0..distance).for_each(|_| {
                h.y += 1;
                if h.y.abs_diff(t.y) > 1 && h.x.abs_diff(t.x) == 1 {
                    t.x = h.x;
                    t.y = h.y - 1;
                    if !seen.contains_key(&t.y) {
                        seen.insert(t.y, HashSet::new());
                    }
                    seen.get_mut(&(t.y as isize)).unwrap().insert(t.x);
                } else if h.y.abs_diff(t.y) > 1 {
                    t.y += 1;
                    if !seen.contains_key(&t.y) {
                        seen.insert(t.y, HashSet::new());
                    }
                    seen.get_mut(&t.y).unwrap().insert(t.x);
                }
            });
        }
        "D" => {
            (0..distance).for_each(|_| {
                h.y -= 1;
                if h.y.abs_diff(t.y) > 1 && h.x.abs_diff(t.x) == 1 {
                    t.x = h.x;
                    t.y = h.y + 1;
                    if !seen.contains_key(&t.y) {
                        seen.insert(t.y, HashSet::new());
                    }
                    seen.get_mut(&t.y).unwrap().insert(t.x);
                } else if t.y.abs_diff(h.y) > 1 {
                    t.y -= 1;
                    if !seen.contains_key(&t.y) {
                        seen.insert(t.y, HashSet::new());
                    }
                    seen.get_mut(&t.y).unwrap().insert(t.x);
                }
            });
        }
        _ => unreachable!(),
    }
}
