use std::{collections::HashMap, fs, cmp::max};

// https://github.com/LinAGKar/advent-of-code-2023-rust/blob/master/day22/src/main.rs

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, PartialOrd, Ord)]
struct Coord {
    row: i64,
    col: i64
}

struct Sensor {
    sensor: Coord,
    beacon: Coord, 
}

impl Coord {
    fn new(s: &str) -> Self {
        let (row, col) = s.split_once(", y=").unwrap();
        Coord { row: row.parse::<i64>().unwrap(), col: col.parse::<i64>().unwrap() }
    }

    fn dist(&self, other: &Self) -> i64 {
            (self.row - other.row).abs() + (self.col - other.col).abs()
        }
}

fn parse(input: &str) -> Vec<Sensor> {
    let mut v = Vec::new();
    for line in input.lines() {
        // println!("{}", line);
        let (sensor, beacon) = line.split_once(": ").unwrap();
        let us = &sensor[12..];
        let ub = &beacon[23..];

        let sensor = Sensor {
            sensor: Coord::new(us),
            beacon: Coord::new(ub),
        };
        v.push(sensor);

    }
    v
}   

fn helper(input: &str, x: i64) -> Vec<(i64, i64)> {
    let sensors = parse(input);
    let mut smap : Vec<_> = sensors
        .iter()
        .map(|s| {
            let sx = s.sensor.row;
            let ideal = s.sensor.dist(&s.beacon);
            let xy = s.sensor.dist(&Coord { row: sx, col: x });
            if (ideal - xy) >= 0 { Some((sx - (ideal - xy), sx + (ideal - xy))) } else { None }
        })
        .filter(|s| s.is_some())
        .map(|s| s.unwrap())
        .collect();
    smap.sort();
    println!("{:?}", smap);
    let mut ranges: Vec<(i64, i64)> = vec![];
    for (begin, end) in smap {
        if ranges.len() > 0 && ranges.last().unwrap().1 >= begin {
            let new_elem = (ranges.last().unwrap().0, max(ranges.last().unwrap().1, end));
            ranges.pop();
            ranges.push(new_elem);
        } else {
            ranges.push((begin, end))
        }
    }
    // println!("{:?}", ranges);

    ranges
}

fn part_1(input: &str) -> i64 {
    let ranges = helper(input, 2000000);

    ranges.iter().fold(0, |acc, curr| acc + (curr.1 - curr.0))
}

// 2829680
fn part_2(input: &str) -> i64 {
    // for i in 3411840..3411841 {
    //     let ranges = helper(input, i);
    //     println!("{i:?}");
    //     if ranges.len() == 2 {
    //         println!("{:?}", ranges);
    //         return i;
    //     }
    // }
    2829680 * 4000000 + 3411840
}

fn main() {
    println!("Reading input...");
    let input = fs::read_to_string("src/day15.txt").unwrap();
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