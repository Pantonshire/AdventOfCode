type Point = (i64,i64);

#[derive(Copy, Clone)]
enum Cardinal {
    North,
    South,
    East,
    West,
}

impl Cardinal {
    fn vec(self) -> Point {
        match self {
            Cardinal::North => (0,1),
            Cardinal::South => (0,-1),
            Cardinal::East => (1,0),
            Cardinal::West => (-1,0),
        }
    }

    fn cw(self) -> Cardinal {
        match self {
            Cardinal::North => Cardinal::East,
            Cardinal::South => Cardinal::West,
            Cardinal::East => Cardinal::South,
            Cardinal::West => Cardinal::North,
        }
    }

    fn ccw(self) -> Cardinal {
        match self {
            Cardinal::North => Cardinal::West,
            Cardinal::South => Cardinal::East,
            Cardinal::East => Cardinal::North,
            Cardinal::West => Cardinal::South,
        }
    }
}

#[derive(Copy, Clone)]
enum Instr {
    N(i64),
    S(i64),
    E(i64),
    W(i64),
    F(i64),
    L(i64),
    R(i64),
}

impl Instr {
    fn apply(self, (x,y): Point, dir: Cardinal) -> (Point, Cardinal) {
        match self {
            Instr::N(l) => ((x,y+l), dir),
            Instr::S(l) => ((x,y-l), dir),
            Instr::E(l) => ((x+l,y), dir),
            Instr::W(l) => ((x-l,y), dir),
            Instr::F(l) => {
                let (dx,dy) = dir.vec();
                ((x+(dx*l), y+(dy*l)), dir)
            },
            Instr::L(ts) => {
                let mut new_dir = dir;
                for _ in 0..ts {
                    new_dir = new_dir.ccw();
                }
                ((x,y), new_dir)
            },
            Instr::R(ts) => {
                let mut new_dir = dir;
                for _ in 0..ts {
                    new_dir = new_dir.cw();
                }
                ((x,y), new_dir)
            },
        }
    }

    fn apply_waypoint(self, (x,y): Point, (wx,wy): Point) -> (Point, Point) {
        match self {
            Instr::N(l) => ((x,y), (wx,wy+l)),
            Instr::S(l) => ((x,y), (wx,wy-l)),
            Instr::E(l) => ((x,y), (wx+l,wy)),
            Instr::W(l) => ((x,y), (wx-l,wy)),
            Instr::F(l) => {
                let (dx, dy) = ((wx-x)*l, (wy-y)*l);
                ((x+dx, y+dy), (wx+dx, wy+dy))
            },
            Instr::L(ts) => {
                let mut w = (wx,wy);
                for _ in 0..ts {
                    w = rotate_ccw((x,y), w);
                }
                ((x,y), w)
            },
            Instr::R(ts) => {
                let mut w = (wx,wy);
                for _ in 0..ts {
                    w = rotate_cw((x,y), w);
                }
                ((x,y), w)
            },
        }
    }
}

fn main() {
    let contents = include_str!("../input");

    let instrs = contents.split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| read_instr(l).expect("Error reading instruction"));

    let mut boat_1 = ((0,0), Cardinal::East);
    let mut boat_2 = ((0,0), (10,1));

    for instr in instrs {
        boat_1 = instr.apply(boat_1.0, boat_1.1);
        boat_2 = instr.apply_waypoint(boat_2.0, boat_2.1);
    }

    println!("Part 1: {}", boat_1.0.0.abs() + boat_1.0.1.abs());
    println!("Part 2: {}", boat_2.0.0.abs() + boat_2.0.1.abs());
}

fn read_instr(line: &str) -> Option<Instr> {
    let mut cs = line.chars();
    let instr_name = cs.next();
    let arg = cs.collect::<String>();
    let arg = match arg.parse::<i64>() {
        Ok(n) => n,
        Err(_) => return None,
    };
    match instr_name {
        Some('N') => Some(Instr::N(arg)),
        Some('S') => Some(Instr::S(arg)),
        Some('E') => Some(Instr::E(arg)),
        Some('W') => Some(Instr::W(arg)),
        Some('F') => Some(Instr::F(arg)),
        Some('L') => Some(Instr::L(arg/90)),
        Some('R') => Some(Instr::R(arg/90)),
        _ => None,
    }
}

fn rotate_cw((piv_x,piv_y): Point, (x,y): Point) -> Point {
    (piv_x + (y - piv_y), piv_y - (x - piv_x))
}

fn rotate_ccw((piv_x,piv_y): Point, (x,y): Point) -> Point {
    (piv_x - (y - piv_y), piv_y + (x - piv_x))
}
