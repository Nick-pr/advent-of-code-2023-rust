use crate::{navigate_loop, parse_input};
use std::collections::HashSet;

pub fn solution(input: &str) -> i64 {
    let tiles = parse_input(input);

    let s_pos: (usize, usize) = (60, 75);

    let mut seen = HashSet::new();

    navigate_loop(s_pos, (1, 0), &tiles, |curr| {
        seen.insert(*curr);
    });

    let mut count = 0;
    for (i, row) in tiles.iter().enumerate() {
        let mut inside = false;
        let mut last_corner = '.';
        for (j, tile) in row.iter().enumerate() {
            if seen.get(&(i, j)).is_some() {
                match tile {
                    '|' => inside = !inside,
                    'F' | 'L' => last_corner = *tile,
                    'J' if last_corner == 'F' => inside = !inside,
                    '7' if last_corner == 'L' => inside = !inside,
                    _ => (),
                };
            } else if inside {
                count += 1;
            }
        }
    }

    return count;
}
