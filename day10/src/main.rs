use std::{io::Read, collections::{HashMap, HashSet}, fs};

// https://github.com/LinAGKar/advent-of-code-2023-rust/blob/master/day22/src/main.rs

    // // 16616892
    // // 14505727
    // // 16616892
    // // 14505727
    // let mut curr: u128 = 1;
    // let mut ans: u128 = 0;

    // for i in 1..1000000000 {
    //     // if (16616892 + i * 20201227) % 7 == 0 {
    //     //     println!("a: {:?}", (16616892 + i * 20201227) / 7);
    //     // }
    //     // if (14505727 + i * 20201227) % 7 == 0 {
    //     //     println!("b: {:?}", (14505727 + i * 20201227) / 7);
    //     // } 
    //     curr = (curr * 7) % 20201227;
    //     if curr == 16616892 {
    //         println!("a: {:?}", i);
    //         ans = i;
    //         break;
    //     }
    // }

    // curr = 1;
    // for _ in 0..ans {
    //     curr = (curr * 14505727) % 20201227;
    // }
    // curr

fn part_1(input: &str) -> u128 { 
    let mut v: Vec<_> = input.lines().map(|x| x.parse::<u128>().unwrap()).collect();
    v.sort();
    let mut ones = 1;
    let mut threes = 1;
    for i in 1..v.len() {
        if v[i] - v[i - 1] == 1 {
            ones += 1;
        } else if v[i] - v[i - 1] == 3 {
            threes += 1;
        } else {
            panic!("at the disco");
        } 
    }
    ones * threes
}

fn part_2(input: &str) -> usize {
    let mut v: Vec<_> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();
    v.sort();

    let mut dp = vec![0 as usize; *v.last().unwrap() + 1];
    dp[0] = 1;
    
    for i in v.clone().into_iter() {
        dp[i] += dp[i - 1];
        if i >= 2 {
            dp[i] += dp[i - 2];
        }
        if i >= 3 {
            dp[i] += dp[i - 3];
        }
    }
    println!("{:?}", dp);
    dp[*v.last().unwrap()]
}

fn main() {
    
    let input = fs::read_to_string("src/day10.txt").unwrap();

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 1 result: {}", result);

    let start_time = std::time::Instant::now();
    let result = part_2(&input);
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 2 result: {}", result);
}