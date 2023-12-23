mod part_1;
mod part_2;

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

pub const INPUT: &str = include_str!("../input");

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    return input.lines().map(|line| line.chars().collect()).collect();
}

pub fn navigate_loop<F>(start: (usize, usize), dir: (isize, isize), tiles: &Vec<Vec<char>>, mut f: F)
where
    F: FnMut(&(usize, usize)),
{
    f(&start);

    // Navigate the pipes all the way back to S, counting how long the whole loop is
    let mut last: (usize, usize) = (start.0, start.1);
    let mut current: (usize, usize) = (
        (start.0 as isize + dir.0) as usize,
        (start.1 as isize + dir.1) as usize,
    );

    f(&current);

    while current != start {
        f(&current);
        let possible_dirs = possible_directions(current, tiles[current.0][current.1]);

        let temp = current;
        current = *possible_dirs.iter().find(|c| **c != last).unwrap();
        last = temp;
    }
}

fn possible_directions(pos: (usize, usize), c: char) -> [(usize, usize); 2] {
    return match c {
        '|' => [(pos.0 - 1, pos.1), (pos.0 + 1, pos.1)],
        '-' => [(pos.0, pos.1 + 1), (pos.0, pos.1 - 1)],
        'L' => [(pos.0 - 1, pos.1), (pos.0, pos.1 + 1)],
        'J' => [(pos.0 - 1, pos.1), (pos.0, pos.1 - 1)],
        '7' => [(pos.0, pos.1 - 1), (pos.0 + 1, pos.1)],
        'F' => [(pos.0, pos.1 + 1), (pos.0 + 1, pos.1)],
        _ => panic!("Unexpected pipe"),
    };
}
