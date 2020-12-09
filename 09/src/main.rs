fn main() {
    let contents = include_str!("../input");
    let ns: Vec<u64> = contents.split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().expect("Error parsing number in input"))
        .collect();
    
    let invalid = find_invalid(&ns).expect("No invalid values found");
    println!("Part 1: {}", invalid);

    let weakness = find_weakness(&ns, invalid).expect("No weakness found");
    println!("Part 2: {}", weakness);
}

fn find_invalid(ns: &Vec<u64>) -> Option<u64> {
    let l = ns.len();
    'search: for it in 25..l {
        let t = ns[it];
        for i0 in it-24..it {
            for i1 in it-25..i0 {
                if ns[i0] + ns[i1] == t {
                    continue 'search;
                }
            }
        }
        return Some(t);
    }
    None
}

fn find_weakness(ns: &Vec<u64>, t: u64) -> Option<u64> {
    let l = ns.len();
    'search: for i0 in 0..l-1 {
        let mut sum = ns[i0];
        for i1 in i0+1..l {
            sum += ns[i1];
            if sum > t {
                continue 'search;
            } else if sum == t {
                let run = &ns[i0..i1+1];
                let min = run.iter().min().unwrap();
                let max = run.iter().max().unwrap();
                return Some(min + max);
            }
        }
    }
    None
}
