mod almanac;

use almanac::*;
pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

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
mod part_2 {
    use super::Almanac;
    use std::ops::Range;
    pub fn solution(input: &str) -> u64 {
        let almanac = Almanac::from(input);
        let mut min: u64 = u64::MAX;

        let mut possible_locations: Vec<&(Range<u64>, u64)> =
            almanac.humidity_to_location.ranges().collect();

        possible_locations
            .sort_by(|(_, dest_start), (_, dest_start_b)| dest_start.cmp(dest_start_b));

        for (src_range, dest_start) in possible_locations {
            for i in *dest_start..*dest_start + (src_range.end - src_range.start) {
                let seed = almanac.location_to_seed(i);
                if seed.is_none() {
                    continue;
                }
                return i
            }
        }
        return 0;
    }
}
