use num::integer;

fn main() {
    let contents = include_str!("../input");

    let mut lines = contents.split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty());

    let earliest = lines.next()
        .expect("First line not present")
        .parse::<u32>()
        .expect("Failed to parse first line");

    let buses = lines.next()
        .expect("Second line not present")
        .split(",")
        .enumerate()
        .filter(|&(_,t)| !t.eq("x"))
        .map(|(i,t)| (i,t.parse::<u32>().expect("Error parsing bus departure time")))
        .collect::<Vec<(usize,u32)>>();

    let (earliest_bus_id, earliest_waiting_time) = buses.iter()
        .map(|&(_,t)| (t, waiting_time(earliest, t)))
        .min_by(|(_,wt0), (_,wt1)| wt0.cmp(wt1))
        .expect("No minimum waiting time");

    let part_1 = earliest_bus_id * earliest_waiting_time;
    println!("Part 1: {}", part_1);

    let part_2 = solve(&make_system(&buses));
    println!("Part 2: {}", part_2);
}

fn waiting_time(earliest: u32, bus_id: u32) -> u32 {
    bus_id - (earliest % bus_id)
}

fn make_system(buses: &Vec<(usize,u32)>) -> Vec<(u64,u64)> {
    buses.iter()
        .map(|&(index,id)| (id as u64, umod(id as i64 - index as i64, id as i64)))
        .collect()
}

fn solve(system: &[(u64,u64)]) -> u64 {
    let mut x = 0;
    for i in 0..system.len() {
        let step = lcm((&system[..i]).iter().map(|&(base,_)| base));
        x = find_system_solution(x, step, &system[..i+1]);
    }
    x
}

fn find_system_solution(start: u64, step: u64, system: &[(u64,u64)]) -> u64 {
    let mut x = start;
    while system.iter().any(|&(base,equals)| x % base != equals) {
        x += step;
    }
    x
}

fn umod(x: i64, base: i64) -> u64 {
    (((x % base) + base) % base) as u64
}

fn lcm<I>(mut ns: I) -> u64
where
    I: Iterator<Item=u64>
{
    let mut l = match ns.next() {
        Some(n) => n,
        None => return 1,
    };
    for n in ns {
        l = integer::lcm(n,l);
    }
    l
}
