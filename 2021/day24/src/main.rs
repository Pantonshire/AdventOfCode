fn main() {
    let instructions = include_str!("input")
        .lines()
        .filter_map(|line| match line.trim() {
            line if line.is_empty() => None,
            line => Some(parse_instruction(line).unwrap()),
        })
        .collect::<Vec<_>>();

    let mut alu = Alu::new(instructions.iter().copied(), [9, 2, 9, 1, 5, 9, 7, 9, 9, 9, 9, 4, 9, 8].into_iter());
    while alu.step() {}
    println!("{}", alu.registers[3]);

    let mut alu = Alu::new(instructions.iter().copied(), [2, 1, 6, 1, 1, 5, 1, 3, 9, 1, 1, 1, 8, 1].into_iter());
    while alu.step() {}
    println!("{}", alu.registers[3]);
}

struct Alu<I, J> {
    instructions: I,
    input: J,
    registers: [i64; 4],
}

impl<I, J> Alu<I, J>
where
    I: Iterator<Item = Instruction>,
    J: Iterator<Item = i64>,
{
    fn new(instructions: I, input: J) -> Self {
        Self {
            instructions,
            input,
            registers: [0; 4],
        }
    }

    fn step(&mut self) -> bool {
        let instruction = match self.instructions.next() {
            Some(instruction) => instruction,
            None => return false,
        };

        match instruction {
            Instruction::Inp(i) => self.registers[i] = self.input.next().unwrap(),
            Instruction::Add(i, o) => self.registers[i] = self.registers[i] + self.value_of(o),
            Instruction::Mul(i, o) => self.registers[i] = self.registers[i] * self.value_of(o),
            Instruction::Div(i, o) => self.registers[i] = self.registers[i] / self.value_of(o),
            Instruction::Mod(i, o) => self.registers[i] = self.registers[i] % self.value_of(o),
            Instruction::Eql(i, o) => self.registers[i] = if self.registers[i] == self.value_of(o) { 1 } else { 0 },
        }

        true
    }

    fn value_of(&self, operand: Operand) -> i64 {
        match operand {
            Operand::Constant(c) => c,
            Operand::Variable(i) => self.registers[i],
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Inp(usize),
    Add(usize, Operand),
    Mul(usize, Operand),
    Div(usize, Operand),
    Mod(usize, Operand),
    Eql(usize, Operand),
}

#[derive(Copy, Clone, Debug)]
enum Operand {
    Constant(i64),
    Variable(usize),
}

fn parse_instruction(s: &str) -> Option<Instruction> {
    let (op, args) = s.split_once(' ')?;
    match op {
        "inp" => Some(Instruction::Inp(parse_variable(args)?)),
        _ => {
            let (arg1, arg2) = args.split_once(' ')?;
            let arg1 = parse_variable(arg1)?;
            let arg2 = parse_operand(arg2)?;
            match op {
                "add" => Some(Instruction::Add(arg1, arg2)),
                "mul" => Some(Instruction::Mul(arg1, arg2)),
                "div" => Some(Instruction::Div(arg1, arg2)),
                "mod" => Some(Instruction::Mod(arg1, arg2)),
                "eql" => Some(Instruction::Eql(arg1, arg2)),
                _ => None,
            }
        }
    }
}

fn parse_operand(s: &str) -> Option<Operand> {
    parse_variable(s).map(Operand::Variable)
        .or_else(|| s.parse().ok().map(Operand::Constant))
}

fn parse_variable(s: &str) -> Option<usize> {
    match s {
        "w" => Some(0),
        "x" => Some(1),
        "y" => Some(2),
        "z" => Some(3),
        _ => None,
    }
}
