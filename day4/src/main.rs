fn main() {
    let input = include_str!("puzzle-input.txt");

    part_one(&input);
    part_two(&input);
}

fn check_complete_overlap(vec: Vec<usize>) -> usize {
    if vec[0] <= vec[2] && vec[3] <= vec[1] {
        1
    } else if vec[2] <= vec[0] && vec[1] <= vec[3] {
        1
    } else {
        0
    }
}

fn part_one(input: &str) {
    let sum: usize = input
        .trim()
        .split("\n")
        .map(|pair| {
            check_complete_overlap(
                pair.split(",")
                    .flat_map(|assignment| {
                        assignment
                            .split("-")
                            .filter_map(|num| num.parse::<usize>().ok())
                    })
                    .collect::<Vec<usize>>(),
            )
        })
        .sum();

    println!("Part 1: {:?}", sum);
}

fn check_overlap(vec: Vec<usize>) -> usize {
    if vec[0] <= vec[3] && vec[2] <= vec[1] {
        1
    } else {
        0
    }
}

fn part_two(input: &str) {
    let sum: usize = input
        .trim()
        .split("\n")
        .map(|pair| {
            check_overlap(
                pair.split(",")
                    .flat_map(|assignment| {
                        assignment
                            .split("-")
                            .filter_map(|num| num.parse::<usize>().ok())
                    })
                    .collect::<Vec<usize>>(),
            )
        })
        .sum();

    println!("Part 2: {:?}", sum);
}
