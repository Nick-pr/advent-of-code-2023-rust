use crate::{parse_input, Direction};
use num::integer::lcm;

pub fn solution(input: &str) -> u64 {
    let (directions, nodes) = parse_input(input);

    let steps: Vec<u64> = nodes
        .keys()
        .filter(|node| node.ends_with("A"))
        .map(|node| {
            let mut steps: u64 = 0;
            let mut current = *node;

            while !current.ends_with("Z") {
                for direction in directions.iter() {
                    let (left, right) = nodes.get(current).unwrap();
                    current = match direction {
                        Direction::Right => right,
                        Direction::Left => left,
                    };
                    steps += 1;
                }
            }
            steps
        })
        .collect();

    return lcm_vec(steps);
}

fn lcm_vec(nums: Vec<u64>) -> u64 {
    let mut result: u64 = lcm(nums[0], nums[1]);

    for num in &nums[2..] {
        result = lcm(result, *num);
    }
    return result;
}
