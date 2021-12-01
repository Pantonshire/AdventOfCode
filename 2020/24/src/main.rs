use std::collections::HashSet;

fn main() {
    let content = include_str!("../input");

    let tiles = content.split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(read_tile)
        .collect::<Option<Vec<(i32,i32)>>>()
        .expect("Error reading tiles");

    let mut flipped: HashSet<(i32,i32)> = HashSet::new();
    
    for tile in tiles {
        if !flipped.remove(&tile) {
            flipped.insert(tile);
        }
    }

    println!("Part 1: {}", flipped.len());

    let mut min_x = *flipped.iter().map(|(x,_)| x).min().unwrap();
    let mut max_x = *flipped.iter().map(|(x,_)| x).max().unwrap();
    let mut min_y = *flipped.iter().map(|(_,y)| y).min().unwrap();
    let mut max_y = *flipped.iter().map(|(_,y)| y).max().unwrap(); 

    for _ in 0..100 {
        let mut new_flipped = HashSet::new();
        let mut new_min_x = 0;
        let mut new_max_x = 0;
        let mut new_min_y = 0;
        let mut new_max_y = 0;

        for x in min_x-1..max_x+2 {
            for y in min_y-1..max_y+2 {
                let n_flipped = neighbours((x,y)).into_iter()
                    .filter(|p| flipped.contains(p))
                    .count();

                let mut was_flipped = false;
                if n_flipped == 2 {
                    new_flipped.insert((x,y));
                    was_flipped = true;
                } else if flipped.contains(&(x,y)) && n_flipped == 1 {
                    new_flipped.insert((x,y));
                    was_flipped = true;
                }

                if was_flipped {
                    if x < new_min_x {
                        new_min_x = x;
                    } else if x > new_max_x {
                        new_max_x = x;
                    }
                    if y < new_min_y {
                        new_min_y = y;
                    } else if y > new_max_y {
                        new_max_y = y;
                    }
                }
            }
        }

        flipped = new_flipped;
        min_x = new_min_x;
        max_x = new_max_x;
        min_y = new_min_y;
        max_y = new_max_y;
    }

    println!("Part 2: {}", flipped.len());
}

fn neighbours((x,y): (i32,i32)) -> Vec<(i32,i32)> {
    return vec![(x-1,y), (x+1,y), (x,y-1), (x,y+1), (x-1,y-1), (x+1,y+1)];
}

fn read_tile(mut l: &str) -> Option<(i32,i32)> {
    let (mut x, mut y) = (0,0);
    while !l.is_empty() {
        match l.strip_prefix("w") {
            Some(s) => {
                x -= 1;
                l = s;
                continue;
            },
            _ => (),
        }
        match l.strip_prefix("e") {
            Some(s) => {
                x += 1;
                l = s;
                continue;
            },
            _ => (),
        }
        match l.strip_prefix("nw") {
            Some(s) => {
                y += 1;
                l = s;
                continue;
            },
            _ => (),
        }
        match l.strip_prefix("ne") {
            Some(s) => {
                x += 1;
                y += 1;
                l = s;
                continue;
            },
            _ => (),
        }
        match l.strip_prefix("sw") {
            Some(s) => {
                x -= 1;
                y -= 1;
                l = s;
                continue;
            },
            _ => (),
        }
        match l.strip_prefix("se") {
            Some(s) => {
                y -= 1;
                l = s;
                continue;
            },
            _ => (),
        }
        return None;
    }
    Some((x,y))
}
