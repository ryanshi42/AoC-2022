use std::{collections::HashMap, fs};

// https://github.com/LinAGKar/advent-of-code-2023-rust/blob/master/day22/src/main.rs

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Element {
    Number(u8),
    Vector(Vec<Element>),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Order {
    IsOrder,
    NoComment,
    IsNotOrder,
}

fn parse(input: &str) -> Vec<(&str, &str)> {
    let mut v = Vec::new();
    for line in input.split("\n\n") {
        v.push(line.split_once("\n").unwrap());
    }
    v
}

// Make own evaluate function lmao
fn parse_str(input: &str) -> Element {
    if input.is_empty() {
        return Element::Vector(Vec::new());
    } else if input.chars().nth(0).unwrap() == '[' {
        let mut v = Vec::new();
        let mut j = 0;
        let temp = &input[1..input.len() - 1];
        let mut i = 0;
        while i < temp.len() {
            let c = temp.chars().nth(i).unwrap();
            if c == '[' && j == i {
                let mut open = 1; 
                let mut k = i + 1;
                while open != 0 {
                    let tc = temp.chars().nth(k).unwrap();
                    if tc == '[' {
                        open += 1;
                    } else if tc == ']' {
                        open -= 1;
                    }
                    k += 1;
                }
                i = k;
                continue;
            }
            if c == ',' {
                // println!("{:?}", &temp[j..i]);
                v.push(parse_str(&temp[j..i]));
                j = i + 1;
            }
            if c == ']' {
                break;
            }
            i += 1;
        }
        v.push(parse_str(&temp[j..i]));
        Element::Vector(v)
    } else {
        Element::Number(input.parse().unwrap())
    }

}

fn is_order(left: Element, right: Element) -> Order {
    // println!("{:?}", left);
    // println!("{:?}", right);
    match (left, right) {
        (Element::Number(l), Element::Number(r)) => {
            if l < r {
                return Order::IsOrder;
            } else if l > r {
                return Order::IsNotOrder;
            } else {
                return Order::NoComment;
            }
        }
        (Element::Vector(l), Element::Vector(r)) => {
            let mut tidx = 0;
            while tidx < l.len() && tidx < r.len() {
                let (tl, tr) = (l[tidx].clone(), r[tidx].clone());
                let res = is_order(tl, tr);
                if res != Order::NoComment {
                    return res;
                }
                tidx += 1;
            }
            if tidx == l.len() && tidx < r.len() {
                return Order::IsOrder;
            } else if tidx == r.len() && tidx < l.len() {
                return Order::IsNotOrder;
            } else {
                return Order::NoComment;
            }
        }
        (x, Element::Vector(v)) => {
            return is_order(Element::Vector(vec![x]), Element::Vector(v));
        },
        (Element::Vector(v), x) => {
            return is_order(Element::Vector(v), Element::Vector(vec![x]));
        },
    }
}

fn part_1(input: &str) -> usize {
    let v = parse(input);
    let mut ans = 0;

    for (idx, (left, right)) in v.iter().enumerate() {
        let ordered = is_order(parse_str(left), parse_str(right));
        if ordered == Order::IsOrder {
            // println!("{}", idx + 1);
            ans += idx + 1;
        }
    }

    ans
}

fn part_2(_input: &str) -> usize {
    0
}

fn main() {
    println!("Reading input...");
    let input = fs::read_to_string("src/day13.txt").unwrap();
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