fn main() {
    let crabs = include_str!("input")
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    
    let positions = crabs.iter().copied().min().unwrap()..crabs.iter().copied().max().unwrap();

    let min_fuel_1 = positions.clone().map(|pos| {
        crabs.iter().copied().fold(0, |acc, crab| acc + (crab - pos).abs())
    }).min().unwrap();

    println!("Part 1: {}", min_fuel_1);

    let min_fuel_2 = positions.map(|pos| {
        crabs.iter().copied().fold(0, |acc, crab| {
            let diff = (crab - pos).abs();
            acc + (diff * (diff + 1) / 2)
        })
    }).min().unwrap();

    println!("Part 2: {}", min_fuel_2);
}
