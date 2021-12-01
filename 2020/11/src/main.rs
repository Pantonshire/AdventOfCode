type Grid = Vec<Vec<Cell>>;

#[derive(Copy, Clone)]
enum Cell {
    Floor,
    Empty,
    Taken,
}

impl Cell {
    fn is_floor(&self) -> bool {
        match self {
            Cell::Floor => true,
            _ => false,
        }
    }
    fn is_taken(&self) -> bool {
        match self {
            Cell::Taken => true,
            _ => false,
        }
    }
}

const DIRS: [(i32,i32); 8] = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];

fn main() {
    let contents = include_str!("../input");
    let grid: Grid = contents.split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars()
            .map(|c| read_cell(c).expect("Error reading cell"))
            .collect())
        .collect();
    {
        let mut grid1 = grid.clone();

        loop {
            match evolve(&grid1, adjacent_neighbours, 4) {
                None => break,
                Some(g) => grid1 = g,
            }
        }

        let taken: usize = grid1.iter().map(|row| row.iter().filter(|c| c.is_taken()).count()).sum();
        println!("Part 1: {}", taken);
    }
    {
        let mut grid2 = grid.clone();

        loop {
            match evolve(&grid2, visible_neighbours, 5) {
                None => break,
                Some(g) => grid2 = g,
            }
        }

        let taken: usize = grid2.iter().map(|row| row.iter().filter(|c| c.is_taken()).count()).sum();
        println!("Part 2: {}", taken);
    }
}

fn read_cell(c: char) -> Option<Cell> {
    match c {
        '.' => Some(Cell::Floor),
        'L' => Some(Cell::Empty),
        '#' => Some(Cell::Taken),
        _ => None,
    }
}

fn adjacent_neighbours(_: &Grid, (i,j): (usize,usize), (max_i,max_j): (usize,usize)) -> Vec<(usize,usize)> {
    let i_range = 0..(max_i as i32);
    let j_range = 0..(max_j as i32);
    DIRS.iter()
        .map(|(x,y)| (i as i32 + x, j as i32 + y))
        .filter(|(x,y)| i_range.contains(&x) && j_range.contains(&y))
        .map(|(x,y)| (x as usize, y as usize))
        .collect()
}

fn visible_neighbours(grid: &Grid, (i,j): (usize,usize), (max_i,max_j): (usize,usize)) -> Vec<(usize,usize)> {
    let i_range = 0..(max_i as i32);
    let j_range = 0..(max_j as i32);
    let mut neighbours = Vec::new();
    for (dx, dy) in DIRS.iter() {
        let (mut x, mut y) = (i as i32 + dx, j as i32 +dy);
        while i_range.contains(&x) && j_range.contains(&y) && grid[x as usize][y as usize].is_floor() {
            x += dx;
            y += dy;
        }
        if i_range.contains(&x) && j_range.contains(&y) {
            neighbours.push((x as usize, y as usize));
        }
    }
    neighbours
}

fn evolve<F>(grid: &Grid, neighbour_fn: F, leave_threshold: usize) -> Option<Grid>
where
    F: Fn(&Grid, (usize,usize), (usize,usize)) -> Vec<(usize,usize)>
{
    let mut changed = false;
    let mut evolved_grid = grid.clone();
    let max_i = grid.len();
    for (i,row) in grid.iter().enumerate() {
        let max_j = row.len();
        for (j,cell) in row.iter().enumerate() {
            if cell.is_floor() {
                continue;
            }
            let neighbours_taken = neighbour_fn(grid, (i,j), (max_i,max_j))
                .iter()
                .filter(|&p| grid[p.0][p.1].is_taken())
                .count();
            if cell.is_taken() {
                if neighbours_taken >= leave_threshold {
                    evolved_grid[i][j] = Cell::Empty;
                    changed = true;
                }
            } else {
                if neighbours_taken == 0 {
                    evolved_grid[i][j] = Cell::Taken;
                    changed = true;
                }
            }
        }
    }
    if changed {
        Some(evolved_grid)
    } else {
        None
    }
}
