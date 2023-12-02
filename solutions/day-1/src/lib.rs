pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {
    pub fn solution(input: &str) -> u32 {
        let mut result = 0;
        let mut buf: [u8; 2] = [0; 2];

        for line in input.lines() {
            for byte in line.bytes() {
                if byte.is_ascii_digit() {
                    buf[0] = byte;
                    break;
                }
            }
            for byte in line.bytes().rev() {
                if byte.is_ascii_digit() {
                    buf[1] = byte;
                    break;
                }
            }
            let num: u32 = core::str::from_utf8(&buf).unwrap().parse().unwrap();
            result += num
        }
        return result;
    }
}
mod part_2 {
    use std::collections::HashMap;

    pub fn solution(input: &str) -> u32 {
        let word_map: HashMap<&str, u32> = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]);

        let mut result: u32 = 0;
        let mut matches: Vec<u32> = Vec::new();
        for line in input.lines() {
            for (i, char) in line.chars().enumerate() {
                if char.is_digit(10) {
                    matches.push(char.to_digit(10).unwrap());
                    continue;
                }
                let sub_string = &line[i..];

                for (key, value) in word_map.iter() {
                    if key.len() > sub_string.len() {
                        continue;
                    }
                    if sub_string.starts_with(key) {
                        matches.push(*value);
                        break;
                    }
                }
            }
            let first = matches.first().unwrap();
            let last = matches.last().unwrap();
            // println!("{} -> {}", line, first * 10 + last);
            result += first * 10 + last;
            matches.clear();
        }
        return result;
    }
}
