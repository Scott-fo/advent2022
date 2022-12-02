fn main() {
    let input = include_str!("../puzzle-input.txt");
    let lines = input.split("\n");
    let mut score: usize = 0;

    let lines: Vec<&str> = lines.flat_map(|line| line.split("\n")).collect();
    for line in lines.into_iter() {
        let strategy: Vec<&str> = line.split(" ").collect();
        match strategy[0] {
            "A" => {
                score += get_score(strategy[1]) as usize;
                if strategy[1] == "Y" {
                    score += 6;
                } else if strategy[1] == "X" {
                    score += 3;
                }
            }
            "B" => {
                score += get_score(strategy[1]) as usize;
                if strategy[1] == "Z" {
                    score += 6;
                } else if strategy[1] == "Y" {
                    score += 3;
                }
            }
            "C" => {
                score += get_score(strategy[1]) as usize;
                if strategy[1] == "X" {
                    score += 6;
                } else if strategy[1] == "Z" {
                    score += 3;
                }
            }
            _ => {}
        }
    }

    println!("{:?}", score);
}

fn get_score(input: &str) -> u8 {
    match input {
       "X" => 1,
       "Y" => 2,
       "Z" => 3,
       _ => panic!("Unexpected move"),
    }
}
