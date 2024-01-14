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
    if x > 20 || y > 20 || z > 20 {
        return 0;
    }
    if seen.contains(&(x, y, z)) {
        return 0;
    }
    seen.insert((x, y, z));

    let mut ans = 0;

    if hm.get(&(x + 1, y, z)).is_none() {
        ans += helper(hm, seen, x + 1, y, z);
    }
    else {
        ans += 1;
    }
    if hm.get(&(x - 1, y, z)).is_none() {
        ans += helper(hm, seen, x - 1, y, z);
    } 
    else {
        ans += 1;
    }
    if hm.get(&(x, y + 1, z)).is_none() {
        ans += helper(hm, seen, x, y + 1, z);
    }
    else {
        ans += 1;
    }
    if hm.get(&(x, y - 1, z)).is_none() {
        ans += helper(hm, seen, x, y - 1, z);
    }
    else {
        ans += 1;
    }
    if hm.get(&(x, y, z - 1)).is_none() {
        ans += helper(hm, seen, x, y, z - 1);
    }
    else {
        ans += 1;
    }
    if hm.get(&(x, y, z + 1)).is_none() {
        ans += helper(hm, seen, x, y, z + 1);
    }
    else {
        ans += 1;
    }
    ans


}

fn floodfill(hm: &mut HashSet<(i64, i64, i64)>) -> usize {
    let mut seen = HashSet::new();
    helper(hm, &mut seen, 0, 0, 0)

}

fn part_2(input: &str) -> usize {
    let mut hm = HashSet::new();
    let mut mx = 999999;
    let mut my = 999999;
    let mut mz = 999999;
    for line in input.lines() {
        let (a, x) = line.split_once(",").unwrap();
        let (b, c) = x.split_once(",").unwrap();
        let (x, y, z): (i64, i64, i64) = (a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap());
        hm.insert((x, y, z));
        mx = mx.min(x);
        my = my.min(y);
        mz = mz.min(z); 
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