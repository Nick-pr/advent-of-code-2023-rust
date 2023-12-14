pub use part_1::solution as part_1;
// pub use part_2::solution as part_2;

mod char_counter;
use char_counter::CharCounter;

pub const INPUT: &str = include_str!("../input");

mod part_1 {
    use super::CharCounter;

    pub fn solution(input: &str) -> u32 {
        let mut char_counter = CharCounter::new();

        let chars: Vec<char> = input.chars().collect();

        for c in 0..3 {
            char_counter.inc(&chars[c]);
        }

        for i in 4..input.len() {
            char_counter.inc(&chars[i - 1]);
            if !char_counter.has_duplicates() {
                return i as u32;
            }
            char_counter.dec(&chars[i - 4]);
        }

        return 0;
    }
}
// mod part_2 {
//     pub fn solution() -> u32 {
//         return 0;
//     }
// }
