struct School {
    fish: [u64; 9],
}

impl School {
    fn count(&self) -> u64 {
        self.fish.iter().copied().sum()
    }

    fn update(&mut self) {
        let spawned = self.fish[0];
        for i in 0..8 {
            self.fish[i] = self.fish[i + 1];
        }
        self.fish[6] += spawned;
        self.fish[8] = spawned;
    }
}

fn main() {
    let fish = include_str!("input")
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let mut initial = [0u64; 9];
    for f in fish {
        initial[f as usize] += 1;
    }

    let mut school = School { fish: initial };

    for _ in 0..80 {
        school.update()
    }

    println!("Part 1: {}", school.count());

    for _ in 80..256 {
        school.update();
    }

    println!("Part 2: {}", school.count());
}
