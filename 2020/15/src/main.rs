use std::collections::HashMap;

fn main() {
    let ns: [u32;6] = [1,0,16,5,17,4];

    let mut spoken: HashMap<u32, (u32,u32)> = HashMap::new();

    let mut j: u32 = 0;

    for (i,&n) in ns.iter().enumerate() {
        spoken.insert(n, (i as u32, i as u32));
        j += 1
    }

    let mut prev = *ns.last().unwrap();

    while j < 2020 {
        let (n, n_spoken) = step(j, prev, &spoken);
        prev = n;
        spoken.insert(n, n_spoken);
        j += 1
    }

    println!("Part 1: {}", prev);

    while j < 30000000 {
        let (n, n_spoken) = step(j, prev, &spoken);
        prev = n;
        spoken.insert(n, n_spoken);
        j += 1
    }

    println!("Part 2: {}", prev);
}

fn step(i: u32, prev: u32, spoken: &HashMap<u32, (u32,u32)>) -> (u32, (u32, u32)) {
    let spoken_prev = spoken.get(&prev).unwrap();
    let n = spoken_prev.0 - spoken_prev.1;
    match spoken.get(&n) {
        Some(&(recent,_)) => (n, (i, recent)),
        None => (n, (i, i)),
    }
}
