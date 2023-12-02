#[test]
fn test_part_1() {
    let input = std::fs::read_to_string("input").unwrap();
    assert_eq!(day_1::part_1(&input), 53194);
}

#[test]
fn test_part_2() {
    let input = std::fs::read_to_string("input").unwrap();
    assert_eq!(day_1::part_2(&input), 54249);
}
