struct School {
    fish: [u64; 9],
}

impl School {
    fn count(&self) -> u64 {
        self.fish.iter().copied().sum()
    }

    fn update(&mut self) {
        let mut next = [0u64; 9];
        for i in 1..9 {
            next[i - 1] = self.fish[i];
        }
        next[6] += self.fish[0];
        next[8] = self.fish[0];
        for i in 0..9 {
            self.fish[i] = next[i];
        }
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
