use crate::{parse_input, Direction};

pub fn solution(input: &str) -> u64 {
    let (directions, nodes) = parse_input(input);

    let mut current_node = "AAA";
    let mut steps = 0;

    while current_node != "ZZZ" {
        for direction in directions.iter() {
            let (left, right) = nodes.get(current_node).unwrap();
            current_node = match direction {
                Direction::Right => right,
                Direction::Left => left,
            };
            steps += 1;
        }
    }

    return steps;
}
