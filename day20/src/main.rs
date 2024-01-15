use std::{io::Read, collections::HashSet};

// https://github.com/LinAGKar/advent-of-code-2023-rust/blob/master/day22/src/main.rs

fn part_1(input: &str) -> i64 {
    // Assumption of NOT set was invalid... ugh.
    let mut back = vec![];
    let mut next = vec![];
    let v: Vec<_> = input.split("\n").map(|x| x.parse::<i64>().unwrap()).enumerate().collect();
    for (i, line) in v.iter().enumerate() {
        back.push(*line);
        if i != 0 {
            next.push(*line);
        }
    }
    back[0] = *v.last().unwrap();
    next.push(v[0]);
    
    let mut vcopy = v.clone();
    for (i, elem) in v.iter() {
        // println!("{:?}", vcopy);
        let idx: i64 = vcopy.iter().position(|x| *x == (*i, *elem)).unwrap() as i64;
        let mut new_idx = (idx + elem) % (vcopy.len() - 1) as i64;
        if new_idx <= 0 {
            new_idx += (vcopy.len() - 1) as i64;
        }
        // println!("idx: {}, new_idx: {}", idx, new_idx);
        vcopy.remove(idx as usize);
        vcopy.insert(new_idx as usize, (*i, *elem));
    }

    let idx = vcopy.iter().position(|x| x.1 == 0).unwrap() as usize;
    println!("{:?} {}", vcopy, vcopy.len());
    println!("{} {} {}", vcopy[(idx + 1000) % vcopy.len()].1, vcopy[(idx + 2000) % vcopy.len()].1, vcopy[(idx + 3000) % vcopy.len()].1);
    vcopy[(idx + 1000) % vcopy.len()].1 + vcopy[(idx + 2000) % vcopy.len()].1 + vcopy[(idx + 3000) % vcopy.len()].1

}

fn part_2(input: &str) -> usize {
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