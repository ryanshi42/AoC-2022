use std::io::Read;

// https://github.com/LinAGKar/advent-of-code-2023-rust/blob/master/day22/src/main.rs

fn part_1(input: &str) -> usize {
    0
}

fn part_2(input: &str) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 1 result: {}", result);

    let start_time = std::time::Instant::now();
    let result = part_2(&input);
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 2 result: {}", result);
}