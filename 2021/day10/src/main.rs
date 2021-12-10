enum Bracket {
    Paren,
    Square,
    Curly,
    Angle,
}

fn main() {
    let input = include_str!("input")
        .lines()
        .flat_map(|line| match line.trim() {
            s if s.is_empty() => None,
            s => Some(s),
        })
        .collect::<Vec<_>>();

    let mut error = 0u64;
    
    let mut scores = input.iter().filter_map(|line| {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' => stack.push(Bracket::Paren),
                '[' => stack.push(Bracket::Square),
                '{' => stack.push(Bracket::Curly),
                '<' => stack.push(Bracket::Angle),
                c => if let Some(e) = stack.pop() {
                    match (e, c) {
                        (Bracket::Paren, ')') => (),
                        (Bracket::Square, ']') => (),
                        (Bracket::Curly, '}') => (),
                        (Bracket::Angle, '>') => (),
                        (_, c) => {
                            error += match c {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                _ => 0,
                            };
                            return None;
                        }
                    }
                },
            }
        }
        Some(stack.into_iter().rev().fold(0, |acc: u64, c| {
            acc * 5 + match c {
                Bracket::Paren => 1,
                Bracket::Square => 2,
                Bracket::Curly => 3,
                Bracket::Angle => 4,
            }
        }))
    }).collect::<Vec<_>>();

    println!("Part 1: {}", error);

    scores.sort_unstable();
    let middle = scores[scores.len() / 2];

    println!("Part 2: {}", middle);
}
