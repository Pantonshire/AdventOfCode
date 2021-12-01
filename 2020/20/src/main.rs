use std::fmt::{Display, Formatter, Result};
use std::collections::HashMap;

fn main() {
    let contents = include_str!("../input");

    let tiles = contents.split("\n\n")
        .map(|ls| ls.split("\n")
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect::<Vec<&str>>())
        .map(|ls| read_tile(&ls))
        .collect::<Option<Vec<Tile>>>()
        .expect("Error reading tiles");

    let mut corners = Vec::new();

    for t1 in tiles.iter() {
        let mut n = 0;
        let mut bs1 = borders(&t1.img);
        bs1.extend(bs1.clone());
        for b in bs1.iter_mut().skip(4) {
            b.reverse();
        }
        for t2 in tiles.iter() {
            if t1.id == t2.id {
                continue;
            }
            let bs2 = borders(&t2.img);
            if bs1.iter().any(|b1| bs2.contains(b1)) {
                n += 1;
            }
        }
        if n == 2 {
            corners.push(t1.id);
        }
    }

    assert_eq!(corners.len(), 4);

    println!("Part 1: {}", corners.iter().product::<u64>());
    println!("{:?}", corners);

    // let h = (tiles.len() as f64).sqrt() as usize;
    // let w = h;

    // let mut trs = HashMap::new();
    // let mut non_corners = Vec::new();
    // for t in tiles {
    //     trs.insert(t.id, transformations(&t.img));
    //     if !corners.contains(&t.id) {
    //         non_corners.push(t.id);
    //     }
    // }

    // let grid = assemble_tile_grid(h, w, 0, &Vec::new(), &non_corners, &trs, (corners[0], corners[1], corners[2], corners[3]));

    // assert_ne!(grid, None);
}

type Image = Vec<Vec<Pixel>>;

#[derive(Clone)]
struct Tile {
    id: u64,
    img: Image,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Pixel {
    Dot,
    Hash,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Pixel::Dot  => write!(f, "{}", '.'),
            Pixel::Hash => write!(f, "{}", '#'),
        }
    }
}

fn read_tile(ls: &Vec<&str>) -> Option<Tile> {
    let mut ls = ls.iter();
    let tile_id = ls.next()?
        .strip_prefix("Tile")?
        .strip_suffix(":")?
        .trim()
        .parse::<u64>().ok()?;
    let tile = ls.map(|l| l.chars()
            .map(read_pixel)
            .collect())
        .collect::<Option<Image>>()?;
    Some(Tile{
        id: tile_id,
        img: tile,
    })
}

fn read_pixel(c: char) -> Option<Pixel> {
    match c {
        '.' => Some(Pixel::Dot),
        '#' => Some(Pixel::Hash),
        _   => None,
    }
}

//TODO: I think this needs a "possible adjacency" cache to be performant (rcs and bcs in main.py)
fn assemble_tile_grid(h: usize, w: usize, i: usize, grid: &Vec<(u64, usize)>, tiles: &Vec<u64>, trs: &HashMap<u64, Vec<Image>>, corners: (u64, u64, u64, u64)) -> Option<Vec<(u64, usize)>> {
    if i >= w * h {
        return Some(grid.clone());
    }
    let col = i % w;
    let row = i / w;
    let left = if row > 0 { grid.get(i-1) } else { None };
    let top = if i >= w { grid.get(i-w) } else { None };
    let corner = if row == 0 && col == 0 {
        Some(corners.0)
    } else if row == 0 && col == w-1 {
        Some(corners.1)
    } else if row == h-1 && col == 0 {
        Some(corners.2)
    } else if row == h-1 && col == w-1 {
        Some(corners.3)
    } else {
        None
    };
    let mut corner_vec = vec![0];
    let possible_tiles = match corner {
        Some(t) => {
            corner_vec[0] = t;
            &corner_vec
        },
        None => tiles,
    };
    for t_id in possible_tiles {
        let mut tiles = tiles.clone();
        tiles.retain(|t| t != t_id);
        for (tr_id, t) in trs[t_id].iter().enumerate() {
            match top {
                Some(top) => if !matches_v(&trs[&top.0][top.1], t) {
                    continue;
                },
                None => (),
            }
            match left {
                Some(left) => if !matches_h(&trs[&left.0][left.1], t) {
                    continue;
                },
                None => (),
            }
            let mut grid = grid.clone();
            grid.push((*t_id, tr_id));
            match assemble_tile_grid(h, w, i+1, &grid, &tiles, trs, corners) {
                Some(grid) => return Some(grid),
                None => (),
            }
        }
    }
    None
}

fn borders(img: &Image) -> Vec<Vec<Pixel>> {
    let img_trn = transpose(img);
    vec![img[0].clone(), img.last().unwrap().clone(), img_trn[0].clone(), img_trn.last().unwrap().clone()]
}

fn matches_h(left: &Image, right: &Image) -> bool {
    let left_trn = transpose(left);
    let right_trn = transpose(right);
    return left_trn.last().unwrap().eq(&right_trn[0])
}

fn matches_v(top: &Image, bottom: &Image) -> bool {
    return top.last().unwrap().eq(&bottom[0])
}

fn transformations(img: &Image) -> Vec<Image> {
    let mut ts = Vec::new();
    ts.extend(rotations(&img));
    ts.extend(rotations(&flip_h(img)));
    ts.extend(rotations(&flip_v(img)));
    ts
}

fn rotations(img: &Image) -> Vec<Image> {
    let r1 = rotate(img);
    let r2 = rotate(&r1);
    let r3 = rotate(&r2);
    vec![img.clone(), r1, r2, r3]
}

fn flip_h(img: &Image) -> Image {
    let mut result = img.clone();
    for row in result.iter_mut() {
        row.reverse();
    }
    result
}

fn flip_v(img: &Image) -> Image {
    let mut result = img.clone();
    result.reverse();
    result
}

fn rotate(img: &Image) -> Image {
    let mut result = transpose(img);
    for row in result.iter_mut() {
        row.reverse();
    }
    result
}

fn transpose(img: &Image) -> Image {
    let mut result = vec![vec![Pixel::Dot; img.len()]; img[0].len()];
    for (i,row) in img.iter().enumerate() {
        for (j,pix) in row.iter().enumerate() {
            result[j][i] = *pix;
        }
    }
    result
}

fn print_img(img: &Image) {
    for row in img {
        for pix in row {
            print!("{}", pix);
        }
        println!();
    }
}
