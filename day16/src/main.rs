use std::{fs, collections::{HashMap, VecDeque, HashSet, BTreeSet}};

// https://github.com/LinAGKar/advent-of-code-2023-rust/blob/master/day22/src/main.rs

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Valve<'a> {
    name: &'a str,
    fv: usize,
    edges: Vec<&'a str>,
    idx: usize,
}

fn parse(input: &str) -> HashMap<&str, Valve> {
    input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let (name, rest) = line[6..].split_once(" has flow rate=").unwrap();
            let (flowval, tunnels) = rest.split_once("; tunnel").unwrap();
            let mut ws = 16;
            if tunnels.chars().nth(0) == Some('s') {
                ws = 17;
            }
            (name, Valve {
                name,
                fv: flowval.parse().unwrap(),
                edges: tunnels[ws..].split(", ").collect(),
                idx: idx,
            })
        })
        .collect()
}

fn part_1(input: &str) -> usize {

    let vv = parse(input);
    let mut imp = vv.values().filter(|v| v.fv > 0).collect::<Vec<_>>();
    let start = vv.get("AA").unwrap();
    imp.push(start);

    let mut dists: HashMap<(&str, &str), usize> = HashMap::new();

    for iv in imp {
        dists.insert((iv.name, iv.name), 0);
        let mut q = VecDeque::new();
        q.push_back((iv, 0));
        let mut seen = HashSet::new();

        while let Some((v, dist)) = q.pop_front() {
            if seen.contains(&v.name) {
                continue;
            }
            seen.insert(v.name);

            dists.insert((iv.name, v.name), dist); 
            for e in v.edges.iter() {
                // println!("{}", e);
                q.push_back((vv.get(e).unwrap(), dist + 1));
            }
        }
        // println!("{:?}", dists);
    }

    let mut deque = VecDeque::new();
    let s = (vec![vv.get("AA").unwrap()], 0, 0);

    deque.push_back(s);

    let mut ans = 0;

    const M : usize = 30;

    // let mut dp = HashMap::new();

    while let Some((path, t, ps)) = deque.pop_front() {
        // println!("{:?}\n\n", deque);

        // let mut config = BTreeSet::new();
        // for i in 1..path.len() {
        //     if path[i].name == path[i-1].name {
        //         config.insert(path[i].name);
        //     }
        // }
        // if dp.contains_key(&(config.clone(), t)) {
        //     if *dp.get(&(config.clone(), t)).unwrap() > ps {
        //         continue;
        //     }
        // }
        // dp.insert((config, t), ps);
        if ps > ans {
            ans = ps;
            println!("{:?} {} {:?}", path, t, ans)
        }
        if t == M {
            if ps > ans {
                ans = ps;
                println!("{:?} {} {:?}", path, t, ans);

            }
            // ans = ans.max(ps);
        } else {
            let last = path.last().unwrap();
            if path.iter().nth_back(1) != path.iter().nth_back(0) && last.fv > 0 {
                let mut new_path = path.clone();
                new_path.push(last);
                deque.push_back((new_path, t + 1, ps + (M - t - 1) * last.fv));
            } else {
                // Only opening valves saves a TRUCKLOAD lot of time! Exponential! Not -not opening- valves - since this is covered in our distance matrix anyways.
                // 1. To optimise, you could convert to a bitmap approach instead of path.
                // 2. Could do Floyd-Warshall which is Vec<Vec<usize>>, dist[i][j], instead of always BFSing
                // 3. Strip prefix is probably more efficient. BFS or DFS probs both work lol.

                // strip_prefix
                // or
                // and_then
                // iter + enumerate + fold (can insert into a new hashmap with enumeration easily)

                // let init_mask: u64 = (1 << len) - 1; to get easy bitmap.
                // Can just do a bitmap even over trivial valves, don't even need to squish, then can just do a bitset depending on index (which is non trivial valves)
                // Floyd Warshall will take a bit longer to run though.
                for e in dists.iter().filter(|(k, _)| k.0 == last.name).map(|(k, v)| (k.1, v)).collect::<Vec<_>>() {
                    if path.iter().find(|v| v.name == e.0).is_some() || t + e.1 > M {
                        // println!("{:?} {} {:?}", path, t, e);
                        continue;
                    }
                    let mut new_path = path.clone();
                    new_path.push(vv.get(e.0).unwrap());
                    deque.push_back((new_path, t + e.1, ps));
                }
            }
        }
    }
    ans
}

fn part_2(input: &str) -> usize {
    0
}

fn main() {
    println!("Reading input...");
    let input = fs::read_to_string("src/day16.txt").unwrap();
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