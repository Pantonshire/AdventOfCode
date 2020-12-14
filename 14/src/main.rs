use std::collections::HashMap;

const DEFAULT_VAL_MASKS: (u64, u64) = (0xFFFFFFFFFFFFFFFF, 0);

enum Instr {
    Mask(Vec<BitMask>),
    Store(usize, u64),
}

enum BitMask {
    Zero,
    One,
    Floating,
}

fn main() {
    let contents = include_str!("../input");

    let instrs = contents.split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(read_instr)
        .collect::<Option<Vec<Instr>>>()
        .expect("Error reading instructions");

    let mut mem_1: HashMap<usize,u64> = HashMap::new();
    let mut masks_1 = DEFAULT_VAL_MASKS;

    let mut mem_2: HashMap<usize,u64> = HashMap::new();
    let mut mask_2 = Vec::new();

    for instr in instrs {
        match instr {
            Instr::Mask(mask) => {
                masks_1 = make_val_masks(&mask);
                mask_2 = mask;
            },
            Instr::Store(addr, val) => {
                mem_1.insert(addr, apply_val_masks(masks_1, val));
                for m_addr in apply_addr_mask(&mask_2, addr) {
                    mem_2.insert(m_addr, val);
                }
            },
        }
    }

    let sum_1 = mem_1.iter()
        .map(|(_,val)| val)
        .sum::<u64>();
    println!("Part 1: {}", sum_1);

    let sum_2 = mem_2.iter()
        .map(|(_,val)| val)
        .sum::<u64>();
    println!("Part 2: {}", sum_2);
}

fn read_instr(s: &str) -> Option<Instr> {
    let args = s.splitn(2, "=")
        .map(|x| x.trim())
        .collect::<Vec<&str>>();
    if args.len() != 2 {
        return None;
    }
    match args[0].strip_prefix("mem[")
        .and_then(|x| x.strip_suffix("]"))
        .and_then(|x| x.parse().ok()) {
        Some(addr) => {
            let val = args[1].parse().ok()?;
            return Some(Instr::Store(addr, val));
        },
        None => (),
    }
    if args[0].eq("mask") {
        let mask = read_mask(args[1])?;
        return Some(Instr::Mask(mask));
    }
    None
}

fn read_mask(s: &str) -> Option<Vec<BitMask>> {
    s.chars()
        .rev()
        .map(read_bit_mask)
        .collect()
}

fn read_bit_mask(c: char) -> Option<BitMask> {
    match c {
        '0' => Some(BitMask::Zero),
        '1' => Some(BitMask::One),
        'X' => Some(BitMask::Floating),
        _ => None,
    }
}

fn make_val_masks(mask: &[BitMask]) -> (u64,u64) {
    let (mut and_mask, mut or_mask) = DEFAULT_VAL_MASKS;
    for (i,m) in mask.iter().enumerate() {
        match m {
            BitMask::Zero => and_mask &= !(1 << i),
            BitMask::One => or_mask |= 1 << i,
            BitMask::Floating => (),
        }
    }
    (and_mask, or_mask)
}

fn apply_val_masks((and_mask, or_mask): (u64, u64), val: u64) -> u64 {
    (val & and_mask) | or_mask
}

fn apply_addr_mask(mask: &[BitMask], addr: usize) -> Vec<usize> {
    fn apply_addr_mask_acc(mask: &[BitMask], addr: usize, i: usize) -> Vec<usize> {
        if mask.is_empty() {
            return vec![addr];
        }
        let ms = apply_addr_mask_acc(&mask[1..], addr, i+1);
        match mask.first().unwrap() {
            BitMask::Zero => ms,
            BitMask::One => ms.iter()
                .map(|x| x | (1 << i))
                .collect(),
            BitMask::Floating => {
                let mut fms = ms.iter()
                    .map(|x| x | (1 << i))
                    .collect::<Vec<usize>>();
                fms.extend(ms.iter()
                    .map(|x| x & !(1 << i)));
                fms
            },
        }
    }
    apply_addr_mask_acc(mask, addr, 0)
}
