fn main() {
    let input = include_str!("input")
        .lines()
        .flat_map(|line| match line.trim() {
            s if s.is_empty() => None,
            s => Some(s.split_once('|').unwrap()),
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", input.iter()
        .map(|(_, os)| {
            os.split_whitespace()
                .filter(|o| o.len() == 2 || o.len() == 3 || o.len() == 4 || o.len() == 7)
                .count()
        })
        .sum::<usize>());

    println!("Part 2: {}", input.iter().map(|(ss, os)| {
        let signals = ss.split_whitespace().map(|s| to_arr(s)).collect::<Vec<_>>();
        let sig_1 = signals.iter().copied().find(|s| s.iter().filter(|b| **b).count() == 2).unwrap();
        let sig_4 = signals.iter().copied().find(|s| s.iter().filter(|b| **b).count() == 4).unwrap();
        let sig_7 = signals.iter().copied().find(|s| s.iter().filter(|b| **b).count() == 3).unwrap();
        let sig_8 = signals.iter().copied().find(|s| s.iter().filter(|b| **b).count() == 7).unwrap();
        let sig_6 = signals.iter().copied().find(|s| s.iter().filter(|b| **b).count() == 6
            && s.iter().enumerate().any(|(i, b)| !*b && sig_7[i])).unwrap();
        let sig_0 = signals.iter().copied().find(|s| s.iter().filter(|b| **b).count() == 6
            && *s != sig_6
            && s.iter().enumerate().any(|(i, b)| !*b && sig_4[i])).unwrap();
        let sig_9 = signals.iter().copied().find(|s| s.iter().filter(|b| **b).count() == 6
            && *s != sig_0
            && *s != sig_6).unwrap();
        let sig_5 = signals.iter().copied().find(|s| s.iter().filter(|b| **b).count() == 5
            && s.iter().enumerate().any(|(i, b)| !*b && !sig_6[i])).unwrap();
        let sig_3 = signals.iter().copied().find(|s| s.iter().filter(|b| **b).count() == 5
            && *s != sig_5
            && s.iter().enumerate().any(|(i, b)| !*b && !sig_9[i])).unwrap();
        let sig_2 = signals.iter().copied().find(|s| s.iter().filter(|b| **b).count() == 5
            && *s != sig_5
            && *s != sig_3).unwrap();

        let mut os = os.split_whitespace().map(to_arr).map(|o| {
            if o == sig_0 {
                0
            } else if o == sig_1 {
                1
            } else if o == sig_2 {
                2
            } else if o == sig_3 {
                3
            } else if o == sig_4 {
                4
            } else if o == sig_5 {
                5
            } else if o == sig_6 {
                6
            } else if o == sig_7 {
                7
            } else if o == sig_8 {
                8
            } else if o == sig_9 {
                9
            } else {
                panic!()
            }
        });
        os.next().unwrap() * 1000 + os.next().unwrap() * 100 + os.next().unwrap() * 10 + os.next().unwrap()
    }).sum::<u64>());
}

fn to_arr(s: &str) -> [bool; 7] {
    let mut a = [false; 7];
    if s.contains('a') {
        a[0] = true;
    }
    if s.contains('b') {
        a[1] = true;
    }
    if s.contains('c') {
        a[2] = true;
    }
    if s.contains('d') {
        a[3] = true;
    }
    if s.contains('e') {
        a[4] = true;
    }
    if s.contains('f') {
        a[5] = true;
    }
    if s.contains('g') {
        a[6] = true;
    }
    a
}
