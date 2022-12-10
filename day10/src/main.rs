use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<isize> = VecDeque::new();
    let mut running_total = 1;

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

    let mut cycle = 0;
    while queue.len() > 0 {
        let val = queue.pop_front().unwrap();
        draw_crt(running_total, cycle);
        cycle += 1;

        if val != 0 {
            draw_crt(running_total, cycle);
            cycle += 1;
        }

        running_total += val;
    }
}

fn draw_crt(total: isize, cycle: isize) {
    let position = cycle % 40;
    if cycle != 0 && position == 0 {
        println!("")
    }
    if (total - 1) <= position && (total + 1) >= position {
        print!("#")
    } else {
        print!(".")
    }
}
