
// https://github.com/LinAGKar/advent-of-code-2023-rust/blob/master/day22/src/main.rs

use std::{fs, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockType {
    Minus,
    Plus, 
    L,
    Eye,
    Square
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Block {
    block_type: BlockType,
    height: usize,
    width: (usize, usize)
}

impl BlockType {
    fn next_block(&self) -> BlockType {
        match self {
            BlockType::Minus => BlockType::Plus,
            BlockType::Plus => BlockType::Minus,
            BlockType::L => BlockType::Eye,
            BlockType::Eye => BlockType::Square,
            BlockType::Square => BlockType::Minus
        }
    }

    fn default_width(&self) -> (usize, usize) {
        match self {
            BlockType::Minus => (2, 6),
            BlockType::Plus => (2, 5),
            BlockType::L => (2, 5),
            BlockType::Eye => (3, 3),
            BlockType::Square => (3, 3)
        }
    }
}

impl Block {
    fn push_left(&mut self) {
        if self.width.0 > 0 {
            self.width.0 -= 1;
            self.width.1 -= 1;
        }
    }
    fn push_right(&mut self) {
        if self.width.1 < 6 {
            self.width.0 += 1;
            self.width.1 += 1;
        }
    }
    fn push_down(&mut self) {
        self.height -= 1;
    }
    fn would_collide(&self, heights: &Vec<usize>) -> bool {
        let (x, y) = self.width;
        match self.block_type {
            BlockType::Minus => heights[x] == self.height - 1 || heights[y] == self.height - 1 || heights[x + 1] == self.height - 1 || heights[x + 2] == self.height - 1,
            BlockType::Plus => heights[x] == self.height - 2 || heights[y] == self.height - 2 || heights[x + 1] == self.height - 1,
            BlockType::L => heights[x] == self.height - 1 || heights[y] == self.height - 1 || heights[x + 1] == self.height - 1,
            BlockType::Eye => heights[x] == self.height - 1,
            BlockType::Square => heights[x] == self.height - 1 || heights[y] == self.height - 1
        }
    }
}

fn part_1(input: &str) -> usize {
    let mut curr_block = Block { block_type: BlockType::Minus, height: 3, width: BlockType::Minus.default_width() };
    let mut heights = vec![[0; 7]];
    let mut i = 1;
    loop {
        let jet = input.chars().nth(i % input.len()).unwrap();
        if i == 4 {
            break;
        }
        match jet {
            '<' => curr_block.push_left(),
            '>' => curr_block.push_right(),
            _ => (),
        }
        if curr_block.would_collide(&heights) {
            println!("colliding");
            let (x, _) = curr_block.width;
            match curr_block.block_type {
                BlockType::Minus => {
                    heights[x] = curr_block.height;
                    heights[x + 1] = curr_block.height;
                    heights[x + 2] = curr_block.height;
                    heights[x + 3] = curr_block.height;
                },
                BlockType::Plus => {
                    heights[x] = curr_block.height + 1;
                    heights[x + 1] = curr_block.height + 2;
                    heights[x + 2] = curr_block.height + 1;
                },
                BlockType::L => {
                    heights[x] = curr_block.height;
                    heights[x + 1] = curr_block.height;
                    heights[x + 2] = curr_block.height + 2;
                }, 
                BlockType::Eye => heights[x] = curr_block.height + 3,
                BlockType::Square => {
                    heights[x] = curr_block.height + 1;
                    heights[x + 1] = curr_block.height + 1;
                }
                _ => ()
            }

            let next_block_type = curr_block.block_type.next_block();
            curr_block = Block { block_type: next_block_type.clone(), height: *heights.iter().max().unwrap() + 3, width: next_block_type.default_width() };
        } else {
            curr_block.push_down();

        }
        i += 1;
    }
    *heights.last().unwrap().iter().max().unwrap()
}

fn part_2(input: &str) -> usize {
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