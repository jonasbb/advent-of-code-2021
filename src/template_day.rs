/// TODO Copy task description here

#[aoc_runner_derive::aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<u32> {
    todo!()
}

#[aoc_runner_derive::aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    todo!()
}

#[aoc_runner_derive::aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    todo!()
}

#[test]
fn test_part1() {
    let input = r#"1721
979
366
299
675
1456"#;
    let values = input_generator(input);
    assert_eq!(514579, part1(&values));
}

#[test]
fn test_part1_solution() {
    let values = input_generator(include_str!("../input/2021/day1.txt").trim());
    assert_eq!(todo!(), part1(&values));
}

#[test]
fn test_part2() {
    let input = todo!();
    let values = input_generator(input);
    assert_eq!(todo!(), part2(&values));
}

#[test]
fn test_part2_solution() {
    let values = input_generator(include_str!("../input/2021/day1.txt").trim());
    assert_eq!(todo!(), part2(&values));
}
