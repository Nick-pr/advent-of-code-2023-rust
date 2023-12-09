pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

pub const INPUT: &str = include_str!("../input");

mod part_1 {
    use std::collections::HashSet;

    #[derive(Debug)]
    pub struct ScratchCard {
        pub card_num: u32,
        pub winning_nums: HashSet<u32>,
        pub nums: Box<[u32]>,
    }

    impl ScratchCard {
        pub fn from_line(line: &str) -> ScratchCard {
            let colon_offset = line.find(":").unwrap();
            let num_divider_offset = line.find("|").unwrap();

            let card_num: u32 = line[4..colon_offset].trim().parse().unwrap();

            let winning_nums: HashSet<u32> = line[colon_offset + 2..num_divider_offset - 1]
                .split_whitespace()
                .map(|num| num.trim().parse::<u32>().unwrap())
                .collect();

            let nums: Box<[u32]> = line[num_divider_offset + 1..]
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            return ScratchCard {
                card_num,
                nums,
                winning_nums,
            };
        }
    }

    pub fn solution(input: &str) -> u32 {
        let mut total_points = 0;
        for line in input.lines() {
            let scratch_pad = ScratchCard::from_line(line);

            let mut nums_iter = scratch_pad.nums.iter();
            let mut card_points = 0;

            while let Some(num) = nums_iter.next() {
                if scratch_pad.winning_nums.contains(num) {
                    card_points = 1;
                    break;
                }
            }

            for num in nums_iter {
                if scratch_pad.winning_nums.contains(num) {
                    card_points *= 2;
                }
            }

            total_points += card_points;
        }

        return total_points;
    }
}
mod part_2 {
    use super::part_1::ScratchCard;
    pub fn solution(input: &str) -> u32 {
        let mut copies: [u32; 220] = [1; 220];

        for scratch_card in input.lines().map(|line| ScratchCard::from_line(line)) {
            let num_matching = calculate_number_of_matching_numbers(&scratch_card);

            for i in 1..=num_matching {
                if scratch_card.card_num + i >= 220 {
                    break;
                }
                copies[(scratch_card.card_num + i) as usize] +=
                    copies[scratch_card.card_num as usize];
            }
        }

        return copies.iter().sum::<u32>() - 1;
    }

    fn calculate_number_of_matching_numbers(scratch_card: &ScratchCard) -> u32 {
        let mut matches = 0;

        for num in scratch_card.nums.iter() {
            if scratch_card.winning_nums.contains(num) {
                matches += 1;
            };
        }

        return matches;
    }
}
