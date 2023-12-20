pub use part_1::solution as part_1;

mod part_1 {
    use crate::parse_input;

    pub fn solution(input: &str) -> u64 {
        let mut hands = parse_input(input);
        hands.sort();

        let mut result: u64 = 0;

        for (i, (_, bid)) in hands.into_iter().enumerate() {
            result += bid as u64 * (i as u64 + 1)
        }
        return result;
    }
}

// mod part_2 {
//     pub fn solution(input: &str) -> u32 {

//     }
// }
