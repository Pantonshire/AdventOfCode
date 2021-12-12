use std::collections::{hash_map::Entry, HashMap};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Cave {
    Start,
    End,
    Big(u8),
    Small(u8),
}

fn main() {
    let input = include_str!("input")
        .lines()
        .flat_map(|line| match line.trim() {
            s if s.is_empty() => None,
            s => Some(s),
        });
    
    let mut graph = HashMap::new();

    let (mut big_ids, mut small_ids) = Default::default();
    let (mut big_id_next, mut small_id_next) = Default::default();

    for line in input {
        let (from, to) = line.split_once('-').unwrap();

        let from = parse_cave(from, &mut big_ids, &mut small_ids, &mut big_id_next, &mut small_id_next);
        let to = parse_cave(to, &mut big_ids, &mut small_ids, &mut big_id_next, &mut small_id_next);

        match graph.entry(from) {
            Entry::Occupied(occupied) => occupied.into_mut(),
            Entry::Vacant(vacant) => vacant.insert(Vec::new()),
        }.push(to);

        match graph.entry(to) {
            Entry::Occupied(occupied) => occupied.into_mut(),
            Entry::Vacant(vacant) => vacant.insert(Vec::new()),
        }.push(from);
    }

    let part_1 = traverse(&graph, 0, Cave::Start, false);
    println!("Part 1: {}", part_1);

    let part_2 = traverse(&graph, 0, Cave::Start, true);
    println!("Part 2: {}", part_2);
}

fn parse_cave<'a>(
    name: &'a str,
    big_ids: &mut HashMap<&'a str, u8>,
    small_ids: &mut HashMap<&'a str, u8>,
    big_id_next: &mut u8,
    small_id_next: &mut u8
) -> Cave
{
    match name {
        "start" => return Cave::Start,
        "end" => return Cave::End,
        _ => (),
    }

    let (small, ids, id_next) = if name.chars().next().map(|c| c.is_lowercase()).unwrap_or(false) {
        (true, small_ids, small_id_next)
    } else {
        (false, big_ids, big_id_next)
    };

    let id = match ids.get(name) {
        Some(id) => *id,
        None => {
            let prev_id = *id_next;
            ids.insert(name, prev_id);
            *id_next += 1;
            prev_id
        },
    };

    if small {
        Cave::Small(id)
    } else {
        Cave::Big(id)
    }
}

fn traverse(
    graph: &HashMap<Cave, Vec<Cave>>,
    mut blocklist: u64,
    cave_from: Cave,
    twice: bool
) -> u64
{
    match cave_from {
        Cave::End => return 1,
        Cave::Small(id) => blocklist |= 1u64 << id,
        _ => (),
    }

    let mut num_paths = 0;

    let caves_to = match graph.get(&cave_from) {
        None => return 0,
        Some(caves_to) => caves_to,
    };

    for cave_to in caves_to.iter().copied() {
        let (accept, twice) = match cave_to {
            Cave::Start => (false, false),
            Cave::Big(_) | Cave::End => (true, twice),
            Cave::Small(id) => {
                if (blocklist >> id) & 1u64 == 1 {
                    (twice, false)
                } else {
                    (true, twice)
                }
            },
        };

        if accept {
            num_paths += traverse(graph, blocklist, cave_to, twice);
        }
    }

    num_paths
}
