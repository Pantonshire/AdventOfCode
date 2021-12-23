use std::collections::HashMap;

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
    ATop,
    ABot,
    BTop,
    BBot,
    CTop,
    CBot,
    DTop,
    DBot,
}

impl Room {
    fn rooms_for(amphipod: usize) -> [Self; 2] {
        match amphipod {
            0 | 1 => [Self::ABot, Self::ATop],
            2 | 3 => [Self::BBot, Self::BTop],
            4 | 5 => [Self::CBot, Self::CTop],
            _ => [Self::DBot, Self::DTop],
        }
    }

    fn is_room_for(self, amphipod: usize) -> bool {
        match (self, amphipod) {
            (Self::ABot | Self::ATop, 0 | 1) => true,
            (Self::BBot | Self::BTop, 2 | 3) => true,
            (Self::CBot | Self::CTop, 4 | 5) => true,
            (Self::DBot | Self::DTop, 6 | 7) => true,
            _ => false,
        }
    }
    
    fn opposite(self) -> RoomType {
        match self {
            Self::ABot => RoomType::Top(Self::ATop),
            Self::BBot => RoomType::Top(Self::BTop),
            Self::CBot => RoomType::Top(Self::CTop),
            Self::DBot => RoomType::Top(Self::DTop),
            Self::ATop => RoomType::Bot(Self::ABot),
            Self::BTop => RoomType::Bot(Self::BBot),
            Self::CTop => RoomType::Bot(Self::CBot),
            Self::DTop => RoomType::Bot(Self::DBot),
        }
    }

    fn exit_pos(self) -> u8 {
        match self {
            Self::ATop | Self::ABot => 2,
            Self::BTop | Self::BBot => 4,
            Self::CTop | Self::CBot => 6,
            Self::DTop | Self::DBot => 8,
        }
    }

    fn dist_to_exit(self) -> u64 {
        match self {
            Self::ATop | Self::BTop | Self::CTop | Self::DTop => 1,
            Self::ABot | Self::BBot | Self::CBot | Self::DBot => 2,
        }
    }

    fn dist(self, hallway: Hallway) -> u64 {
        let exit = self.exit_pos();
        let hall = hallway.pos();
        self.dist_to_exit() + if exit > hall {
            exit as u64 - hall as u64
        } else {
            hall as u64 - exit as u64
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum RoomType {
    Top(Room),
    Bot(Room),
}

fn main() {
    // let starting_positions = [
    //     Position::Room(Room::ABot), Position::Room(Room::DBot),
    //     Position::Room(Room::ATop), Position::Room(Room::CTop),
    //     Position::Room(Room::BTop), Position::Room(Room::CBot),
    //     Position::Room(Room::BBot), Position::Room(Room::DTop),
    // ];

    let starting_positions = [
        Position::Room(Room::BBot), Position::Room(Room::DTop),
        Position::Room(Room::BTop), Position::Room(Room::DBot),
        Position::Room(Room::ABot), Position::Room(Room::CTop),
        Position::Room(Room::ATop), Position::Room(Room::CBot),
    ];

    println!("Part 1: {}", search(starting_positions, 0, Some(15356), &mut HashMap::new()).unwrap());
}

fn search(positions: [Position; 8], cost: u64, cost_bound: Option<u64>, visited: &mut HashMap<[Position; 8], u64>) -> Option<u64> {
    if let Some(cost_bound) = cost_bound {
        if cost >= cost_bound {
            return Some(cost_bound);
        }
    }
    
    if is_satisfied(positions) {
        return Some(cost);
    }

    if let Some(&c) = visited.get(&positions) {
        if c <= cost {
            return cost_bound;
        }
    }

    visited.insert(positions, cost);

    let mut best_cost = cost_bound;

    for (amphipod, pos) in positions.into_iter().enumerate() {
        match pos {
            Position::Hallway(hallway) => for room in Room::rooms_for(amphipod) {
                if !positions.into_iter().any(|p| p == Position::Room(room)) {
                    if !positions.into_iter().any(|p| match p {
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
                        let mut positions = positions;
                        positions[amphipod] = Position::Room(room);
                        let search_cost = search(positions, cost, best_cost, visited);
                        if let Some(search_cost) = search_cost {
                            match best_cost {
                                Some(best) if search_cost < best => best_cost = Some(search_cost),
                                None => best_cost = Some(search_cost),
                                _ => (),
                            }
                        }
                    }
                    break;
                }
            },

            Position::Room(room) => {
                let allowed = match room.opposite() {
                    RoomType::Top(top) => {
                        !positions.into_iter().any(|p| p == Position::Room(top))
                    },
                    RoomType::Bot(bot) => {
                        !room.is_room_for(amphipod) || positions.into_iter().enumerate().any(|(i, p)| {
                            p == Position::Room(bot) && !bot.is_room_for(i)
                        })
                    },
                };

                if allowed {
                    for hallway in Hallway::HALLWAYS {
                        if !positions.into_iter().any(|p| match p {
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
                            let mut positions = positions;
                            positions[amphipod] = Position::Hallway(hallway);
                            let search_cost = search(positions, cost, best_cost, visited);
                            if let Some(search_cost) = search_cost {
                                match best_cost {
                                    Some(best) if search_cost < best => best_cost = Some(search_cost),
                                    None => best_cost = Some(search_cost),
                                    _ => (),
                                }
                            }
                        }
                    }
                }
            },
        }
    }

    best_cost
}

fn is_satisfied(positions: [Position; 8]) -> bool {
    positions.into_iter().enumerate().all(|(amphipod, pos)| match pos {
        Position::Hallway(_) => false,
        Position::Room(room) => room.is_room_for(amphipod),
    })
}

fn movement_cost(amphipod: usize) -> u64 {
    match amphipod {
        0 | 1 => 1,
        2 | 3 => 10,
        4 | 5 => 100,
        _ => 1000,
    }
}
