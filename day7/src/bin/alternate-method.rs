fn main() {
    let input = std::fs::read_to_string("puzzle-input.txt").unwrap();

    let mut stack = vec![0];

    let mut total = 0;

    for line in input.lines().filter(|l| !l.is_empty()) {
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd ") {
            let dir = &line[5..];
            if dir == ".." {
                let amount = stack.pop().unwrap();

                if amount <= 100000 {
                    total += amount;
                }

                *stack.last_mut().unwrap() += amount;
            } else {
                stack.push(0)
            }

            continue;
        }

        let (amount, _) = line.split_once(" ").unwrap();
        if let Ok(amount) = amount.parse::<usize>() {
            *stack.last_mut().unwrap() += amount
        } else if amount == "dir" {
        }
    }

    while stack.len() > 1 {
        let amount = stack.pop().unwrap();
        if amount <= 100000 {
            total += amount;
        }
        *stack.last_mut().unwrap() += amount;
    }

    println!("{:?}", total)
}
