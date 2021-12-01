use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

const ROWS: i32 = 128;
const COLS : i32 = 8;

fn main() {
    let fd = File::open("input").expect("Failed to open file");
    let br = BufReader::new(fd);
    let mut max_id = 0;
    let mut seats = HashMap::new();
    for line in br.lines() {
        let line = line.expect("Failed to read line");
        let (row, col) = read_seat_code(line.trim(), ROWS, COLS).expect("Error reading seat code");
        let id = seat_id(row, col);
        if id > max_id {
            max_id = id;
        }
        seats.insert(id, (row, col));
    }
    println!("Max id: {}", max_id);
    let mut missing_ids = Vec::new();
    for id in 0..(max_id+1) {
        match seats.get(&id) {
            Some((_, _)) => (),
            None => {
                let (prev, next) = (id-1, id+1);
                if seats.contains_key(&prev) && seats.contains_key(&next) {
                    missing_ids.push(id);
                }
            },
        }
    }
    println!("Missing ids: {:?}", missing_ids)
}

fn read_seat_code(s: &str, rows: i32, cols: i32) -> Option<(i32, i32)> {
    let (mut min_row, mut max_row, mut min_col, mut max_col) = (0,rows,0,cols);
    for c in s.chars() {
        let (mid_row, mid_col) = ((min_row + max_row) / 2, (min_col + max_col) / 2);
        match c {
            'F' => max_row = mid_row,
            'B' => min_row = mid_row,
            'L' => max_col = mid_col,
            'R' => min_col = mid_col,
            _ => return None,
        }
    }
    if min_row + 1 == max_row && min_col + 1 == max_col {
        Some((min_row, min_col))
    } else {
        None
    }
}

fn seat_id(row: i32, col: i32) -> i32 {
    (row * COLS) + col
}
