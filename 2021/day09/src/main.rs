use std::collections::HashSet;

fn main() {
    let input = include_str!("input")
        .lines()
        .flat_map(|line| match line.trim() {
            s if s.is_empty() => None,
            s => Some(s.as_bytes()),
        })
        .collect::<Vec<_>>();
    
    let mut risk: u64 = 0;

    let mut low_points = Vec::new();

    for (i, line) in input.iter().enumerate() {
        for (j, x) in line.iter().enumerate() {
            let is_low = neighbours(&input, i, j)
                .into_iter()
                .flatten()
                .all(|(neighbour, _, _)| *x < neighbour);
            
            if is_low {
                risk += (*x - 0x30) as u64 + 1;
                low_points.push((i, j));
            }
        }
    }

    println!("Part 1: {}", risk);

    let mut basin_sizes = Vec::new();

    for low_point in low_points {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        let mut basin_size = 0u64;
        stack.push(low_point);
        while let Some((i, j)) = stack.pop() {
            let current = input[i][j];
            if current as char != '9' && !visited.contains(&(i, j)) {
                basin_size += 1;
                visited.insert((i, j));
                for (neighbour, ni, nj) in neighbours(&input, i, j).into_iter().flatten() {
                    if neighbour > current {
                        stack.push((ni, nj))
                    }
                }
            }
        }
        basin_sizes.push(basin_size);
    }

    basin_sizes.sort_unstable();
    println!("Part 2: {}", basin_sizes.into_iter().rev().take(3).product::<u64>());
}

fn neighbours(map: &[&[u8]], i: usize, j: usize) -> [Option<(u8, usize, usize)>; 4] {
    let above = (i.checked_sub(1)).and_then(|k| map.get(k)
        .and_then(|line| line.get(j).cloned().map(|x| (x, k, j))));

    let below = map.get(i + 1)
        .and_then(|line| line.get(j).cloned().map(|x| (x, i + 1, j)));

    let left = (j.checked_sub(1)).and_then(|k| map.get(i)
        .and_then(|line| line.get(k).cloned().map(|x| (x, i, k))));

    let right = map.get(i)
        .and_then(|line| line.get(j + 1).cloned().map(|x| (x, i, j + 1)));
    
    [above, below, left, right]
}
