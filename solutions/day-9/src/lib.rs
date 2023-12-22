mod part_1;
mod part_2;

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

pub const INPUT: &str = include_str!("../input");

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    let mut result: Vec<Vec<i64>> = Vec::with_capacity(21);
    for line in input.lines() {
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        result.push(nums);
    }
    return result;
}
