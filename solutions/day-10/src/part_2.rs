use crate::{navigate_loop, parse_input};
use std::collections::HashSet;

pub fn solution(input: &str) -> u64 {
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

/// This solution utilizes [Pick's Theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem) and the
/// [Shoelace Forumula](https://en.wikipedia.org/wiki/Shoelace_formula) to derive the answer.
///
/// Pick's Theorem states A = i + (b/2) -1 where A = The area, B = the number or boundry
/// points, and I = the number of interior points.
///
/// We know b, the length of the loop, and we can calcuate A utilizing
/// the shoelace formula. Thus we can calculate I, which is ultimately what we are after.
pub fn solution_shoelace_picks(input: &str) -> u64 {
    let tiles = parse_input(input);

    let s_pos: (usize, usize) = (60, 75);

    // Shoelace formula expects a CONSECUTIVE list of points ending back at where you started.
    // So if you had point A, B, C, D, the forumla would expect A,B,C,D,A.
    let mut points: Vec<(usize, usize)> = Vec::new();
    navigate_loop(s_pos, (1, 0), &tiles, |curr| {
        points.push(*curr);
    });
    points.push(s_pos);

    // Because we appended the start point to the points vector for the Shoelace forumla, we must
    // subtract one to calculate the loop length, which will be B in Pick's Theorem.
    let loop_length = points.len() - 1;

    return (shoelace_formula(&points) + 1_f64 - (loop_length as f64 / 2_f64)).ceil() as u64;
}

fn shoelace_formula(points: &Vec<(usize, usize)>) -> f64 {
    let mut result: i64 = 0;
    for pairs in points.windows(2) {
        let (x1, y1) = (pairs[0].0 as i64, pairs[0].1 as i64);
        let (x2, y2) = (pairs[1].0 as i64, pairs[1].1 as i64);

        result += (x2 * y1) - (x1 * y2)
    }

    return (result as f64 / 2 as f64).abs();
}
