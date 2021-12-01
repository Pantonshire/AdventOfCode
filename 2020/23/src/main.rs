fn main() {
    const INPUT: &'static str = "219347865";
    
    let cups = INPUT.chars()
        .map(|x| x.to_digit(10).map(|x| x as usize))
        .collect::<Option<Vec<usize>>>()
        .expect("Error reading cups");

    let min_cup = *cups.iter().min().unwrap();
    let max_cup_1 = *cups.iter().max().unwrap();
    let max_cup_2 = 1_000_000;

    let mut nexts_1 = vec![0; max_cup_1 + min_cup];
    let mut nexts_2 = vec![0; max_cup_2 + min_cup];

    for (i,&x) in cups.iter().enumerate() {
        nexts_1[x] = cups[(i+1) % cups.len()];
        nexts_2[x] = cups[(i+1) % cups.len()];
    }

    nexts_2[*cups.last().unwrap()] = max_cup_1 + 1;

    for x in (max_cup_1 + 1)..max_cup_2 {
        nexts_2[x] = x + 1;
    }

    nexts_2[max_cup_2] = cups[0];

    let final_1 = play(cups[0], &nexts_1, min_cup, max_cup_1, 100);
    print!("Part 1: ");
    let mut part_1_cup = final_1[1];
    while part_1_cup != 1 {
        print!("{}", part_1_cup);
        part_1_cup = final_1[part_1_cup];
    }
    println!();

    let final_2 = play(cups[0], &nexts_2, min_cup, max_cup_2, 10_000_000);

    let part_2_cup_1 = final_2[1];
    let part_2_cup_2 = final_2[part_2_cup_1];
    println!("Part 2: {}", part_2_cup_1 as u64 * part_2_cup_2 as u64);
}

fn play(mut current: usize, nexts: &[usize], min: usize, max: usize, moves: usize) -> Vec<usize> {
    let mut nexts = Vec::from(nexts);
    for _ in 0..moves {
        let r1 = nexts[current];
        let r2 = nexts[r1];
        let r3 = nexts[r2];
        nexts[current] = nexts[r3];
        let mut destination = current - 1;
        if destination < min {
            destination = max;
        }
        while destination == r1 || destination == r2 || destination == r3 {
            destination -= 1;
            if destination < min {
                destination = max;
            }
        }
        let link_to = nexts[destination];
        nexts[destination] = r1;
        nexts[r3] = link_to;
        current = nexts[current];
    }
    nexts
}
