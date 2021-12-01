fn main() {
    let input = include_str!("input")
        .lines()
        .flat_map(|line| match line.trim() {
            s if s.is_empty() => None,
            s => Some(s),
        })
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut prev = None;
    let mut c = 0;
    for x in input.iter() {
        if let Some(prev) = prev {
            if prev < x {
                c += 1;
            }
        }
        prev = Some(x);
    }

    println!("Part 1: {}", c);

    let mut prev = None;
    let mut c = 0;

    let window_iter = input.iter()
        .zip(input.iter().skip(1))
        .zip(input.iter().skip(2));

    for ((x, y), z) in window_iter {
        let sum = x + y + z;
        if let Some(prev) = prev {
            if prev < sum {
                c += 1;
            }
        }
        prev = Some(sum);
    }

    println!("Part 2: {}", c);
}
