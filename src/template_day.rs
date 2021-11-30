/// TODO Copy task description here
use crate::prelude::*;

#[aoc_runner_derive::aoc_generator(day1)]
fn input_generator(input: &str) -> Result<Vec<u32>> {
    todo!()
}

#[aoc_runner_derive::aoc(day1, part1)]
fn part1(input: &[u32]) -> Result<u32> {
    todo!()
}

#[aoc_runner_derive::aoc(day1, part2)]
fn part2(input: &[u32]) -> Result<u32> {
    todo!()
}

#[test]
fn test_part1() -> Result<()> {
    let input = todo!();
    let values = input_generator(input)?;
    assert_eq!(todo!(), part1(&values)?);
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day1.txt").trim())?;
    assert_eq!(todo!(), part1(&values)?);
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let input = todo!();
    let values = input_generator(input)?;
    assert_eq!(todo!(), part2(&values)?);
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day1.txt").trim())?;
    assert_eq!(todo!(), part2(&values)?);
    Ok(())
}
