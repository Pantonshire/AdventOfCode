fn main() {
    let input = include_str!("input")
        .lines()
        .flat_map(|line| match line.trim() {
            s if s.is_empty() => None,
            s => Some(s),
        })
        .map(parse_instr)
        .collect::<Vec<_>>();

    let mut h = 0;
    let mut d = 0;

    for i in input.iter() {
        match i {
            Instr::Forward(x) => h += x,
            Instr::Down(x) => d += x,
            Instr::Up(x) => d -= x,
        }
    }

    println!("Part 1: {}", h * d);

    let mut h = 0;
    let mut d = 0;
    let mut aim = 0;

    for i in input.iter() {
        match i {
            Instr::Forward(x) => { d += aim * x; h += x; },
            Instr::Down(x) => aim += x,
            Instr::Up(x) => aim -= x,
        }
    }

    println!("Part 2: {}", h * d);
}

enum Instr {
    Forward(i64),
    Down(i64),
    Up(i64),
}

fn parse_instr(s: &str) -> Instr {
    let mut s = s.split_whitespace();
    let p1 = s.next().unwrap();
    let i = s.next().unwrap().parse().unwrap();
    match p1 {
        "forward" => Instr::Forward(i),
        "up" => Instr::Up(i),
        "down" => Instr::Down(i),
        _ => panic!(),
    }
}
