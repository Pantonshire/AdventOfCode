fn main() {
    let p1_start = 7u8;
    let p2_start = 5u8;
    // let p1_start = 4u8;
    // let p2_start = 8u8;

    let mut p1_pos = p1_start as u16;
    let mut p2_pos = p2_start as u16;
    let mut p1_score = 0u64;
    let mut p2_score = 0u64;

    let mut next_roll = 1;
    let mut num_rolls = 0u64;

    let mut p1_turn = true;

    while p1_score < 1000 && p2_score < 1000 {
        let (r1, r2, r3) = (roll(&mut next_roll), roll(&mut next_roll), roll(&mut next_roll));
        num_rolls += 3;
        let (pos, score) = match p1_turn {
            true => (&mut p1_pos, &mut p1_score),
            false => (&mut p2_pos, &mut p2_score),
        };
        *pos = ((*pos + r1 + r2 + r3 - 1) % 10) + 1;
        *score += *pos as u64;
        num_rolls += 3;
        p1_turn = !p1_turn;
    }

    println!("Part 1: {}", p1_score.min(p2_score) * num_rolls / 2);

    let (us1, us2) = play(p1_start, p2_start, 0, 0);

    println!("Part 2: {}", us1.max(us2));
}

fn play(p1_pos: u8, p2_pos: u8, p1_score: u8, p2_score: u8) -> (u64, u64) {
    const ROLLS: [(u8, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    if p1_score >= 21 {
        return (1, 0);
    } else if p2_score >= 21 {
        return (0, 1);
    }

    let (mut total_us1, mut total_us2) = (0, 0);

    for (roll, num_us) in ROLLS {
        let p1_pos = ((p1_pos + roll - 1) % 10) + 1;
        let p1_score = p1_score + p1_pos;

        let (us2, us1) = play(p2_pos, p1_pos, p2_score, p1_score);
        total_us1 += us1 * num_us;
        total_us2 += us2 * num_us;
    }

    (total_us1, total_us2)
}

fn roll(next: &mut u16) -> u16 {
    let roll = *next;
    *next = (*next % 100) + 1;
    roll
}
