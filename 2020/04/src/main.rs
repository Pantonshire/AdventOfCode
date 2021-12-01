use std::fs;

const BYR: &str = "byr";
const IYR: &str = "iyr";
const EYR: &str = "eyr";
const HGT: &str = "hgt";
const HCL: &str = "hcl";
const ECL: &str = "ecl";
const PID: &str = "pid";
const CID: &str = "cid";

type Passport<'a> = Vec<(&'a str, &'a str)>;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read file");

    let lines = contents.split("\n\n");

    let required_keys = [BYR, IYR, EYR, HGT, HCL, ECL, PID];

    let (mut valid1, mut valid2) = (0, 0);

    for line in lines.map(|l| l.trim()).filter(|&l| !l.is_empty()) {
        let passport = read_passport(line).expect("Error parsing passport");
        if contains_keys(&passport, &required_keys) {
            valid1 += 1;
            if vals_valid(&passport) {
                valid2 += 1;
            }
        }
    }

    println!("Part 1: {}", valid1);
    println!("Part 2: {}", valid2);
}

fn read_passport(data: &str) -> Option<Passport> {
    let mut passport = Vec::new();
    for item in data.trim().split_ascii_whitespace() {
        let mut pair = item.splitn(2, ':');
        let key = pair.next()?;
        let val = pair.next()?;
        passport.push((key, val));
    }
    Some(passport)
}

fn contains_keys(passport: &Passport, keys: &[&str]) -> bool {
    for key in keys.iter().cloned() {
        if !passport.iter().any(|&f| f.0.eq(key)) {
            return false;
        }
    }
    true
}

fn vals_valid(passport: &Passport) -> bool {
    passport.iter().all(|&x| val_valid(x))
}

fn val_valid(item: (&str, &str)) -> bool {
    match item {
        (BYR, val) => str_num_range(val, 1920, 2002),
        (IYR, val) => str_num_range(val, 2010, 2020),
        (EYR, val) => str_num_range(val, 2020, 2030),
        (HGT, val) => height_valid(val),
        (HCL, val) => hcl_valid(val),
        (ECL, val) => ["amb","blu","brn","gry","grn","hzl","oth"].iter().any(|&s| s.eq(val)),
        (PID, val) => pid_valid(val),
        (CID, _) => true,
        _ => false,
    }
}

fn str_num_range(s: &str, min: i32, max: i32) -> bool {
    match s.parse() {
        Ok(num) => min <= num && num <= max,
        Err(_) => false,
    }
}

fn height_valid(s: &str) -> bool {
    let num: &str;
    let min: i32;
    let max: i32;
    match s.strip_suffix("cm") {
        Some(n) => {
            num = n;
            min = 150;
            max = 193;
        },
        None => match s.strip_suffix("in") {
            Some(n) => {
                num = n;
                min = 59;
                max = 76;
            },
            None => return false,
        },
    }
    str_num_range(num, min, max)
}

fn hcl_valid(s: &str) -> bool {
    let mut cs = s.chars();
    if cs.next() != Some('#') {
        return false;
    }
    let mut digits = 0;
    for c in cs {
        if !c.is_digit(16) {
            return false;
        }
        digits += 1;
    }
    digits == 6
}

fn pid_valid(s: &str) -> bool {
    let mut digits = 0;
    for c in s.chars() {
        if !c.is_digit(10) {
            return false;
        }
        digits += 1;
    }
    digits == 9
}
