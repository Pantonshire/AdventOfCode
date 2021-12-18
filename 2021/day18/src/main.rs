use std::iter::Peekable;
use std::fmt;

fn main() {
    let pairs = include_str!("input_test")
        .lines()
        .filter_map(|line| match line.trim() {
            s if s.is_empty() => None,
            s => Some(parse_pair(&mut s.chars().peekable())),
        })
        .collect::<Vec<_>>();

    let mut sum = None;

    for pair in pairs.clone() {
        sum = match sum {
            None => Some(pair),
            Some(lhs) => Some(add_pairs(lhs, pair)),
        };
    }
    
    println!("Part 1: {}", sum.unwrap().magnitude());

    let mut max = 0;
    for (i, lhs) in pairs.clone().into_iter().enumerate() {
        for (_, rhs) in pairs.clone().into_iter().enumerate().filter(|(j, _)| i != *j) {
            let sum = add_pairs(lhs.clone(), rhs).magnitude();
            if sum > max {
                max = sum;
            }
        }
    }

    println!("Part 2: {}", max);
}

#[derive(Clone)]
struct Pair {
    left: Element,
    right: Element,
}

impl fmt::Debug for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?},{:?}]", self.left, self.right)
    }
}

impl Pair {
    fn magnitude(&self) -> u64 {
        3 * match &self.left {
            Element::Number(n) => *n,
            Element::Pair(p) => p.magnitude(),
        } + 2 * match &self.right {
            Element::Number(n) => *n,
            Element::Pair(p) => p.magnitude(),
        }
    }
}

#[derive(Clone)]
enum Element {
    Number(u64),
    Pair(Box<Pair>),
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Number(n) => n.fmt(f),
            Element::Pair(p) => p.fmt(f),
        }
    }
}

fn add_pairs(lhs: Pair, rhs: Pair) -> Pair {
    let mut sum = Pair {
        left: Element::Pair(Box::new(lhs)),
        right: Element::Pair(Box::new(rhs)),
    };

    println!("{:?}", sum);

    loop {
        if explode_pair(&mut sum, 0).0 {
            println!("{:?}   (exploded)", sum);
            continue;
        }
        if split(&mut sum) {
            println!("{:?}   (split)", sum);
            continue;
        }
        break;
    }
    
    // println!("{:?}", sum);
    // while explode_pair(&mut sum, 0).0 || split(&mut sum) {
        // println!("= {:?}", sum);
    // }

    sum
}

fn split(pair: &mut Pair) -> bool {
    match &mut pair.left {
        Element::Number(n) => {
            if *n >= 10 {
                pair.left = Element::Pair(Box::new(Pair {
                    left: Element::Number(*n / 2),
                    right: Element::Number(*n - (*n / 2)),
                }));
                return true;
            }
        },
        Element::Pair(p) => {
            if split(p) {
                return true;
            }
        },
    }
    match &mut pair.right {
        Element::Number(n) => {
            if *n >= 10 {
                pair.right = Element::Pair(Box::new(Pair {
                    left: Element::Number(*n / 2),
                    right: Element::Number(*n - (*n / 2)),
                }));
                return true;
            }
        },
        Element::Pair(p) => {
            if split(p) {
                return true;
            }
        },
    }
    return false;
}

fn explode_pair(pair: &mut Pair, depth: usize) -> (bool, Option<u64>, Option<u64>) {
    let (cl, lel, ler) = explode(&mut pair.left, depth + 1);
    if let Some(er) = ler {
        *match &mut pair.right {
            Element::Number(n) => n,
            Element::Pair(p) => leftmost(p),
        } += er;
    }

    if cl {
        return (true, lel, None);
    }

    let (cr, rel, rer) = explode(&mut pair.right, depth + 1);
    if let Some(el) = rel {
        *match &mut pair.left {
            Element::Number(n) => n,
            Element::Pair(p) => rightmost(p),
        } += el;
    }

    (cl || cr, lel, rer)
}

fn explode(element: &mut Element, depth: usize) -> (bool, Option<u64>, Option<u64>) {
    if let Element::Pair(pair) = element {
        if depth == 4 {
            let left = if let Element::Number(lhs) = pair.left {
                Some(lhs)
            } else {
                None
            };
            let right = if let Element::Number(rhs) = pair.right {
                Some(rhs)
            } else {
                None
            };
            *element = Element::Number(0);
            return (true, left, right);
        }

        explode_pair(pair, depth)
    } else {
        (false, None, None)
    }
}

fn rightmost(pair: &mut Pair) -> &mut u64 {
    match &mut pair.right {
        Element::Number(n) => n,
        Element::Pair(pair) => rightmost(pair),
    }
}

fn leftmost(pair: &mut Pair) -> &mut u64 {
    match &mut pair.left {
        Element::Number(n) => n,
        Element::Pair(pair) => leftmost(pair),
    }
}

fn parse_pair<I>(cs: &mut Peekable<I>) -> Pair where I: Iterator<Item = char> {
    cs.next();
    let left = match cs.peek().unwrap() {
        '[' => Element::Pair(Box::new(parse_pair(cs))),
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
