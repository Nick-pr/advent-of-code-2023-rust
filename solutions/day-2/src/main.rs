fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("------- Day 2 -------");
    println!("Part 1: {}", day_2::part_1(&input));
    println!("Part 2: {}", day_2::part_2(&input));
}
