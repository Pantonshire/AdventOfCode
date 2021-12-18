use std::fmt;
use std::iter::Peekable;
use std::ops;

fn main() {
    let pairs = include_str!("input")
        .lines()
        .filter_map(|line| match line.trim() {
            s if s.is_empty() => None,
            s => Some(parse_pair(&mut s.chars().peekable())),
        })
        .collect::<Vec<_>>();
    
    let max_magnitude = pairs.clone().into_iter().enumerate()
        .map(|(i, lhs)| pairs.clone().into_iter().enumerate()
            .filter(move |(j, _)| i != *j)
            .map(move |(_, rhs)| (lhs.clone() + rhs).magnitude()))
        .flatten()
        .max()
        .unwrap();

    let sum = pairs.into_iter().fold(None, |acc, rhs| {
        match acc {
            None => Some(rhs),
            Some(lhs) => Some(lhs + rhs),
        }
    }).unwrap();

    println!("Part 1: {}", sum.magnitude());
    println!("Part 2: {}", max_magnitude);
}

#[derive(Clone)]
struct Pair {
    left: Element,
    right: Element,
}

impl Pair {
    fn magnitude(&self) -> u64 {
        3 * self.left.number_or_else(Self::magnitude)
            + 2 * self.right.number_or_else(Self::magnitude)
    }

    fn leftmost(&mut self) -> &mut u64 {
        match &mut self.left {
            Element::Number(n) => n,
            Element::Pair(pair) => pair.leftmost(),
        }
    }

    fn rightmost(&mut self) -> &mut u64 {
        match &mut self.right {
            Element::Number(n) => n,
            Element::Pair(pair) => pair.rightmost(),
        }
    }
}

impl ops::Add for Pair {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = Pair {
            left: Element::new_pair(self),
            right: Element::new_pair(rhs),
        };
        while explode_pair(&mut sum, 0).0 || split_pair(&mut sum) {}
        sum
    }
}

impl fmt::Debug for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?},{:?}]", self.left, self.right)
    }
}

#[derive(Clone)]
enum Element {
    Number(u64),
    Pair(Box<Pair>),
}

impl Element {
    fn new_pair(pair: Pair) -> Self {
        Element::Pair(Box::new(pair))
    }

    fn number_or_none(&self) -> Option<u64> {
        match self {
            Element::Number(n) => Some(*n),
            Element::Pair(_) => None,
        }
    }

    fn number_or_else<F>(&self, pair_fn: F) -> u64
    where
        F: FnOnce(&Pair) -> u64,
    {
        match self {
            Element::Number(n) => *n,
            Element::Pair(p) => pair_fn(p),
        }
    }

    fn number_or_else_mut<F>(&mut self, pair_fn: F) -> &mut u64
    where
        F: FnOnce(&mut Pair) -> &mut u64,
    {
        match self {
            Element::Number(n) => n,
            Element::Pair(p) => pair_fn(p),
        }
    }
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Number(n) => n.fmt(f),
            Element::Pair(p) => p.fmt(f),
        }
    }
}

fn split_pair(pair: &mut Pair) -> bool {
    split_element(&mut pair.left) || split_element(&mut pair.right)
}

fn split_element(element: &mut Element) -> bool {
    match element {
        Element::Number(n) if *n >= 10 => {
            let half_floor = *n / 2;
            let half_ceil = *n - half_floor;
            *element = Element::new_pair(Pair {
                left: Element::Number(half_floor),
                right: Element::Number(half_ceil),
            });
            true
        },
        Element::Pair(p) => split_pair(p),
        _ => false,
    }
}

fn explode_pair(pair: &mut Pair, depth: usize) -> (bool, Option<u64>, Option<u64>) {
    let (changed_left, explode_left, explode_right) = explode_element(&mut pair.left, depth + 1);
    if let Some(explode_right) = explode_right {
        *pair.right.number_or_else_mut(Pair::leftmost) += explode_right;
    }

    if changed_left {
        return (true, explode_left, None);
    }

    let (changed_right, explode_left, explode_right) = explode_element(&mut pair.right, depth + 1);
    if let Some(explode_left) = explode_left {
        *pair.left.number_or_else_mut(Pair::rightmost) += explode_left;
    }

    (changed_right, None, explode_right)
}

fn explode_element(element: &mut Element, depth: usize) -> (bool, Option<u64>, Option<u64>) {
    if let Element::Pair(pair) = element {
        if depth == 4 {
            let left = pair.left.number_or_none();
            let right = pair.right.number_or_none();
            *element = Element::Number(0);
            return (true, left, right);
        }
        explode_pair(pair, depth)
    } else {
        (false, None, None)
    }
}

fn parse_pair<I>(cs: &mut Peekable<I>) -> Pair
where
    I: Iterator<Item = char>,
{
    cs.next();
    let left = match cs.peek().unwrap() {
        '[' => Element::new_pair(parse_pair(cs)),
        _ => {
            let c = cs.next().unwrap();
            Element::Number(c as u64 - 0x30)
        },
    };
    cs.next();
    let right = match cs.peek().unwrap() {
        '[' => Element::Pair(Box::new(parse_pair(cs))),
        _ => {
            let c = cs.next().unwrap();
            Element::Number(c as u64 - 0x30)
        },
    };
    cs.next();
    Pair {
        left,
        right,
    }
}
