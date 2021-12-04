fn main() {
    let start = std::time::Instant::now();
    
    let mut input = include_str!("input")
        .lines()
        .flat_map(|line| match line.trim() {
            s if s.is_empty() => None,
            s => Some(s),
        });

    let mut ns = input.next().unwrap().split(",").map(|s| s.parse::<u8>().unwrap());

    let mut cards = Vec::new();

    'outer: loop {
        let mut card = [0; 25];

        for i in 0..5 {
            let line = input.next();
            match line {
                Some(l) => for (j, n) in l.split_whitespace().map(|s| s.parse::<u8>().unwrap()).enumerate() {
                    card[i * 5 + j] = n;
                },
                None => break 'outer,
            }
        }

        cards.push(card);
    }

    let mut called = [false; 100];

    for _ in 0..5 {
        called[ns.next().unwrap() as usize] = true;
    }

    let mut score = None;
    let mut score2 = None;

    let mut won = vec![false; cards.len()];
    let mut remaining = cards.len();

    for last in ns {
        called[last as usize] = true;
        for (i, card) in cards.iter().enumerate() {
            if !won[i as usize] {
                if let Some(sum) = check_card(card, &called) {
                    won[i as usize] = true;
                    remaining -= 1;
                    if score.is_none() {
                        score = Some(sum * last as u64);
                    }
                    if remaining == 0 && score2.is_none() {
                        score2 = Some(sum * last as u64);
                    }
                }
            }
        }
    }

    let end = std::time::Instant::now();
    let delta = end - start;
    println!("{} us", delta.as_micros());

    println!("Part 1: {}", score.unwrap());
    println!("Part 2: {}", score2.unwrap());
}

fn check_card(card: &[u8; 25], called: &[bool; 100]) -> Option<u64> {
    let mut won = false;

    'outer: for i in 0..5 {
        for j in 0..5 {
            if !called[card[i * 5 + j] as usize] {
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
            if !called[card[i * 5 + j] as usize] {
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

fn uncalled_sum(card: &[u8; 25], called: &[bool; 100]) -> u64 {
    card.iter().fold(0, |acc, &n| if called[n as usize] {
        acc
    } else {
        acc + n as u64
    })
}
