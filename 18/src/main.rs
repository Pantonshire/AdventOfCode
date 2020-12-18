#[derive(Clone, Copy, Debug)]
enum Token {
    Num(u64),
    Add,
    Mul,
    LParen,
    RParen,
}

fn main() {
    let contents = include_str!("../input");

    let lines = contents.split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(tokenise)
        .collect::<Option<Vec<Vec<Token>>>>()
        .expect("Tokenisation error");

    let part_1 = lines.iter()
        .map(|ts| eval_1(&ts).expect("Parse error"))
        .sum::<u64>();

    println!("Part 1: {}", part_1);

    let part_2 = lines.iter()
        .map(|ts| eval_2(&ts).expect("Parse error"))
        .sum::<u64>();

    println!("Part 2: {}", part_2);
}

fn tokenise(line: &str) -> Option<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut num_buf = String::new();
    for c in line.chars() {
        if c.is_ascii_digit() {
            num_buf.push(c);
        } else {
            if !num_buf.is_empty(){
                tokens.push(Token::Num(num_buf.parse::<u64>().ok()?));
                num_buf.clear();
            }
            if c.is_ascii_whitespace() {
                continue;
            }
            tokens.push(match c {
                '+' => Token::Add,
                '*' => Token::Mul,
                '(' => Token::LParen,
                ')' => Token::RParen,
                _ => return None,
            });
        }
    }
    if !num_buf.is_empty() {
        tokens.push(Token::Num(num_buf.parse::<u64>().ok()?));
    }
    Some(tokens)
}

fn eval_1(tokens: &[Token]) -> Option<u64> {
    fn eval_op(tokens: &[Token]) -> Option<(u64, usize)> {
        let (mut x, mut i) = eval_paren(tokens)?;
        loop {
            match opt_slice(tokens, i).map(|tokens| eval_apply_op(tokens, x)).unnest() {
                None => return Some((x, i)),
                Some((res, j)) => {
                    x = res;
                    i += j;
                },
            }
        }
    }

    fn eval_apply_op(tokens: &[Token], x: u64) -> Option<(u64, usize)> {
        let op = tokens.first()?;
        let tokens = opt_slice(tokens, 1)?;
        let (y,i) = eval_paren(tokens)?;
        match op {
            Token::Add => Some((x+y, i+1)),
            Token::Mul => Some((x*y, i+1)),
            _ => None,
        }
    }

    fn eval_paren(tokens: &[Token]) -> Option<(u64, usize)> {
        match tokens.first() {
            Some(Token::Num(n)) => Some((*n, 1)),
            Some(Token::LParen) => {
                let (x,i) = eval_op(opt_slice(tokens, 1)?)?;
                match tokens.get(i+1) {
                    Some(Token::RParen) => Some((x, i+2)),
                    _ => None,
                }
            },
            _ => None,
        }
    }

    let (x,i) = eval_op(tokens)?;
    if i < tokens.len() { None } else { Some(x) }
}

fn eval_2(tokens: &[Token]) -> Option<u64> {
    fn eval_mul(tokens: &[Token]) -> Option<(u64, usize)> {
        let (lhs, i) = eval_plus(tokens)?;
        match opt_slice(tokens, i).map(|ts| ts.first()).unnest() {
            Some(Token::Mul) => eval_mul(opt_slice(tokens, i+1)?).map(|(rhs,j)| (lhs*rhs, i+1+j)),
            _ => Some((lhs, i)),
        }
    }

    fn eval_plus(tokens: &[Token]) -> Option<(u64, usize)> {
        let (lhs, i) = eval_paren(tokens)?;
        match opt_slice(tokens, i).map(|ts| ts.first()).unnest() {
            Some(Token::Add) => eval_plus(opt_slice(tokens, i+1)?).map(|(rhs,j)| (lhs+rhs, i+1+j)),
            _ => Some((lhs, i)),
        }
    }

    fn eval_paren(tokens: &[Token]) -> Option<(u64, usize)> {
        match tokens.first() {
            Some(Token::Num(n)) => Some((*n, 1)),
            Some(Token::LParen) => {
                let (x,i) = eval_mul(opt_slice(tokens, 1)?)?;
                match tokens.get(i+1) {
                    Some(Token::RParen) => Some((x, i+2)),
                    _ => None,
                }
            },
            _ => None,
        }
    }

    let (x,i) = eval_mul(tokens)?;
    if i < tokens.len() { None } else { Some(x) }
}

fn opt_slice<T>(s: &[T], i: usize) -> Option<&[T]> {
    if i < s.len() {
        Some(&s[i..])
    } else {
        None
    }
}

trait Unnest<T> {
    fn unnest(self) -> T;
}

impl<T> Unnest<Option<T>> for Option<Option<T>> {
    fn unnest(self) -> Option<T> {
        match self {
            Some(x) => x,
            None => None,
        }
    }
}
