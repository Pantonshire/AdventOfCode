use std::collections::HashSet;

fn main() {
    let input = include_str!("input");
    let (enhance, image) = input.split_once("\n\n").unwrap();

    let enhance = enhance.trim().chars()
        .filter_map(|c| match c {
            c if c.is_ascii_whitespace() => None,
            '#' => Some(true),
            _ => Some(false),
        }).collect::<Vec<_>>();

    let image = image.lines().filter_map(|line| match line.trim() {
        l if l.is_empty() => None,
        l => Some(l.chars().map(|c| match c {
            '#' => true,
            _ => false,
        }).collect::<Vec<_>>())
    }).collect::<Vec<_>>();

    let mut pixels = image.into_iter().enumerate()
        .map(|(j, row)| row.into_iter().enumerate()
            .filter_map(move |(i, pixel)| pixel.then(|| (i as i64, j as i64))))
        .flatten()
        .collect::<HashSet<_>>();

    pixels = step(&pixels, &enhance, false);
    pixels = step(&pixels, &enhance, true);

    println!("Part 1: {}", pixels.len());

    for inverted in [false, true].into_iter().cycle().take(48) {
        pixels = step(&pixels, &enhance, inverted);
    }

    println!("Part 2: {}", pixels.len());
}

macro_rules! unwrap_or_return {
    ($option:expr, $default:expr) => {
        match $option {
            Some(x) => x,
            None => return $default,
        }
    };
}

fn step(pixels: &HashSet<(i64, i64)>, enhance: &[bool], inverted: bool) -> HashSet<(i64, i64)> {
    let mut next_pixels = HashSet::new();

    let min_x = unwrap_or_return!(pixels.iter().copied().map(|(x, _)| x).min(), next_pixels);
    let min_y = unwrap_or_return!(pixels.iter().copied().map(|(_, y)| y).min(), next_pixels);
    let max_x = unwrap_or_return!(pixels.iter().copied().map(|(x, _)| x).max(), next_pixels);
    let max_y = unwrap_or_return!(pixels.iter().copied().map(|(_, y)| y).max(), next_pixels);

    for x in min_x-1..=max_x+1 {
        for y in min_y-1..=max_y+1 {
            let mut n = 0u16;
            for ry in 0..3 {
                for rx in 0..3 {
                    let previous_contained= pixels.contains(&(x + rx - 1, y + ry - 1));
                    if inverted != previous_contained {
                        n |= 1 << (8 - (ry * 3 + rx));
                    }
                }
            }

            if inverted == enhance[n as usize] {
                next_pixels.insert((x, y));
            }
        }
    }

    next_pixels
}
