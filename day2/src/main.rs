fn main() {
    let input = include_str!("../puzzle-input.txt");
    let lines = input.split("\n");
    let mut score: usize = 0;

    let lines: Vec<&str> = lines.flat_map(|line| line.split("\n")).collect();
    for line in lines.into_iter() {
        let strategy: Vec<&str> = line.split(" ").collect();
        match strategy[0] {
            "A" => score += choose_move(strategy[1], "Z", "X", "Y"),
            "B" => score += choose_move(strategy[1], "X", "Y", "Z"),
            "C" => score += choose_move(strategy[1], "Y", "Z", "X"),
            _ => panic!("Unexpected strategy"),
        }
    }

    println!("{:?}", score);
}

fn choose_outcome(input: &str) -> u8 {
    match input {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!("Unexpected outcome"),
    }
}

fn get_score(input: &str) -> u8 {
    match input {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("Unexpected move"),
    }
}

fn choose_move(strategy: &str, lose: &str, draw: &str, win: &str) -> usize {
    let result = choose_outcome(strategy) as usize;
    match result {
        0 => result + get_score(lose) as usize,
        3 => result + get_score(draw) as usize,
        6 => result + get_score(win) as usize,
        _ => panic!("Unexpected result"),
    }
}
