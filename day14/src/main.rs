use std::{collections::HashMap, fs};

// https://github.com/LinAGKar/advent-of-code-2023-rust/blob/master/day22/src/main.rs

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Empty,
    Rock,
    Sand,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, PartialOrd, Ord)]
struct Coord {
    row: usize,
    col: usize
}

fn parse(input: &str) -> (HashMap<Coord, Tile>, usize) {
    let mut hm = HashMap::new();
    let mut lowest = 0;
    for line in input.lines() {
        println!("{}", line);
        let pts: Vec<Coord> = line
            .split(" -> ")
            .map(|s| s.split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>())
            .map(|points| Coord { row: points[0], col: points[1] })
            .collect();
        for i in 1..pts.len() {
            let mut prev = pts[i - 1];
            let mut curr = pts[i];

            let mut tmp = [prev, curr];
            tmp.sort_by(|a, b| a.cmp(b));
            [prev, curr] = tmp;
            

            for r in prev.row..=curr.row {
                for c in prev.col..=curr.col {
                    hm.insert(Coord { row: r, col: c }, Tile::Rock);
                    lowest = lowest.max(c);
                }
            }
        }
    }
    (hm, lowest + 2)
}

fn part_1(input: &str) -> usize {

    let mut ans = 0;
    let mut s = Coord { row: 500, col: 0 };
    let start = s;
    let (mut grid, lowest) = parse(input);
    println!("lowest: {}", lowest);

    for i in 0..1000 {
        grid.insert(Coord { row: i, col: lowest }, Tile::Rock);
    }
    // println!("{:?}", grid);
    while s.col < 500 {
        // println!("{:?}", s);
        let bottom = Coord { row: s.row, col: s.col + 1 };
        if grid.get(&bottom).is_some() {
            let left = Coord { row: s.row - 1, col: s.col + 1 };
            if grid.get(&left).is_some() {
                let right = Coord { row: s.row + 1, col: s.col + 1 };
                if grid.get(&right).is_some() {
                    if s == start {
                        return ans + 1;
                    }
                    grid.insert(s, Tile::Sand);
                    ans += 1;
                    s = Coord { row: 500, col: 0 };
                } else {
                    s = right;
                } 
            } else {
                s = left;
            }
        } else {
            s = bottom;
        }

        // s = Coord { row: 500, col: 0 };
        
    }
    ans + 1
}

fn part_2(_input: &str) -> usize {
    0
}

fn main() {
    println!("Reading input...");
    let input = fs::read_to_string("src/day14.txt").unwrap();
    println!("Finished reading input...");

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 1 result: {}", result);

    let start_time = std::time::Instant::now();
    let result = part_2(&input);
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 2 result: {}", result);
}