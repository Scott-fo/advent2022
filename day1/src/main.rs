use std::collections::BinaryHeap;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file_path = "puzzle-input.txt";
    let file = fs::File::open(file_path).expect("Could not find file");

    let reader = BufReader::new(file);
    let mut max_heap = BinaryHeap::new();
    let mut sum = 0;

    for line in reader.lines() {
        let entry = line.unwrap();
        if entry.is_empty() {
            max_heap.push(sum);
            sum = 0;
            continue;
        }

        sum = sum + entry.parse::<i64>().unwrap();
    }

    max_heap.push(sum);

    // Part 1
    // println!("Max value is: {:?}", max_heap.peek());

    let top_three_sum = max_heap.pop().unwrap() + max_heap.pop().unwrap() + max_heap.pop().unwrap();

    println!("Top three sum: {:?}", top_three_sum);
}
