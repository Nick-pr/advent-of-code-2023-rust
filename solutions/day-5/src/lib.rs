mod almanac;

use almanac::*;
pub use part_1::solution as part_1;
// pub use part_2::solution as part_2;

pub const INPUT: &str = include_str!("../input");

mod part_1 {
    use super::Almanac;
    pub fn solution(input: &str) -> u64 {
        let almanac = Almanac::from(input);

        return almanac
            .seeds()
            .map(|seed| almanac.seed_to_location(*seed))
            .min()
            .unwrap();
    }
}
// mod part_2 {
//     fn solution(input: &str) -> u64 {
//         return 0
//     }
// }
