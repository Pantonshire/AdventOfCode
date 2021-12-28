use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Position {
    Hallway(Hallway),
    Room(Room),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Hallway {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
}

impl Hallway {
    const HALLWAYS: [Self; 7] = [Self::H1, Self::H2, Self::H3, Self::H4, Self::H5, Self::H6, Self::H7];

    fn pos(self) -> u8 {
        match self {
            Self::H1 => 0,
            Self::H2 => 1,
            Self::H3 => 3,
            Self::H4 => 5,
            Self::H5 => 7,
            Self::H6 => 9,
            Self::H7 => 10,
        }
    }

    fn dist(self, room: Room) -> u64 {
        room.dist(self)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Room {
    A(u8),
    B(u8),
    C(u8),
    D(u8),
}

impl Room {
    fn rooms_for(amphipod: usize) -> [Self; 4] {
        match amphipod {
            0 | 1 | 2 | 3 => [Self::A(3), Self::A(2), Self::A(1), Self::A(0)],
            4 | 5 | 6 | 7 => [Self::B(3), Self::B(2), Self::B(1), Self::B(0)],
            8 | 9 | 10 | 11 => [Self::C(3), Self::C(2), Self::C(1), Self::C(0)],
            _ => [Self::D(3), Self::D(2), Self::D(1), Self::D(0)],
        }
    }

    fn is_room_for(self, amphipod: usize) -> bool {
        match (self, amphipod) {
            (Self::A(_), 0 | 1 | 2 | 3) => true,
            (Self::B(_), 4 | 5 | 6 | 7) => true,
            (Self::C(_), 8 | 9 | 10 | 11) => true,
            (Self::D(_), 12 | 13 | 14 | 15) => true,
            _ => false,
        }
    }
    
    fn above(self) -> Option<Room> {
        match self {
            Self::A(0) => None,
            Self::A(n) => Some(Self::A(n - 1)),
            Self::B(0) => None,
            Self::B(n) => Some(Self::B(n - 1)),
            Self::C(0) => None,
            Self::C(n) => Some(Self::C(n - 1)),
            Self::D(0) => None,
            Self::D(n) => Some(Self::D(n - 1)),
        }
    }

    fn below(self) -> Option<Room> {
        match self {
            Self::A(3) => None,
            Self::A(n) => Some(Self::A(n + 1)),
            Self::B(3) => None,
            Self::B(n) => Some(Self::B(n + 1)),
            Self::C(3) => None,
            Self::C(n) => Some(Self::C(n + 1)),
            Self::D(3) => None,
            Self::D(n) => Some(Self::D(n + 1)),
        }
    }

    fn exit_pos(self) -> u8 {
        match self {
            Self::A(_) => 2,
            Self::B(_) => 4,
            Self::C(_) => 6,
            Self::D(_) => 8,
        }
    }

    fn dist_to_exit(self) -> u64 {
        (match self {
            Self::A(n) => n,
            Self::B(n) => n,
            Self::C(n) => n,
            Self::D(n) => n,
        }) as u64 + 1
    }

    fn dist(self, hallway: Hallway) -> u64 {
        let exit = self.exit_pos();
        let hall = hallway.pos();
        self.dist_to_exit() + abs_dist(exit, hall)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct StateCost {
    state: [Position; 16],
    cost: u64,
}

impl StateCost {
    fn new(state: [Position; 16], cost: u64) -> Self {
        Self {
            state,
            cost,
        }
    }
}

impl cmp::Ord for StateCost {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl cmp::PartialOrd for StateCost {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let start = std::time::Instant::now();

    let starting_positions = [
        Position::Room(Room::B(3)), Position::Room(Room::C(2)), Position::Room(Room::D(0)), Position::Room(Room::D(1)),
        Position::Room(Room::B(0)), Position::Room(Room::B(2)), Position::Room(Room::C(1)), Position::Room(Room::D(3)),
        Position::Room(Room::A(3)), Position::Room(Room::B(1)), Position::Room(Room::C(0)), Position::Room(Room::D(2)),
        Position::Room(Room::A(0)), Position::Room(Room::A(1)), Position::Room(Room::A(2)), Position::Room(Room::C(3)),
    ];

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut costs = HashMap::new();

    queue.push(StateCost::new(starting_positions, 0));

    let mut best_cost = None;

    while let Some(StateCost { state, cost, .. }) = queue.pop() {
        if is_satisfied(&state) {
            best_cost = Some(cost);
            break;
        }

        if visited.insert(state) {
            push_neighbours(&mut queue, &mut costs, state, cost);
            costs.remove(&state);
        }
    }

    let delta = std::time::Instant::now() - start;
    println!("{} ms", delta.as_millis());

    println!("Part 2: {}", best_cost.unwrap());
}

fn push_neighbours(queue: &mut BinaryHeap<StateCost>, costs: &mut HashMap<[Position; 16], u64>, state: [Position; 16], cost: u64) {
    for (amphipod, pos) in state.into_iter().enumerate() {
        match pos {
            Position::Hallway(hallway) => for room in Room::rooms_for(amphipod) {
                if let Some((i, _)) = state.into_iter().enumerate().find(|&(_, p)| p == Position::Room(room)) {
                    if room.is_room_for(i) {
                        continue;
                    } else {
                        break;
                    }
                }

                if !state.into_iter().any(|p| match p {
                    Position::Room(_) => false,
                    Position::Hallway(h) => {
                        let h_pos = h.pos();
                        let min = hallway.pos().min(room.exit_pos());
                        let max = hallway.pos().max(room.exit_pos());
                        min < h_pos && h_pos < max
                    }
                })
                {
                    let cost = cost + hallway.dist(room) * movement_cost(amphipod);
                    let mut state = state;
                    state[amphipod] = Position::Room(room);
                    if costs.get(&state).map(|&old_cost| cost < old_cost).unwrap_or(true) {
                        queue.push(StateCost::new(state, cost));
                        costs.insert(state, cost);
                    }
                }

                break;
            },

            Position::Room(room) => {
                if allowed_to_move(room, &state) {
                    for hallway in Hallway::HALLWAYS {
                        if !state.into_iter().any(|p| match p {
                            Position::Room(_) => false,
                            Position::Hallway(h) => {
                                let h_pos = h.pos();
                                let min = hallway.pos().min(room.exit_pos());
                                let max = hallway.pos().max(room.exit_pos());
                                h == hallway || (min < h_pos && h_pos < max)
                            }
                        })
                        {
                            let cost = cost + room.dist(hallway) * movement_cost(amphipod);
                            let mut state = state;
                            state[amphipod] = Position::Hallway(hallway);
                            if costs.get(&state).map(|&old_cost| cost < old_cost).unwrap_or(true) {
                                queue.push(StateCost::new(state, cost));
                                costs.insert(state, cost);
                            }
                        }
                    }
                }
            },
        }
    }
}

fn allowed_to_move(room: Room, state: &[Position]) -> bool {
    let mut r = room;
    while let Some(above) = r.above() {
        r = above;
        if state.iter().any(|&p| p == Position::Room(r)) {
            return false;
        }
    }

    let mut any_below = false;

    let mut r = Some(room);
    while let Some(room) = r {
        r = room.below();
        any_below = true;
        if state.iter().enumerate()
            .any(|(amphipod, &pos)| pos == Position::Room(room) && !room.is_room_for(amphipod))
        {
            return true;
        }
    }

    !any_below
}

fn is_satisfied(state: &[Position]) -> bool {
    state.iter().copied().enumerate().all(|(amphipod, pos)| match pos {
        Position::Hallway(_) => false,
        Position::Room(room) => room.is_room_for(amphipod),
    })
}

fn movement_cost(amphipod: usize) -> u64 {
    match amphipod / 4 {
        0 => 1,
        1 => 10,
        2 => 100,
        _ => 1000,
    }
}

fn abs_dist(p1: u8, p2: u8) -> u64 {
    if p1 > p2 {
        p1 as u64 - p2 as u64
    } else {
        p2 as u64 - p1 as u64
    }
}
