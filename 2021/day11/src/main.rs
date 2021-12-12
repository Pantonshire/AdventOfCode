const SIZE: usize = 10;

fn main() {
    let mut grid = include_str!("input")
        .lines()
        .filter_map(|line| match line.trim() {
            s if s.is_empty() => None,
            s => Some(s),
        })
        .flat_map(|s| s.chars().map(|c| c as u8 - 0x30).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    
    let mut n = 0;
    for _ in 0..100 {
        let (add_n, new_grid) = step(&grid);
        n += add_n;
        grid = new_grid;
    }

    println!("{}", n);

    let mut i = 100;
    loop {
        let (add_n, new_grid) = step(&grid);
        n += add_n;
        grid = new_grid;
        if add_n == (SIZE * SIZE) as u64 {
            println!("{}", i + 1);
            break;
        }
        i += 1;
    }
}

fn step(grid: &[u8]) -> (u64, Vec<u8>) {
    let mut new_grid = Vec::with_capacity(grid.len());

    let mut n_flashes = 0;
    let mut flashes = Vec::new();

    for (i, o) in grid.iter().copied().enumerate() {
        let e = o + 1;
        new_grid.push(e);
        if e > 9 {
            flashes.push(i);
            n_flashes += 1;
        }
    }

    while !flashes.is_empty() {
        let old_flashes = flashes.clone();
        flashes = Vec::new();
        for i in old_flashes.into_iter() {
            let neighbours = [
                ((i.wrapping_sub(1)), if i % SIZE != 0 { new_grid.get(i.wrapping_sub(1)).copied() } else { None }),
                ((i + 1), if i % SIZE != 9 { new_grid.get(i + 1).copied() } else { None }),
                ((i.wrapping_sub(SIZE)), new_grid.get(i.wrapping_sub(SIZE)).copied()),
                ((i + SIZE), new_grid.get(i + SIZE).copied()),
                ((i.wrapping_sub(SIZE).wrapping_sub(1)), if i % SIZE != 0 { new_grid.get(i.wrapping_sub(SIZE).wrapping_sub(1)).copied() } else { None }),
                ((i.wrapping_sub(SIZE).wrapping_add(1)), if i % SIZE != 9 { new_grid.get(i.wrapping_sub(SIZE).wrapping_add(1)).copied() } else { None }),
                ((i + SIZE - 1), if i % SIZE != 0 { new_grid.get(i + SIZE - 1).copied() } else { None }),
                ((i + SIZE + 1), if i % SIZE != 9 { new_grid.get(i + SIZE + 1).copied() } else { None })
            ];

            for (j, neighbour) in neighbours.into_iter() {
                if let Some(neighbour) = neighbour {
                    if neighbour <= 9 {
                        let e = neighbour + 1;
                        new_grid[j] = e;
                        if e > 9 {
                            flashes.push(j);
                            n_flashes += 1;
                        }
                    }
                }
            }
        }
    }

    for o in new_grid.iter_mut() {
        if *o > 9 {
            *o = 0;
        }
    }

    (n_flashes, new_grid)
}
