pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {
    pub fn solution(input: &str) -> u32 {
        let rgb_limits = [12, 13, 14];
        let mut result = 0;
        'outer: for line in input.lines() {
            let (match_number, rgb_maxes) = parse_line(line);
            for i in 0..3 {
                if rgb_maxes[i] > rgb_limits[i] {
                    continue 'outer;
                }
            }
            result += match_number;
        }
        return result;
    }

    // line -> (match #, [red max, green max, blue max])
    pub fn parse_line(line: &str) -> (u32, [u32; 3]) {
        let mut rgb_maxes = [0; 3];
        let colon_offset = line.find(":").unwrap();
        let match_number: u32 = line[5..colon_offset].parse().unwrap();
        let sub_sets: Vec<&str> = line[colon_offset + 2..].split(";").collect();

        for sub_set in sub_sets {
            let pairs = sub_set.split(", ").collect::<Vec<&str>>();
            for pair in pairs {
                let (count, color): (u32, &str) = pair
                    .trim_start()
                    .split_once(" ")
                    .map(|(count, color)| (count.parse().unwrap(), color))
                    .unwrap();

                let rgb_index = match color {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => panic!("encoutered unexpected color"),
                };

                if count > rgb_maxes[rgb_index] {
                    rgb_maxes[rgb_index] = count
                };
            }
        }
        return (match_number, rgb_maxes);
    }
}
mod part_2 {
    pub fn solution(input: &str) -> u32 {
        let mut result = 0;
        for line in input.lines() {
            let (_, rgb_maxes) = super::part_1::parse_line(line);
            result += rgb_maxes.iter().product::<u32>();
        }
        return result;
    }
}
