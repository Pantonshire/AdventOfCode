use std::cmp;

fn main() {
    let contents = include_str!("../input");
    let mut ns: Vec<u32> = contents.split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().expect("Error parsing number in input"))
        .collect();

    ns.sort();

    ns.insert(0, 0);
    ns.push(ns.last().unwrap() + 3);

    let (mut n1, mut n3) = (0, 0);
    for i in 1..ns.len() {
        let d = ns[i] - ns[i-1];
        if d == 1 {
            n1 += 1;
        } else if d == 3 {
            n3 += 1;
        }
    }

    println!("Part 1: {}", n1 * n3);

    let mut paths: u64 = 1;
    let mut last_i = 0;
    for i in 1..ns.len() {
        //All combinations must include all of the adapters that are 3 more than the next lowest one
        if ns[i] - ns[i-1] == 3 {
            paths *= count_paths_to(&ns[last_i..], ns[i]);
            last_i = i;
        }
    }

    println!("Part 2: {}", paths);
}

fn count_paths_to(ns: &[u32], target: u32) -> u64 {
    if ns.is_empty() {
        return 0;
    }
    let current = *ns.first().unwrap();
    if current == target {
        return 1;
    } else if current > target {
        return 0;
    }
    let mut paths = 0;
    for i in 1..cmp::min(4, ns.len()) {
        if ns[i] - current > 3 {
            break
        }
        paths += count_paths_to(&ns[i..], target);
    }
    paths
}
