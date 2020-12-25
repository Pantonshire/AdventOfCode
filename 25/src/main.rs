fn main() {
    const CARD_KEY: u64 = 335121;
    const DOOR_KEY: u64 = 363891;
    const DIVIDE_BY: u64 = 20201227;

    let mut i = 0;
    let mut x = 1;
    let card_loop;

    loop {
        i += 1;
        x = (x * 7) % DIVIDE_BY;
        if x == CARD_KEY {
            card_loop = i;
            break;
        }
    }

    let mut encryption_key = 1;

    for _ in 0..card_loop {
        encryption_key = (encryption_key * DOOR_KEY) % DIVIDE_BY;
    }

    println!("Part 1: {}", encryption_key);
}
