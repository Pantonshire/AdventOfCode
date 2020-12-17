use std::fmt::Debug;
use std::collections::HashSet;
use std::hash::Hash;

type Point3 = (i64,i64,i64);
type Point4 = (i64,i64,i64,i64);

trait Point: Sized + Copy + Eq + Hash + Debug {
    fn new(x: i64, y: i64) -> Self;
    fn bounds<'a, I: Iterator<Item=Self>>(ps: I) -> (Option<Self>, Option<Self>);
    fn neighbours(self) -> Vec<Self>;
    fn step(self, min: Self, max: Self) -> Option<Self>;
}

impl Point for Point3 {
    fn new(x: i64, y: i64) -> Self {
        return (x,y,0)
    }

    fn bounds<I: Iterator<Item=Self>>(ps: I) -> (Option<Self>, Option<Self>) {
        let (mut min_x, mut max_x) = (None, None);
        let (mut min_y, mut max_y) = (None, None);
        let (mut min_z, mut max_z) = (None, None);
        for (x,y,z) in ps {
            min_x = min_opt(x, min_x);
            max_x = max_opt(x, max_x);
            min_y = min_opt(y, min_y);
            max_y = max_opt(y, max_y);
            min_z = min_opt(z, min_z);
            max_z = max_opt(z, max_z);
        }
        (match (min_x, min_y, min_z) {
            (Some(x), Some(y), Some(z)) => Some((x-1,y-1,z-1)),
            _ => None,
        },
        match (max_x, max_y, max_z) {
            (Some(x), Some(y), Some(z)) => Some((x+1,y+1,z+1)),
            _ => None,
        })
    }

    fn neighbours(self) -> Vec<Self> {
        let mut ns = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    if x != 0 || y != 0 || z != 0 {
                        ns.push((self.0 + x, self.1 + y, self.2 + z));
                    }
                }
            }
        }
        ns
    }

    fn step(self, min: Self, max: Self) -> Option<Self> {
        if max.0 < self.0 || max.1 < self.1 || max.2 < self.2 {
            return None;
        }
        let mut next = self;
        next.2 += 1;
        if next.2 > max.2 {
            next.2 = min.2;
            next.1 += 1;
            if next.1 > max.1 {
                next.1 = min.1;
                next.0 += 1;
            }
        }
        Some(next)
    }
}

impl Point for Point4 {
    fn new(x: i64, y: i64) -> Self {
        return (x,y,0,0)
    }

    fn bounds<I: Iterator<Item=Self>>(ps: I) -> (Option<Self>, Option<Self>) {
        let (mut min_x, mut max_x) = (None, None);
        let (mut min_y, mut max_y) = (None, None);
        let (mut min_z, mut max_z) = (None, None);
        let (mut min_w, mut max_w) = (None, None);
        for (x,y,z,w) in ps {
            min_x = min_opt(x, min_x);
            max_x = max_opt(x, max_x);
            min_y = min_opt(y, min_y);
            max_y = max_opt(y, max_y);
            min_z = min_opt(z, min_z);
            max_z = max_opt(z, max_z);
            min_w = min_opt(w, min_w);
            max_w = max_opt(w, max_w);
        }
        (match (min_x, min_y, min_z, min_w) {
            (Some(x), Some(y), Some(z), Some(w)) => Some((x-1,y-1,z-1,w-1)),
            _ => None,
        },
        match (max_x, max_y, max_z, max_w) {
            (Some(x), Some(y), Some(z), Some(w)) => Some((x+1,y+1,z+1,w+1)),
            _ => None,
        })
    }

    fn neighbours(self) -> Vec<Self> {
        let mut ns = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    for w in -1..2 {
                        if x != 0 || y != 0 || z != 0 || w != 0 {
                            ns.push((self.0 + x, self.1 + y, self.2 + z, self.3 + w));
                        }
                    }
                }
            }
        }
        ns
    }

    fn step(self, min: Self, max: Self) -> Option<Self> {
        if max.0 < self.0 || max.1 < self.1 || max.2 < self.2 || max.3 < self.3 {
            return None;
        }
        let mut next = self;
        next.3 += 1;
        if next.3 > max.3 {
            next.3 = min.3;
            next.2 += 1;
            if next.2 > max.2 {
                next.2 = min.2;
                next.1 += 1;
                if next.1 > max.1 {
                    next.1 = min.1;
                    next.0 += 1;
                }
            }
        }
        Some(next)
    }
}

#[derive(Clone, Copy)]
struct PointRange<T: Point> {
    current: T,
    min: T,
    max: T,
}

impl<T: Point> PointRange<T> {
    fn new(min: T, max: T) -> Self {
        return PointRange{current: min, min: min, max: max}
    }
}

impl<T: Point> Iterator for PointRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let p = self.current;
        self.current = self.current.step(self.min, self.max)?;
        Some(p)
    }
}

fn main() {
    let contents = include_str!("../input");

    let starting_cells = contents.split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars()
            .map(|c| c == '#')
            .collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    let active_3d = run_automata::<Point3>(&starting_cells, 6).unwrap();
    println!("Part 1: {}", active_3d);

    let active_4d = run_automata::<Point4>(&starting_cells, 6).unwrap();
    println!("Part 2: {}", active_4d);
}

fn run_automata<T: Point>(starting_cells: &Vec<Vec<bool>>, cycles: usize) -> Option<usize> {
    let mut space: HashSet<T> = HashSet::new();
    for (x,row) in starting_cells.iter().enumerate() {
        for (y,_) in row.iter().enumerate().filter(|&(_,&cell)| cell) {
            space.insert(Point::new(x as i64, y as i64));
        }
    }
    for _ in 0..cycles {
        let (min, max) = Point::bounds(space.iter().copied());
        let range = PointRange::new(min?, max?);
        let mut new_space: HashSet<T> = HashSet::new();
        for point in range {
            let active_neighbours = point.neighbours()
                .iter()
                .filter(|p| space.contains(p))
                .count();
            if space.contains(&point) {
                if active_neighbours == 2 || active_neighbours == 3 {
                    new_space.insert(point);
                }
            } else {
                if active_neighbours == 3 {
                    new_space.insert(point);
                }
            }
        }
        space = new_space;
    }
    Some(space.len())
}

fn min_opt(x: i64, oy: Option<i64>) -> Option<i64> {
    Some(match oy {
        None => x,
        Some(y) => if x < y { x } else { y },
    })
}

fn max_opt(x: i64, oy: Option<i64>) -> Option<i64> {
    Some(match oy {
        None => x,
        Some(y) => if x > y { x } else { y },
    })
}
