use super::parse_input;

pub fn solution(input: &str) -> i64 {
    let history = parse_input(input);
    return history
        .into_iter()
        .map(|line| extrapolate_next_value(line))
        .sum();
}

fn extrapolate_next_value(line: Vec<i64>) -> i64 {
    if line.iter().all(|x| *x == 0) {
        return 0;
    };

    let diffs: Vec<i64> = line.windows(2).map(|x| x[1] - x[0]).collect();

    return line.last().unwrap() + extrapolate_next_value(diffs);
}
