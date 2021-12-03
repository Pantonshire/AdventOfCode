fn main() {
    let input = include_str!("input")
        .lines()
        .flat_map(|line| match line.trim() {
            s if s.is_empty() => None,
            s => Some(s),
        })
        .collect::<Vec<_>>();

    let mut gamma_buf = String::with_capacity(12);

    for i in 0..12 {
        let mut c1 = 0;
        let mut c0 = 0;

        for x in input.iter() {
            let c = x.chars().nth(i).unwrap();
            if c == '1' {
                c1 += 1;
            } else if c == '0' {
                c0 += 1;
            }
        }

        gamma_buf.push(if c1 >= c0 { '1' } else { '0' });
    }

    let gamma = i64::from_str_radix(&gamma_buf, 2).unwrap();
    let epsilon = 0b111111111111 & !gamma;

    println!("Part 1: {}", gamma * epsilon);

    let mut cand_1 = input.clone();
    let mut cand_2 = input.clone();

    for i in 0..12 {
        if cand_1.len() > 1 {
            let mut c1 = 0;
            let mut c0 = 0;

            for x in cand_1.iter() {
                let c = x.chars().nth(i).unwrap();
                if c == '1' {
                    c1 += 1;
                } else if c == '0' {
                    c0 += 1;
                }
            }

            if c1 >= c0 {
                cand_1.retain(|x| x.chars().nth(i).unwrap() == '1');
            } else {
                cand_1.retain(|x| x.chars().nth(i).unwrap() == '0');
            }
        }

        if cand_2.len() > 1 {
            let mut c1 = 0;
            let mut c0 = 0;

            for x in cand_2.iter() {
                let c = x.chars().nth(i).unwrap();
                if c == '1' {
                    c1 += 1;
                } else if c == '0' {
                    c0 += 1;
                }
            }

            if c1 >= c0 {
                cand_2.retain(|x| x.chars().nth(i).unwrap() == '0');
            } else {
                cand_2.retain(|x| x.chars().nth(i).unwrap() == '1');
            }
        }
    }

    let oxygen = i64::from_str_radix(cand_1[0], 2).unwrap();
    let co2 = i64::from_str_radix(cand_2[0], 2).unwrap();
    println!("Part 2: {}", oxygen * co2);
}
