use std::collections::HashSet;

fn main() {
    let lines = include_str!("input")
        .lines()
        .flat_map(|line| match line.trim() {
            s if s.is_empty() => None,
            s => Some(s),
        })
        .map(parse_line)
        .collect::<Vec<_>>();

    // let lines1 = input.iter()
    //     .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2)
    //     .copied();

    println!("Part 1: {}", overlapping(lines.iter().filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2).copied()));
    println!("Part 2: {}", overlapping(lines.into_iter()));
}

fn overlapping<I>(lines: I) -> usize
where
    I: Iterator<Item = (i32, i32, i32, i32)>,
{
    let mut points = HashSet::new();
    let mut overlapped = HashSet::new();
    let mut c = 0;

    for (x1, y1, x2, y2) in lines {
        let x_step = (x2 - x1).signum();
        let y_step = (y2 - y1).signum();
    
        for i in 0..=((x2 - x1) * x_step).max((y2 - y1) * y_step) {
            let x = x1 + (x_step * i);
            let y = y1 + (y_step * i);
            if points.contains(&(x, y)) && !overlapped.contains(&(x, y)) {
                c += 1;
                overlapped.insert((x, y));
            }
            points.insert((x, y));
        }
    }

    c
}

fn parse_line(s: &str) -> (i32, i32, i32, i32) {
    let (l, r) = s.split_once("->").unwrap();
    let (x1, y1) = l.split_once(',').unwrap();
    let (x2, y2) = r.split_once(',').unwrap();
    (x1.trim().parse().unwrap(), y1.trim().parse().unwrap(), x2.trim().parse().unwrap(), y2.trim().parse().unwrap())
}
