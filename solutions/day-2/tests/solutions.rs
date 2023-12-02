#[test]
fn test_part_1() {
    let input = std::fs::read_to_string("input").unwrap();
    assert_eq!(day_2::part_1(&input), 2683);
}

#[test]
fn test_part_2() {
    let input = std::fs::read_to_string("input").unwrap();
    assert_eq!(day_2::part_2(&input), 49710);
}
