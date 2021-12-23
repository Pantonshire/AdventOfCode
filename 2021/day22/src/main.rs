use std::ops;
use std::fmt;

#[derive(Copy, Clone)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Self {
        Self {
            start,
            end,
        }
    }

    fn clamp(self, min: i64, max: i64) -> Self {
        Self::new(self.start.max(min), self.end.min(max))
    }

    fn into_range_inclusive(self) -> ops::RangeInclusive<i64> {
        self.into()
    }

    fn is_zero(self) -> bool {
        self.end < self.start
    }

    fn length(self) -> i64 {
        if self.end < self.start {
            0
        } else {
            self.end - self.start + 1
        }
    }
}

impl From<Range> for ops::RangeInclusive<i64> {
    fn from(range: Range) -> Self {
        range.start..=range.end
    }
}

impl fmt::Debug for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

#[derive(Copy, Clone)]
struct Cuboid {
    x: Range,
    y: Range,
    z: Range,
}

impl Cuboid {
    fn new(x: Range, y: Range, z: Range) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    fn overlapping_region(self, other: Cuboid) -> Self {
        Cuboid::new(
            Range::new(self.x.start.max(other.x.start), self.x.end.min(other.x.end)),
            Range::new(self.y.start.max(other.y.start), self.y.end.min(other.y.end)),
            Range::new(self.z.start.max(other.z.start), self.z.end.min(other.z.end))
        )
    }

    fn is_zero(self) -> bool {
        self.x.is_zero() || self.y.is_zero() || self.z.is_zero()
    }

    fn volume(self) -> i64 {
        self.x.length() * self.y.length() * self.z.length()
    }
}

impl fmt::Debug for Cuboid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[x={:?}, y={:?}, z={:?}]", self.x, self.y, self.z)
    }
}

fn main() {
    let input = include_str!("input")
        .lines()
        .filter_map(|line| match line.trim() {
            s if s.is_empty() => None,
            s => Some(parse_line(s)),
        })
        .collect::<Vec<_>>();

    let mut region = vec![false; 1030301];

    for (state, cuboid) in input.iter().copied() {
        for x in cuboid.x.clamp(-50, 50).into_range_inclusive() {
            for y in cuboid.y.clamp(-50, 50).into_range_inclusive() {
                for z in cuboid.z.clamp(-50, 50).into_range_inclusive() {
                    region[(x + 50) as usize * 10201 + (y + 50) as usize * 101 + (z + 50) as usize] = state;
                }
            }
        }
    }

    let on_count = region.into_iter().filter(|state| *state).count();

    println!("Part 1: {}", on_count);

    let mut cuboids = Vec::new();

    for (state, cuboid) in input {
        let overlaps = cuboids.iter().copied()
            .filter_map(|(state, other)| match cuboid.overlapping_region(other) {
                overlap if overlap.is_zero() => None,
                overlap => Some((state, overlap)),
            })
            .collect::<Vec<_>>();

        if state {
            cuboids.push((true, cuboid));
        }

        for (overlap_state, overlap) in overlaps {
            cuboids.push((!overlap_state, overlap));
        }
    }

    let on_count = cuboids.into_iter().fold(0i64, |acc, (state, cuboid)| {
        match state {
            true => acc + cuboid.volume(),
            false => acc - cuboid.volume(),
        }
    });

    println!("Part 2: {}", on_count);
}

fn parse_line(line: &str) -> (bool, Cuboid) {
    let (state, cube) = line.split_once(' ').unwrap();
    let state = state == "on";
    let mut cube = cube.splitn(3, ',');
    (state, Cuboid::new(parse_range(cube.next().unwrap()), parse_range(cube.next().unwrap()), parse_range(cube.next().unwrap())))
}

fn parse_range(range: &str) -> Range {
    let (_, range) = range.split_once('=').unwrap();
    let (start, end) = range.split_once("..").unwrap();
    let start = start.parse().unwrap();
    let end = end.parse().unwrap();
    Range::new(start, end)
}
