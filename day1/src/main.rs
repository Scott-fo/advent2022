fn main() {
    let input = include_str!("../puzzle-input.txt");
    let mut lines: Vec<usize> = input
        .split("\n\n")
        .map(|line| {
            line.split("\n")
                .flat_map(|num| num.parse::<usize>())
                .sum::<usize>()
        })
        .collect();
    //.max(); // Instead of collect for part 1

    lines.sort_by(|a, b| b.cmp(a));
    println!("Biggest: {:?}\nTop 3 Sum: {:?}", lines[0], (lines[0] + lines[1] + lines[2]));
}
