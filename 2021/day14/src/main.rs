use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() {
    let (template, rules) = include_str!("input")
        .split_once("\n\n")
        .unwrap();
    
    let template = template.trim().as_bytes().iter().copied().collect::<Vec<_>>();

    let rules = rules.lines().filter_map(|rule| if rule.is_empty() {
        None
    } else {
        Some(rule.split_once(" -> ").unwrap())
    })
    .map(|(pair, replacement)| {
        let pair_bytes = pair.as_bytes();
        (pair_bytes[0], pair_bytes[1], replacement.as_bytes()[0])
    })
    .collect::<Vec<_>>();

    let mut pairs = HashMap::new();

    for window in template.windows(2) {
        *match pairs.entry((window[0], window[1])) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(0u64),
        } += 1;
    }

    for (i, n) in [10, 30].into_iter().enumerate() {
        for _ in 0..n {
            let mut new_pairs = pairs.clone();

            for (r1, r2, rr) in rules.iter().copied() {
                let count = pairs.get(&(r1, r2)).copied().unwrap_or(0u64);
                if count > 0 {
                    *match new_pairs.entry((r1, r2)) {
                        Entry::Occupied(o) => o.into_mut(),
                        Entry::Vacant(v) => v.insert(0u64),
                    } -= count;

                    *match new_pairs.entry((r1, rr)) {
                        Entry::Occupied(o) => o.into_mut(),
                        Entry::Vacant(v) => v.insert(0u64),
                    } += count;

                    *match new_pairs.entry((rr, r2)) {
                        Entry::Occupied(o) => o.into_mut(),
                        Entry::Vacant(v) => v.insert(0u64),
                    } += count;
                }
            }

            pairs = new_pairs;
        }

        let mut counts = HashMap::new();

        for ((p1, _), count) in pairs.iter() {
            *match counts.entry(p1) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(0u64),
            } += count;
        }
        
        *match counts.entry(template.last().unwrap()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(0u64),
        } += 1;

        let max = *counts.values().max().unwrap();
        let min = *counts.values().min().unwrap();

        println!("Part {}: {}", i + 1, max - min);
    }
}
