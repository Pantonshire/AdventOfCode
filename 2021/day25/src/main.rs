#[derive(Copy, Clone)]
enum Space {
    Empty,
    East,
    South,
}

impl Default for Space {
    fn default() -> Self {
        Self::Empty
    }
}

const GRID_WIDTH: usize = 139;
const GRID_HEIGHT: usize = 137;
// const GRID_WIDTH: usize = 10;
// const GRID_HEIGHT: usize = 9;

type Grid = [[Space; GRID_HEIGHT]; GRID_WIDTH];

fn main() {
    let input = include_str!("input")
        .lines()
        .filter_map(|line| match line {
            l if l.is_empty() => None,
            l => Some(l.chars().map(|c| match c {
                '>' => Space::East,
                'v' => Space::South,
                _ => Space::Empty,
            })
            .collect::<Vec<_>>()),
        })
        .collect::<Vec<_>>();

    let mut grid = [[Space::Empty; GRID_HEIGHT]; GRID_WIDTH];

    for (y, row) in input.into_iter().enumerate() {
        for (x, space) in row.into_iter().enumerate() {
            grid[x][y] = space;
        }
    }

    let mut steps = 0u64;

    while let Some(next_grid) = step(grid) {
        grid = next_grid;
        steps += 1;
    }

    println!("Part 1: {}", steps + 1);
}

fn step(grid: Grid) -> Option<Grid> {
    let mut east_grid = [[Space::Empty; GRID_HEIGHT]; GRID_WIDTH];
    let mut east_moved = false;
    for (x, col) in grid.iter().enumerate() {
        for (y, space) in col.iter().copied().enumerate() {
            let right = (x + 1) % GRID_WIDTH;
            if matches!(space, Space::East) && matches!(grid[right][y], Space::Empty) {
                east_grid[right][y] = Space::East;
                east_moved = true;
            } else if !matches!(space, Space::Empty) {
                east_grid[x][y] = space;
            }
        }
    }

    let mut south_grid = [[Space::Empty; GRID_HEIGHT]; GRID_WIDTH];
    let mut south_moved = false;
    for (x, col) in east_grid.iter().enumerate() {
        for (y, space) in col.iter().copied().enumerate() {
            let down = (y + 1) % GRID_HEIGHT;
            if matches!(space, Space::South) && matches!(col[down], Space::Empty) {
                south_grid[x][down] = Space::South;
                south_moved = true;
            } else if !matches!(space, Space::Empty) {
                south_grid[x][y] = space;
            }
        }
    }

    if east_moved || south_moved {
        Some(south_grid)
    } else {
        None
    }
}
