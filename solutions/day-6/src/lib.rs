pub use part_1::solution as part_1;
pub use part_2::solution as part_2;
pub use part_2::solution_mathematical as part_2_mathematical;

pub const INPUT: &str = include_str!("../input");

mod part_1 {
    pub fn solution(input: &str) -> u32 {
        let mut lines = input.lines();

        let times = lines
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        let distances = lines
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        let mut results: Vec<u32> = vec![];

        for (time, distance) in times.zip(distances) {
            let mut count: u32 = 0;
            for t in 0..=time {
                if t * (time - t) >= distance {
                    count += 1;
                }
            }
            results.push(count);
        }
        return results.iter().product();
    }
}

mod part_2 {
    pub fn solution(input: &str) -> u64 {
        let mut lines = input.lines();

        let time = lines
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .split_whitespace()
            .collect::<String>()
            .parse::<u64>()
            .unwrap();

        let distance = lines
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .split_whitespace()
            .collect::<String>()
            .parse::<u64>()
            .unwrap();

        let mut lower: u64 = 0;
        for t in 0..=time {
            if t * (time - t) >= distance {
                lower = t;
                break;
            }
        }

        let mut upper: u64 = 0;
        for t in (0..=time).into_iter().rev() {
            if t * (time - t) >= distance {
                upper = t;
                break;
            }
        }
        return upper - lower + 1;
    }

    fn quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
        let t = ((-1 as f64 * b) + ((b * b) - 4 as f64 * a * c).sqrt()) / 2 as f64 * a;
        let x = ((-1 as f64 * b) - ((b * b) - 4 as f64 * a * c).sqrt()) / 2 as f64 * a;

        return (t, x);
    }

    pub fn solution_mathematical(input: &str) -> u64 {
        let mut lines = input.lines();

        let time = lines
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .split_whitespace()
            .collect::<String>()
            .parse::<f64>()
            .unwrap();

        let distance = lines
            .next()
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .split_whitespace()
            .collect::<String>()
            .parse::<f64>()
            .unwrap();

        let (lower, upper) = quadratic_formula(-1_f64, time, distance * -1_f64);
        return (upper - lower) as u64 + 1;
    }
}
