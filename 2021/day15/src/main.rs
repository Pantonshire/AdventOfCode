use std::collections::{HashSet, HashMap};
use std::collections::hash_map::Entry;

fn main() {
    let input = include_str!("input")
        .lines()
        .filter_map(|line| match line.trim() {
            s if s.is_empty() => None,
            s => Some(s),
        })
        .map(|s| s.chars().map(|c| c as u8 - 0x30).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let nodes = input
        .iter()
        .map(|row| row.as_ref())
        .collect::<Vec<&[_]>>();

    let small_dist = dijkstra(&nodes, (0, 0), (nodes.len() - 1, nodes[0].len() - 1), node_at).unwrap();
    println!("Part 1: {}", small_dist);

    let large_dist = dijkstra(&nodes, (0, 0), (nodes.len() * 5 - 1, nodes[0].len() * 5 - 1), node_at_wrapping).unwrap();
    println!("Part 2: {}", large_dist);
}

fn dijkstra<F>(nodes: &[&[u8]], start: (usize, usize), target: (usize, usize), distance_fn: F) -> Option<u64>
where
    F: Fn(&[&[u8]], Option<usize>, Option<usize>) -> Option<((usize, usize), u8)>
{
    let mut queue = Vec::new();
    queue.push(start);

    let mut distances = HashMap::new();
    distances.insert(start, 0u64);

    let mut visited = HashSet::new();

    while let Some((curr_i, curr_dist)) = queue
        .iter()
        .enumerate()
        .map(|(i, node)| (i, *distances.get(node).unwrap()))
        .min_by(|(_, d1), (_, d2)| d1.cmp(d2))
    {
        let (row, col) = queue.remove(curr_i);
        visited.insert((row, col));
        if (row, col) == target {
            return Some(curr_dist);
        }
        let neighbours = [
            distance_fn(&nodes, row.checked_sub(1), Some(col)),
            distance_fn(&nodes, Some(row), col.checked_sub(1)),
            distance_fn(&nodes, Some(row + 1), Some(col)),
            distance_fn(&nodes, Some(row), Some(col + 1)),
        ];
        for ((nrow, ncol), risk) in neighbours.into_iter().flatten() {
            if !visited.contains(&(nrow, ncol)) {
                let new_dist = risk as u64 + curr_dist;
                match distances.entry((nrow, ncol)) {
                    Entry::Occupied(mut o) => {
                        let old_dist = o.get_mut();
                        if new_dist < *old_dist {
                            *old_dist = new_dist;
                        }
                    },
                    Entry::Vacant(v) => {
                        v.insert(new_dist);
                    },
                }
                if !queue.contains(&(nrow, ncol)) {
                    queue.push((nrow, ncol));
                }
            }
        }
    }
    None
}

fn node_at(nodes: &[&[u8]], row: Option<usize>, col: Option<usize>) -> Option<((usize, usize), u8)> {
    let (row, col) = (row?, col?);
    Some(((row, col), *nodes.get(row)?.get(col)?))
}

fn node_at_wrapping(nodes: &[&[u8]], row: Option<usize>, col: Option<usize>) -> Option<((usize, usize), u8)> {
    let (row, col) = (row?, col?);
    let tile_row = row / nodes.len();
    let row_in_tile = row % nodes.len();
    if tile_row >= 5 {
        return None;
    }
    let tile_col = col / nodes[0].len();
    let col_in_tile = col % nodes[0].len();
    if tile_col >= 5 {
        return None;
    }
    let base_risk = *nodes.get(row_in_tile)?.get(col_in_tile)?;
    let risk = wrap(base_risk as u8 + tile_row as u8 + tile_col as u8, 1, 10);
    Some(((row, col), risk))
}

fn wrap(x: u8, start: u8, modulo: u8) -> u8 {
    (x - start) % (modulo - start) + start
}
