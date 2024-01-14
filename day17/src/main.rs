
// https://github.com/LinAGKar/advent-of-code-2023-rust/blob/master/day22/src/main.rs

use std::{fs, collections::HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockType {
    Minus,
    Plus, 
    L,
    Eye,
    Square
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Block {
    block_type: BlockType,
    v: HashSet<(usize, usize)>,
}

impl BlockType {
    fn next_block(&self) -> BlockType {
        match self {
            BlockType::Minus => BlockType::Plus,
            BlockType::Plus => BlockType::L,
            BlockType::L => BlockType::Eye,
            BlockType::Eye => BlockType::Square,
            BlockType::Square => BlockType::Minus
        }
    }

    fn get_set(&self, h: usize) -> HashSet<(usize, usize)> {
        match self {
            BlockType::Minus => HashSet::from([(2, h), (3, h), (4, h), (5, h)]),
            BlockType::Plus => HashSet::from([(2, h + 1), (3, h), (3, h + 1), (3, h + 2), (4, h + 1)]),
            BlockType::L => HashSet::from([(2, h), (3, h), (4, h), (4, h + 1), (4, h + 2)]),
            BlockType::Eye => HashSet::from([(2, h), (2, h + 1), (2, h + 2), (2, h + 3)]),
            BlockType::Square => HashSet::from([(2, h), (3, h), (2, h + 1), (3, h + 1)])
        }
    }
}

impl Block {
    fn push_left(&mut self, heights: &HashSet<(usize, usize)>) {
        if self.v.iter().any(|(x, _)| *x == 0) {
            return;
        }
        self.v = self.v.iter().map(|(x, y)| (x - 1, *y)).collect();
        if !heights.is_disjoint(&self.v) {
            self.push_right(heights);
        }
    }
    fn push_right(&mut self, heights: &HashSet<(usize, usize)>) {
        if self.v.iter().any(|(x, _)| *x == 6) {
            return;
        }
        self.v = self.v.iter().map(|(x, y)| (x + 1, *y)).collect();
        if !heights.is_disjoint(&self.v) {
            self.push_left(heights);
        }
    }
    fn push_down(&mut self) {
        self.v = self.v.iter().map(|(x, y)| (*x, y - 1)).collect();
    }

    fn push_up(&mut self) {
        self.v = self.v.iter().map(|(x, y)| (*x, y + 1)).collect();
    }

    fn would_collide(&mut self, heights: &HashSet<(usize, usize)>) -> bool {
        !heights.is_disjoint(&self.v)
    }
}

// Cbs, use SET to make sure that you don't move into another block.
fn part_1(input: &str) -> usize {
    let mut curr_block = Block { block_type: BlockType::Minus, v: BlockType::Minus.get_set(4) };
    let mut heights = HashSet::new();
    heights.insert((0, 0));
    heights.insert((1, 0));
    heights.insert((2, 0));
    heights.insert((3, 0));
    heights.insert((4, 0));
    heights.insert((5, 0));
    heights.insert((6, 0));
    heights.insert((7, 0));
    let mut i = 1;
    let mut su = 1;
    loop {
        let jet = input.chars().nth((i - 1) % input.len()).unwrap();
        match jet {
            '<' => curr_block.push_left(&heights),
            '>' => curr_block.push_right(&heights),
            _ => (),
        }
        println!("{i} {:?} {}", curr_block.v, jet);

        // println!("v: {:?}", self.v);
        curr_block.push_down();
        if curr_block.would_collide(&heights) {
            curr_block.push_up();
            println!("colliding");
            heights.extend(curr_block.v);

            let next_block_type = curr_block.block_type.next_block();
            su += 1;
            if su == 2023 {
                return *heights.iter().map(|(_, y)| y).max().unwrap();
            }
            curr_block = Block { block_type: next_block_type.clone(), v: next_block_type.get_set(*heights.iter().map(|(_, y)| y).max().unwrap() + 4) };
        }
        i += 1;
    }
}

fn part_2(_input: &str) -> usize {
    0
}

fn main() {
    println!("Reading input...");
    let input = fs::read_to_string("src/day17.txt").unwrap();
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