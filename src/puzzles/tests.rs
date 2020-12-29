#[test]
fn test_day_1() {
    assert_eq!(super::day_1::part_1(), 32064);
    assert_eq!(super::day_1::part_2(), 193598720);
}

#[test]
fn test_day_2() {
    assert_eq!(super::day_2::part_1(), 640);
    assert_eq!(super::day_2::part_2(), 472);
}

#[test]
fn test_day_3() {
    assert_eq!(super::day_3::part_1(), 187);
    assert_eq!(super::day_3::part_2(), 4723283400);
}

#[test]
fn test_day_4() {
    assert_eq!(super::day_4::part_1(), 213);
    assert_eq!(super::day_4::part_2(), 147);
}

#[test]
fn test_day_5() {
    assert_eq!(super::day_5::part_1(), 838);
    assert_eq!(super::day_5::part_2(), Some(714));
}
