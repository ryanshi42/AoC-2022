use std::{io::Read, collections::{HashMap, HashSet}, fs};

// https://github.com/LinAGKar/advent-of-code-2023-rust/blob/master/day22/src/main.rs

fn part_1(input: &str) -> usize {
    let mut hm = HashSet::new();
    let mut l = 0;

    let mut sub = 0;
    for line in input.lines() {
        l += 1;
        let (a, x) = line.split_once(",").unwrap();
        let (b, c) = x.split_once(",").unwrap();
        let (x, y, z): (i64, i64, i64) = (a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap());
        hm.insert((x, y, z));
        if hm.get(&(x + 1, y, z)).is_some() {
            sub += 2;
        }
        if hm.get(&(x - 1, y, z)).is_some() {
            sub += 2;
        } 
        if hm.get(&(x, y + 1, z)).is_some() {
            sub += 2;
        }
        if hm.get(&(x, y - 1, z)).is_some() {
            sub += 2;
        }
        if hm.get(&(x, y, z - 1)).is_some() {
            sub += 2;
        }
        if hm.get(&(x, y, z + 1)).is_some() {
            sub += 2;
        }
        
    }
    l * 6 - sub
}

fn helper(hm: &HashSet<(i64, i64, i64)>, seen: &mut HashSet<(i64, i64, i64)>, x: i64, y: i64, z: i64) -> usize {
    if x < -1 || y < -1 || z < -1 {
        return 0;
    }
    if x > 21 || y > 21 || z > 21 {
        return 0;
    }
    if seen.contains(&(x, y, z)) {
        return 0;
    }
    seen.insert((x, y, z));

    let mut ans = 0;

    for coord in [(x + 1, y, z), (x - 1, y, z), (x, y + 1, z), (x, y - 1, z), (x, y, z - 1), (x, y, z + 1)] {
        if hm.get(&coord).is_none() {
            ans += helper(hm, seen, coord.0, coord.1, coord.2);
        }
        else {
            ans += 1;
        } 
    }

    ans


}

fn floodfill(hm: &mut HashSet<(i64, i64, i64)>) -> usize {
    let mut seen = HashSet::new();
    helper(hm, &mut seen, 0, 0, 0)

}

fn part_2(input: &str) -> usize {
    let mut hm = HashSet::new();

    for line in input.lines() {
        let (a, x) = line.split_once(",").unwrap();
        let (b, c) = x.split_once(",").unwrap();
        let (x, y, z): (i64, i64, i64) = (a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap());
        hm.insert((x, y, z));

    }
    floodfill(&mut hm)
}

fn main() {
    
    let input = fs::read_to_string("src/day18.txt").unwrap();

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 1 result: {}", result);

    let start_time = std::time::Instant::now();
    let result = part_2(&input);
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 2 result: {}", result);
}