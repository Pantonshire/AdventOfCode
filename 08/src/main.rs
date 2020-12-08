use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Instr {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

fn main() {
    let contents = include_str!("../input");
    let instrs: Vec<Result<Instr, &str>> = contents.split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| read_instruction(l))
        .collect();

    match run_program(instrs.clone()) {
        Ok(acc) => println!("Part 1: {}", acc.0),
        Err(e) => eprintln!("Error in part 1: {}", e),
    }

    fn swap_instr(i: usize, j: usize, instr: Instr) -> Instr {
        if i != j {
            return instr;
        }
        match instr {
            Instr::Nop(x) => Instr::Jmp(x),
            Instr::Jmp(x) => Instr::Nop(x),
            Instr::Acc(x) => Instr::Acc(x),
        }
    }

    for i in 0..(instrs.len()) {
        match run_program_transform(instrs.clone(), |(j,instr)| swap_instr(i, j, instr)) {
            Ok((acc, true)) => {
                println!("Part 2: {}", acc);
                break;
            },
            Ok((_, false)) => continue,
            Err(e) => eprintln!("Error in part 2 transforming line {}: {}", i, e),
        }
    }
}

fn run_program<'a>(instrs: Vec<Result<Instr, &'a str>>) -> Result<(i32, bool), &'a str> {
    run_program_transform(instrs, |(_,instr)| instr)
}

fn run_program_transform<'a, F>(instrs: Vec<Result<Instr, &'a str>>, transform: F) -> Result<(i32, bool), &'a str>
where
    F: Fn((usize, Instr)) -> Instr,
{
    let n = instrs.len();
    let mut i = 0;
    let mut acc = 0;
    let mut visited: HashSet<usize> = HashSet::new();
    while i < n {
        if visited.contains(&i) {
            return Ok((acc, false))
        }
        visited.insert(i);
        match transform((i, instrs[i]?)) {
            Instr::Nop(_) => {
                i += 1
            },
            Instr::Jmp(x) => {
                i = (i as i32 + x) as usize
            },
            Instr::Acc(x) => {
                acc += x;
                i += 1;
            },
        }
    }
    Ok((acc, true))
}

fn read_instruction(line: &str) -> Result<Instr, &str> {
    let parts: Vec<&str> = line.splitn(2, " ").collect();
    if parts.len() != 2 {
        return Err("");
    }
    let (opcode, operand) = (parts[0], parts[1]);
    let operand: i32 = match operand.parse() {
        Ok(n) => n,
        Err(_) => return Err("Error parsing operand"),
    };
    match opcode {
        "nop" => Ok(Instr::Nop(operand)),
        "jmp" => Ok(Instr::Jmp(operand)),
        "acc" => Ok(Instr::Acc(operand)),
        _ => Err("Invalid opcode")
    }
}
