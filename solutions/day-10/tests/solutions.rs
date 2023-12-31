#[test]
fn test_part_1() {
    assert_eq!(day_10::part_1(day_10::INPUT), 6909);
}

#[test]
fn test_part_2() {
    assert_eq!(day_10::part_2(day_10::INPUT), 461);
    assert_eq!(day_10::part_2_shoelace_picks(day_10::INPUT), 461);
}
