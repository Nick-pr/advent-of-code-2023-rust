mod part_1;
mod part_2;

use std::collections::HashMap;

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

pub const INPUT: &str = include_str!("../input");

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        return match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unexpected direction"),
        };
    }
}

pub fn parse_input(input: &str) -> (Vec<Direction>, HashMap<&str, (&str, &str)>) {
    let (directions_str, nodes_str) = input.split_once("\n\n").unwrap();

    let mut directions: Vec<Direction> = Vec::new();
    for direction in directions_str.chars() {
        directions.push(Direction::from(direction));
    }

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in nodes_str.lines() {
        let (node, neighbors) = line.split_once(" = ").unwrap();
        let (left_node, right_node) = neighbors[1..neighbors.len() - 1].split_once(", ").unwrap();
        nodes.insert(node, (left_node, right_node));
    }

    return (directions, nodes);
}
