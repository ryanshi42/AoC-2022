use std::{io::Read, collections::{HashMap, HashSet}, fs};

// https://github.com/LinAGKar/advent-of-code-2023-rust/blob/master/day22/src/main.rs

// day 25
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

// day 10
    // let mut v: Vec<_> = input.lines().map(|x| x.parse::<u128>().unwrap()).collect();
    // v.sort();
    // let mut ones = 1;
    // let mut threes = 1;
    // for i in 1..v.len() {
    //     if v[i] - v[i - 1] == 1 {
    //         ones += 1;
    //     } else if v[i] - v[i - 1] == 3 {
    //         threes += 1;
    //     } else {
    //         panic!("at the disco");
    //     } 
    // }
    // ones * threes

// day 10 part 2
    // let mut v: Vec<_> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();
    // v.sort();

    // let mut dp = vec![0 as usize; *v.last().unwrap() + 1];
    // dp[0] = 1;
    
    // for i in v.clone().into_iter() {
    //     dp[i] += dp[i - 1];
    //     if i >= 2 {
    //         dp[i] += dp[i - 2];
    //     }
    //     if i >= 3 {
    //         dp[i] += dp[i - 3];
    //     }
    // }
    // println!("{:?}", dp);
    // dp[*v.last().unwrap()]

fn part_1(input: &str) -> usize { 
    // let allergens = HashSet::from_iter(["shellfish", "dairy", "soy", "nuts", "fish", "sesame", "wheat", "eggs", "sesame", "peanuts"]);
    let mut allergens = HashSet::new();
    let mut ingreds = HashSet::new();
    let mut poss: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut hm = HashMap::new();
    for line in input.lines() {
        // println!("{:?}", line);
        let (imp, rest) = line.split_once(" (contains ").unwrap();
        let here = rest[..rest.len()-1].split(", ").collect::<Vec<&str>>();
        for allergen in here {
            allergens.insert(allergen);
        }

        let ingred = imp.split(" ").collect::<Vec<&str>>();
        for i in ingred {
            ingreds.insert(i);

            *hm.entry(i).or_insert(0) += 1;
        }
    }

    for a in allergens.iter() {
        poss.insert(a, ingreds.clone());
    }

    for line in input.lines() {
        let (imp, rest) = line.split_once(" (contains ").unwrap();
        let here = rest[..rest.len()-1].split(", ").collect::<Vec<&str>>();

        let ingred = imp.split(" ").collect::<Vec<&str>>();
        let l = HashSet::from_iter(ingred);
        let not_ingred = ingreds.difference(&l);
        for allergen in here {
            for i in not_ingred.clone() {
                poss.get_mut(&allergen).unwrap().remove(i);
            }
        }

    }

    println!("{:?}", poss);

    ingreds
        .iter()
        .filter(|i| {
            for a in poss.values() {
                if a.contains(**i) {
                    return false;
                }
            }
            true
        })
        .map(|x| hm.get(x).unwrap())
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut allergens = HashSet::new();
    let mut ingreds = HashSet::new();
    let mut poss: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut hm = HashMap::new();
    for line in input.lines() {
        // println!("{:?}", line);
        let (imp, rest) = line.split_once(" (contains ").unwrap();
        let here = rest[..rest.len()-1].split(", ").collect::<Vec<&str>>();
        for allergen in here {
            allergens.insert(allergen);
        }

        let ingred = imp.split(" ").collect::<Vec<&str>>();
        for i in ingred {
            ingreds.insert(i);

            *hm.entry(i).or_insert(0) += 1;
        }
    }

    for a in allergens.iter() {
        poss.insert(a, ingreds.clone());
    }

    for line in input.lines() {
        let (imp, rest) = line.split_once(" (contains ").unwrap();
        let here = rest[..rest.len()-1].split(", ").collect::<Vec<&str>>();

        let ingred = imp.split(" ").collect::<Vec<&str>>();
        let l = HashSet::from_iter(ingred);
        let not_ingred = ingreds.difference(&l);
        for allergen in here {
            for i in not_ingred.clone() {
                poss.get_mut(&allergen).unwrap().remove(i);
            }
        }
    }

    let mut mapping = HashMap::new();


    for _ in 0..8 {
        let mut who = "allergens.iter().next().unwrap();";
        for (name, which) in poss.clone().iter() {
            if which.len() == 1 {
                who = which.iter().next().unwrap();
                mapping.insert(*name, who);
            }
        }
        for name in poss.clone().keys().clone() {
            poss.get_mut(name).unwrap().remove(who);
        }
    }

    println!("{:?}", mapping);
    let mut ans: Vec<_> = mapping.into_iter().collect(); 
    ans.sort_by(|a, b| a.0.cmp(b.0));
    
    println!("{:?}", ans.iter().map(|x| x.1).collect::<Vec<_>>().join(","));
    0
}

fn main() {
    
    let input = fs::read_to_string("src/day21.txt").unwrap();

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 1 result: {}", result);

    let start_time = std::time::Instant::now();
    let result = part_2(&input);
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 2 result: {}", result);
}