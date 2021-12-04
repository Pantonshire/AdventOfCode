use std::collections::HashSet;

fn main() {
    let mut input = include_str!("input")
        .lines()
        .flat_map(|line| match line.trim() {
            s if s.is_empty() => None,
            s => Some(s),
        });

    let mut ns = input.next().unwrap().split(",").map(|s| s.parse::<u64>().unwrap());

    let mut cards = Vec::new();

    'outer: loop {
        let mut card: [u64; 25] = [0; 25];

        for i in 0..5 {
            let line = input.next();
            match line {
                Some(l) => for (j, n) in l.split_whitespace().map(|s| s.parse::<u64>().unwrap()).enumerate() {
                    card[i * 5 + j] = n;
                },
                None => break 'outer,
            }
        }

        cards.push(card);
    }

    let mut called = HashSet::new();

    for _ in 0..5 {
        called.insert(ns.next().unwrap());
    }

    let mut score = None;
    let mut score2 = None;

    let mut remaining = HashSet::new();
    for i in 0..cards.len() {
        remaining.insert(i);
    }

    for last in ns {
        called.insert(last);
        for (i, card) in cards.iter().enumerate() {
            if let Some(sum) = check_card(card, &called) {
                if score.is_none() {
                    score = Some(sum * last);
                }
                remaining.remove(&i);
                if remaining.is_empty() && score2.is_none() {
                    score2 = Some(sum * last);
                }
            }
        }
    }

    println!("Part 1: {}", score.unwrap());
    println!("Part 2: {}", score2.unwrap());
}

fn check_card(card: &[u64; 25], called: &HashSet<u64>) -> Option<u64> {
    let mut won = false;

    'outer: for i in 0..5 {
        for j in 0..5 {
            if !called.contains(&card[i * 5 + j]) {
                continue 'outer;
            }
        }
        won = true;
        break;
    }

    if won {
        return Some(uncalled_sum(card, called));
    }

    'outer2: for j in 0..5 {
        for i in 0..5 {
            if !called.contains(&card[i * 5 + j]) {
                continue 'outer2;
            }
        }
        won = true;
        break;
    }

    if won {
        return Some(uncalled_sum(card, called));
    }

    None
}

fn uncalled_sum(card: &[u64; 25], called: &HashSet<u64>) -> u64 {
    card.iter().fold(0, |acc, n| if called.contains(n) {
        acc
    } else {
        acc + n
    })
}
