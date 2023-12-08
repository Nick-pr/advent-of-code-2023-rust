pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

pub const INPUT: &str = include_str!("../input");

mod part_1 {
    type EngineSchematic = [[char; 140]; 140];
    type Seen = [[bool; 140]; 140];

    pub fn solution(input: &str) -> u32 {
        let mut engine_schematic: EngineSchematic = [['.'; 140]; 140];
        let mut seen: [[bool; 140]; 140] = [[false; 140]; 140];

        for (i, line) in input.lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                engine_schematic[i][j] = char
            }
        }

        let mut result: u32 = 0;
        for (i, row) in engine_schematic.iter().enumerate() {
            let gears = row
                .iter()
                .enumerate()
                .filter(|(_, c)| c.is_ascii_punctuation() && **c as u8 != 46)
                .map(|(j, _)| j);

            for j in gears {
                let nums = explore_gear(&engine_schematic, &mut seen, i, j);
                // println!("{},{} -> {:?}", i, j, nums);
                result += nums.iter().sum::<u32>();

                // Reset seen array
                for row in seen.iter_mut() {
                    for col in row.iter_mut() {
                        *col = false
                    }
                }
            }
        }
        return result;
    }

    fn explore_gear(
        engine_schematic: &EngineSchematic,
        mut seen: &mut Seen,
        i: usize,
        j: usize,
    ) -> Vec<u32> {
        let directions = [
            [-1, -1],
            [-1, 0],
            [-1, 1],
            [0, 1],
            [1, 1],
            [1, 0],
            [1, -1],
            [0, -1],
        ];

        let mut result = vec![];
        for [i_offest, j_offset] in directions {
            #[rustfmt::skip]
            let i = (i as isize + i_offest) as usize;
            let j = (j as isize + j_offset) as usize;

            if !(0..140).contains(&j) || !(0..140).contains(&i) {
                continue;
            }

            let neighbor = engine_schematic[i][j];

            if neighbor.is_ascii_digit() && !seen[i][j] {
                let num = explore_digit(&engine_schematic, &mut seen, i, j);
                result.push(num);
            }
        }

        return result;
    }

    fn explore_digit(
        engine_schematic: &EngineSchematic,
        seen: &mut Seen,
        i: usize,
        mut j: usize,
    ) -> u32 {
        while (0..=138).contains(&j) && engine_schematic[i][j + 1].is_ascii_digit() {
            j += 1
        }

        let right = j;

        let mut result = engine_schematic[i][j].to_digit(10).unwrap();
        let mut radix = 10;
        while (1..=139).contains(&j) && engine_schematic[i][j - 1].is_ascii_digit() {
            j -= 1;

            result += engine_schematic[i][j].to_digit(10).unwrap() * radix;
            radix *= 10;
        }

        for k in j..=right {
            seen[i][k] = true;
        }
        return result;
    }
}
mod part_2 {
    type EngineSchematic = [[char; 140]; 140];
    type Seen = [[bool; 140]; 140];

    pub fn solution(input: &str) -> u32 {
        let mut engine_schematic: EngineSchematic = [['.'; 140]; 140];
        let mut seen: [[bool; 140]; 140] = [[false; 140]; 140];

        for (i, line) in input.lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                engine_schematic[i][j] = char
            }
        }

        let mut result: u32 = 0;
        for (i, row) in engine_schematic.iter().enumerate() {
            let gears = row
                .iter()
                .enumerate()
                .filter(|(_, c)| **c as u8 == 42)
                .map(|(j, _)| j);

            for j in gears {
                let nums = explore_gear(&engine_schematic, &mut seen, i, j);
                // println!("{},{} -> {:?}", i, j, nums);
                if nums.len() == 2 {
                    result += nums.iter().product::<u32>();
                }

                // Reset seen array
                for row in seen.iter_mut() {
                    for col in row.iter_mut() {
                        *col = false
                    }
                }
            }
        }
        return result;
    }

    fn explore_gear(
        engine_schematic: &EngineSchematic,
        mut seen: &mut Seen,
        i: usize,
        j: usize,
    ) -> Vec<u32> {
        let directions = [
            [-1, -1],
            [-1, 0],
            [-1, 1],
            [0, 1],
            [1, 1],
            [1, 0],
            [1, -1],
            [0, -1],
        ];

        let mut result = vec![];
        for [i_offest, j_offset] in directions {
            #[rustfmt::skip]
            let i = (i as isize + i_offest) as usize;
            let j = (j as isize + j_offset) as usize;

            if !(0..140).contains(&j) || !(0..140).contains(&i) {
                continue;
            }

            let neighbor = engine_schematic[i][j];

            if neighbor.is_ascii_digit() && !seen[i][j] {
                let num = explore_digit(&engine_schematic, &mut seen, i, j);
                result.push(num);
            }
        }

        return result;
    }

    fn explore_digit(
        engine_schematic: &EngineSchematic,
        seen: &mut Seen,
        i: usize,
        mut j: usize,
    ) -> u32 {
        while (0..=138).contains(&j) && engine_schematic[i][j + 1].is_ascii_digit() {
            j += 1
        }

        let right = j;

        let mut result = engine_schematic[i][j].to_digit(10).unwrap();
        let mut radix = 10;
        while (1..=139).contains(&j) && engine_schematic[i][j - 1].is_ascii_digit() {
            j -= 1;

            result += engine_schematic[i][j].to_digit(10).unwrap() * radix;
            radix *= 10;
        }

        for k in j..=right {
            seen[i][k] = true;
        }
        return result;
    }
}
