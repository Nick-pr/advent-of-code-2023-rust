use crate::{navigate_loop, parse_input};

pub fn solution(input: &str) -> i64 {
    let tiles = parse_input(input);

    let s_pos: (usize, usize) = (60, 75);

    // Navigate the pipes all the way back to S, counting how long the whole loop is
    let mut steps = 0;
    navigate_loop(s_pos, (-1, 0), &tiles, |_| steps += 1);

    return (steps- 1) / 2;
}
