use std::collections::HashSet;

fn main() {
    let (dots_in, folds_in) = include_str!("input")
        .split_once("\n\n")
        .unwrap();
    
    let mut dots = Vec::new();
    let mut folds = Vec::new();

    for line in dots_in.lines() {
        if line.is_empty() {
            break;
        }
        let (x, y) = line.split_once(',').unwrap();
        let x = x.parse::<i32>().unwrap();
        let y = y.parse::<i32>().unwrap();
        dots.push((x, y));
    }

    for fold in folds_in.lines() {
        if fold.is_empty() {
            break;
        }
        let fold = fold.strip_prefix("fold along ").unwrap();
        let (dir, n) = fold.split_once('=').unwrap();
        let is_y = dir == "y";
        let n = n.parse::<i32>().unwrap();
        folds.push((is_y, n));
    }

    let mut first = true;

    for (is_y, n) in folds {
        for (x, y) in &mut dots {
            if is_y {
                if *y > n {
                    *y = n - (*y - n);
                }
            } else {
                if *x > n {
                    *x = n - (*x - n);
                }
            }
        }
        if first {
            first = false;
            println!("Part 1: {}", dots.iter().collect::<HashSet<_>>().len());
        }
    }

    let max_x = dots.iter().map(|(x, _)| *x).max().unwrap();
    let max_y = dots.iter().map(|(_, y)| *y).max().unwrap();

    println!("Part 2:");
    for y in 0..=max_y {
        for x in 0..=max_x {
            if dots.contains(&(x, y)) {
                print!("\u{2b1b}");
            } else {
                print!("\u{2b1c}");
            }
        }
        println!();
    }
}
