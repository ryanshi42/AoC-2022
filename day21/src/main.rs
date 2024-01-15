use std::{io::Read, fs, collections::{HashSet, HashMap}, ops::Div};

// https://github.com/LinAGKar/advent-of-code-2023-rust/blob/master/day22/src/main.rs

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Value<'a> {
    Unfinished(&'a str, &'a str, &'a str),
    Finished(i64)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Monkey<'a> {
    name: &'a str,
    value: Value<'a>,

}

// PART 1: JUST USE RECURSION....
fn part_1(input: &str) -> i64 {
    let mut monkeys: HashMap<&str, Monkey> = HashMap::new();
    let mut um = vec![];
    for line in input.lines() {
        let (name, rest) = line.split_once(": ").unwrap();
        if rest.len() == 11 {
            monkeys.insert(name, Monkey {
                name: name,
                value: Value::Unfinished(&rest[0..4], &rest[5..6], &rest[7..11])
            });
            um.push(name);
        } else {
            monkeys.insert(name, Monkey {
                name: name,
                value: Value::Finished(rest.parse().unwrap())
            });
        }
    }
    loop {
        let mut temp = vec![];
        for unf in um.iter() {
            let monke = monkeys.get(unf).unwrap();
            let mut new_val = Value::Unfinished("x", "y", "z");
            if let Value::Unfinished(l, op, r) = monke.value {
                match (monkeys.get(l).unwrap().value, monkeys.get(r).unwrap().value) {
                    (Value::Finished(lv), Value::Finished(rv)) => {
                        new_val = Value::Finished(match op {
                            "+" => lv + rv,
                            "-" => lv - rv,
                            "*" => lv * rv,
                            "/" => lv / rv,
                            _ => unreachable!()
                        });
                    },
                    _ => temp.push(*unf),
                }
            } else {
                temp.push(*unf);
            }
            if let Value::Finished(x) = new_val {
                monkeys.get_mut(unf).unwrap().value = new_val;
                if *unf == "root" {
                    return x;
                }
            }
        }
        um = temp;
    }
    0
}

fn parse(input: &str) -> (HashMap<&str, Monkey>, Vec<&str>){
    let mut monkeys: HashMap<&str, Monkey> = HashMap::new();
    let mut um = vec![];
    for line in input.lines() {
        let (name, rest) = line.split_once(": ").unwrap();
        if rest.len() == 11 {
            monkeys.insert(name, Monkey {
                name: name,
                value: Value::Unfinished(&rest[0..4], if name != "root" { &rest[5..6] } else { "=" }, &rest[7..11])
            });
            um.push(name);
        } else {
            monkeys.insert(name, Monkey {
                name: name,
                value: Value::Finished(rest.parse().unwrap())
            });
        }
    }
    (monkeys, um)
}

fn helper<'a>(monkeys: &mut HashMap<&'a str, Monkey>, mut um: Vec<&'a str>, p: i64) -> bool {
    let monke = monkeys.get_mut("humn").unwrap();
    monke.value = Value::Finished(p);

    for _ in 0..1000000 {
        let mut temp = vec![];
        for unf in um.iter() {
            let monke = monkeys.get(unf).unwrap();
            let mut new_val = Value::Unfinished("x", "y", "z");
            if let Value::Unfinished(l, op, r) = monke.value {
                match (monkeys.get(l).unwrap().value, monkeys.get(r).unwrap().value) {
                    (Value::Finished(lv), Value::Finished(rv)) => {
                        new_val = Value::Finished(match op {
                            "+" => lv + rv,
                            "-" => lv - rv,
                            "*" => lv * rv,
                            "/" => lv.div(rv),
                            "=" => if lv == rv { 1 } else { 0 },
                            _ => unreachable!()
                        });
                    },
                    _ => temp.push(*unf),
                }
            } else {
                temp.push(*unf);
            }
            if let Value::Finished(x) = new_val {
                // println!("{}", x);
                monkeys.get_mut(unf).unwrap().value = new_val;
                if *unf == "root" {
                    if x == 1 {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
        }
        um = temp;
    }
    false
}

fn part_2(input: &str) -> i64 {
    let (mut monkeys, um) = parse(input);

    for i in 0..1000000000 {
        println!("{}", i);
        if helper(&mut monkeys, um.clone(), i) {
            return i;
        }
    }
    0
}

fn main() {
    let input = fs::read_to_string("src/day22.txt").unwrap();

    // Splitting on a closure!!!
    // let v: Vec<&str> = input
    //     .split(|c| c == 'R' || c == 'L')
    //     .collect();
    // println!("{:?}", v);

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 1 result: {}", result);

    let start_time = std::time::Instant::now();
    let result = part_2(&input);
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 2 result: {}", result);
}