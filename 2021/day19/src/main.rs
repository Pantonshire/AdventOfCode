use std::fmt;
use std::ops;
use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord {
    const ROTATIONS: usize = 24;

    fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    fn rotations(self) -> [Self; Self::ROTATIONS] {
        [
            Coord::new(self.x, self.y, self.z),
            Coord::new(self.y, -self.x, self.z),
            Coord::new(-self.x, -self.y, self.z),
            Coord::new(-self.y, self.x, self.z),
            Coord::new(self.z, self.y, -self.x),
            Coord::new(self.y, -self.z, -self.x),
            Coord::new(-self.z, -self.y, -self.x),
            Coord::new(-self.y, self.z, -self.x),
            Coord::new(-self.x, self.y, -self.z),
            Coord::new(self.y, self.x, -self.z),
            Coord::new(self.x, -self.y, -self.z),
            Coord::new(-self.y, -self.x, -self.z),
            Coord::new(-self.z, self.y, self.x),
            Coord::new(self.y, self.z, self.x),
            Coord::new(self.z, -self.y, self.x),
            Coord::new(-self.y, -self.z, self.x),
            Coord::new(self.x, self.z, -self.y),
            Coord::new(self.z, -self.x, -self.y),
            Coord::new(-self.x, -self.z, -self.y),
            Coord::new(-self.z, self.x, -self.y),
            Coord::new(-self.x, self.z, self.y),
            Coord::new(self.z, self.x, self.y),
            Coord::new(self.x, -self.z, self.y),
            Coord::new(-self.z, -self.x, self.y),
        ]
    }
}

impl ops::Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Neg for Coord {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Coord::new(-self.x, -self.y, -self.z)
    }
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

fn main() {
    let scanners = include_str!("input")
        .split("\n\n")
        .map(|group| group.lines().skip(1)
            .map(|line| {
                let mut numbers = line.split(',');
                Coord::new(numbers.next().unwrap().parse::<i64>().unwrap(),
                    numbers.next().unwrap().parse::<i64>().unwrap(),
                    numbers.next().unwrap().parse::<i64>().unwrap())
            }).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    let mut diffs = vec![vec![None; scanners.len()]; scanners.len()];
    
    for (i, scanner1) in scanners.iter().enumerate() {
        'jloop: for (j, scanner2) in scanners.iter().enumerate().filter(|(j, _)| i != *j) {
            for b1 in scanner1.iter().copied() {
                for r in 0..Coord::ROTATIONS {
                    for b2 in scanner2.iter().map(|&b| b.rotations()[r]) {
                        let diff = b1 - b2;
                        let shared = scanner1.iter().map(|&b| b - diff)
                            .filter(|&b1| scanner2.iter().map(|&b| b.rotations()[r])
                                .any(|b2| b1 == b2))
                            .count();
                        if shared >= 12 {
                            diffs[i][j] = Some((diff, r));
                            continue 'jloop;
                        }
                    }
                }
            }
        }
    }

    let mut scanner_positions = vec![None; scanners.len()];
    scanner_positions[0] = Some((Coord::new(0, 0, 0), vec![0]));

    for i in 1..scanners.len() {
        update_position(i, &mut scanner_positions, &diffs, Vec::new());
    }

    let mut beacons = HashSet::new();

    for (i, mut scanner) in scanners.into_iter().enumerate() {
        let (pos, rs) = scanner_positions[i].as_ref().unwrap();
        for r in rs {
            for c in &mut scanner {
                *c = c.rotations()[*r];
            }
        }
        for c in scanner {
            beacons.insert(c + *pos);
        }
    }

    println!("Part 1: {}", beacons.len());

    let mut max_dist = 0;
    for (c1, _) in scanner_positions.iter().flatten() {
        for (c2, _) in scanner_positions.iter().flatten() {
            let dist = (c1.x - c2.x).abs() + (c1.y - c2.y).abs() + (c1.z - c2.z).abs();
            if dist > max_dist {
                max_dist = dist;
            }
        }
    }

    println!("Part 2: {}", max_dist);
}

fn update_position(scanner: usize, positions: &mut [Option<(Coord, Vec<usize>)>], diffs: &[Vec<Option<(Coord, usize)>>], ignore: Vec<usize>) -> Option<(Coord, Vec<usize>)> {
    if let Some(p) = &positions[scanner] {
        return Some(p.clone());
    }

    for (i, i_diffs) in diffs.iter().enumerate().filter(|(i, _)| *i != scanner && !ignore.contains(i)) {
        if let Some((diff, r)) = i_diffs[scanner] {
            let mut ignore = ignore.clone();
            ignore.push(scanner);
            if let Some((c, rs)) = update_position(i, positions, diffs, ignore) {
                let mut new_diff = diff;
                for r in rs.iter().copied() {
                    new_diff = new_diff.rotations()[r];
                }
                let mut new_rs = Vec::with_capacity(rs.len() + 1);
                new_rs.push(r);
                new_rs.extend(rs.iter());
                let pos = c + new_diff;
                positions[scanner] = Some((pos, new_rs.clone()));
                return Some((pos, new_rs));
            }
        }
    }

    None
}
