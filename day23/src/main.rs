use std::io::Read;

// https://github.com/LinAGKar/advent-of-code-2023-rust/blob/master/day22/src/main.rs

fn part_1(_input: &str) -> usize {
    // 643719258
    let mut cups = vec![6, 4, 3, 7, 1, 9, 2, 5, 8];

    let mut cc = 0;
    let l = cups.len();
    for _ in 0..100 {
        let m = cups[cc];
        let (a, b, c) = (cups[(cc + 1) % l], cups[(cc + 2) % l], cups[(cc + 3) % l]); 
        let mut n = m;
        loop {
            n -= 1;
            if n == 0 {
                n = 9;
            }
            if n == a || n == b || n == c {
                continue;
            } else {
                break;
            }
        }
        // Move 3 back
        let (u, v, x, y, z) = (cups[(cc + 4) % l], cups[(cc + 5) % l], cups[(cc + 6) % l], cups[(cc + 7) % l], cups[(cc + 8) % l]);
        match n {
            _ if n == u => {
                cups[(cc + 1) % l] = u;
                cups[(cc + 2) % l] = a;
                cups[(cc + 3) % l] = b;
                cups[(cc + 4) % l] = c;
            },
            _ if n == v => {
                cups[(cc + 1) % l] = u;
                cups[(cc + 2) % l] = v;
                cups[(cc + 3) % l] = a;
                cups[(cc + 4) % l] = b;
                cups[(cc + 5) % l] = c;
            },
            _ if n == x => {
                cups[(cc + 1) % l] = u;
                cups[(cc + 2) % l] = v;
                cups[(cc + 3) % l] = x;
                cups[(cc + 4) % l] = a;
                cups[(cc + 5) % l] = b;
                cups[(cc + 6) % l] = c;
            },
            _ if n == y => {
                cups[(cc + 1) % l] = u;
                cups[(cc + 2) % l] = v;
                cups[(cc + 3) % l] = x;
                cups[(cc + 4) % l] = y;
                cups[(cc + 5) % l] = a;
                cups[(cc + 6) % l] = b;
                cups[(cc + 7) % l] = c;
            },
            _ if n == z => {
                cups[(cc + 1) % l] = u;
                cups[(cc + 2) % l] = v;
                cups[(cc + 3) % l] = x;
                cups[(cc + 4) % l] = y;
                cups[(cc + 5) % l] = z;
                cups[(cc + 6) % l] = a;
                cups[(cc + 7) % l] = b;
                cups[(cc + 8) % l] = c;
            }
            _ => unreachable!(),
        }


        cc = (cc + 1) % l;

    }
    println!("{:?}", cups);
    0
}

fn part_2(input: &str) -> usize {
    // Use linkedlist.
    0
}

fn main() {

    let input = std::fs::read_to_string("src/day22.txt").unwrap();

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 1 result: {}", result);

    let start_time = std::time::Instant::now();
    let result = part_2(&input);
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 2 result: {}", result);
}