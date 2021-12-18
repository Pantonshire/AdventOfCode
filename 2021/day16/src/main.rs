use bitvec::prelude::*;

#[derive(Copy, Clone, Debug)]
struct Packet {
    ptype: PacketType,
    len: usize,
}

#[derive(Copy, Clone, Debug)]
enum PacketType {
    Lit(u64),
    Sum(Len),
    Prod(Len),
    Min(Len),
    Max(Len),
    Gt(Len),
    Lt(Len),
    Eq(Len),
}

#[derive(Copy, Clone, Debug)]
enum Len {
    Bits(u16),
    Packets(u16),
}

fn main() {
    let input = include_str!("input").trim();
    let nibbles = input.chars().map(decode_nibble).collect::<Vec<_>>();
    let mut bytes = Vec::new();
    let mut nibble_iter = nibbles.into_iter();
    while let Some(nibble) = nibble_iter.next() {
        bytes.push((nibble << 4) | nibble_iter.next().unwrap_or(0))
    }

    let mut bits = bytes.view_bits::<Msb0>();

    let mut version_sum = 0u64;

    let mut packets = Vec::new();

    while bits.len() > 6 {
        let version = bits[..3].load_be::<u8>();
        bits = &bits[3..];
        version_sum += version as u64;
        let id = bits[..3].load_be::<u8>();
        bits = &bits[3..];
        let mut plen = 6;

        if id == 4{
            let mut last = false;
            let mut number_nibbles = Vec::new();
            while !last {
                last = !bits[0];
                bits = &bits[1..];
                number_nibbles.push(bits[..4].load_be::<u8>());
                bits = &bits[4..];
                plen += 5;
            }
            let mut number = 0u64;
            let n = number_nibbles.len();
            for (i, nibble) in number_nibbles.into_iter().enumerate() {
                number |= (nibble as u64) << ((n - i - 1) * 4);
            }
            packets.push(Packet {
                ptype: PacketType::Lit(number),
                len: plen,
            });
        } else {
            let len_type = bits[0];
            bits = &bits[1..];

            let len = if len_type {
                let packet_len = bits[..11].load_be::<u16>();
                bits = &bits[11..];
                plen += 12;
                Len::Packets(packet_len)
            } else {
                let bit_len = bits[..15].load_be::<u16>();
                bits = &bits[15..];
                plen += 16;
                Len::Bits(bit_len)
            };

            let ptype = match id {
                0 => PacketType::Sum(len),
                1 => PacketType::Prod(len),
                2 => PacketType::Min(len),
                3 => PacketType::Max(len),
                5 => PacketType::Gt(len),
                6 => PacketType::Lt(len),
                7 => PacketType::Eq(len),
                _ => panic!(),
            };

            packets.push(Packet {
                ptype,
                len: plen,
            });
        }
    }

    println!("Part 1: {}", version_sum);

    let parsed = parse(&packets);
    let mut parsed = parsed.as_slice();
    println!("Part 2: {}", eval(&mut parsed).0);
}

fn decode_nibble(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        _ => panic!(),
    }
}

#[derive(Debug)]
enum Expr {
    Lit(u64),
    Sum(Len2),
    Prod(Len2),
    Min(Len2),
    Max(Len2),
    Gt(Len2),
    Lt(Len2),
    Eq(Len2),
}

fn parse(packets: &[Packet]) -> Vec<Expr> {
    let mut exprs = Vec::new();

    for (i, p) in packets.iter().enumerate() {
        exprs.push(match p.ptype {
            PacketType::Lit(lit) => Expr::Lit(lit),
            PacketType::Sum(len) => Expr::Sum(to_packet_len(len, &packets[i+1..])),
            PacketType::Prod(len) => Expr::Prod(to_packet_len(len, &packets[i+1..])),
            PacketType::Min(len) => Expr::Min(to_packet_len(len, &packets[i+1..])),
            PacketType::Max(len) => Expr::Max(to_packet_len(len, &packets[i+1..])),
            PacketType::Gt(len) => Expr::Gt(to_packet_len(len, &packets[i+1..])),
            PacketType::Lt(len) => Expr::Lt(to_packet_len(len, &packets[i+1..])),
            PacketType::Eq(len) => Expr::Eq(to_packet_len(len, &packets[i+1..])),
        })
    }

    exprs
}

fn eval(mut exprs: &[Expr]) -> (u64, usize) {
    let expr = &exprs[0];
    let (op, n) = match expr {
        Expr::Lit(n) => return (*n, 1),
        Expr::Sum(n) => (Op::Sum, n),
        Expr::Prod(n) => (Op::Prod, n),
        Expr::Min(n) => (Op::Min, n),
        Expr::Max(n) => (Op::Max, n),
        Expr::Gt(n) => (Op::Gt, n),
        Expr::Lt(n) => (Op::Lt, n),
        Expr::Eq(n) => (Op::Eq, n),
    };
    exprs = &exprs[1..];
    match n {
        Len2::Packets(n) => {
            let n = *n as usize;
            let mut args = Vec::new();
            let mut consumed = 0;
            exprs = &exprs[..n];
            while consumed < n {
                let (v, c) = eval(&exprs[consumed..]);
                args.push(v);
                consumed += c;
            }
            (op.exec(&args), consumed + 1)
        }
        Len2::Subpackets(n) => {
            let mut args = Vec::new();
            let mut consumed = 0;
            for _ in 0..*n {
                let (v, c) = eval(&exprs[consumed..]);
                args.push(v);
                consumed += c;
            }
            (op.exec(&args), consumed + 1)
        }
    }
}

#[derive(Debug)]
enum Op {
    Sum,
    Prod,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

impl Op {
    fn exec(self, args: &[u64]) -> u64 {
        match self {
            Op::Sum => args.iter().sum(),
            Op::Prod => args.iter().product(),
            Op::Min => *args.iter().min().unwrap(),
            Op::Max => *args.iter().max().unwrap(),
            Op::Gt => if args[0] > args[1] { 1 } else { 0 },
            Op::Lt => if args[0] < args[1] { 1 } else { 0 },
            Op::Eq => if args[0] == args[1] { 1 } else { 0 },
        }
    }
}

fn to_packet_len(len: Len, packets: &[Packet]) -> Len2 {
    match len {
        Len::Packets(ps) => Len2::Subpackets(ps),
        Len::Bits(bs) => {
            let mut ps = 0;
            let mut t = 0;
            for p in packets {
                t += p.len;
                if t > bs as usize {
                    break;
                }
                ps += 1;
            }
            Len2::Packets(ps)
        }
    }
}

#[derive(Debug)]
enum Len2 {
    Packets(u16),
    Subpackets(u16),
}
