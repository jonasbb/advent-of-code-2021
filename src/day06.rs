/// TODO Copy task description here
use crate::prelude::*;

/// Fish can have a counter between 0 and 8 (inclusive)
#[derive(Debug, Default, Copy, Clone)]
struct State([u64; 9]);

#[aoc_runner_derive::aoc_generator(day6)]
fn input_generator(input: &str) -> Result<State> {
    let mut state = State::default();
    for c in input.split(',') {
        let c = usize::from_str(c)?;
        state.0[c] += 1;
    }
    Ok(state)
}

fn simulate_days(state: &mut State, days: usize) {
    for _ in 0..days {
        let new_births = state.0[0];
        state.0[0] = state.0[1];
        state.0[1] = state.0[2];
        state.0[2] = state.0[3];
        state.0[3] = state.0[4];
        state.0[4] = state.0[5];
        state.0[5] = state.0[6];
        state.0[6] = state.0[7];
        state.0[7] = state.0[8];

        state.0[6] += new_births;
        state.0[8] = new_births;
    }
}

#[aoc_runner_derive::aoc(day6, part1)]
fn part1(input: &State) -> u64 {
    let mut input = *input;
    simulate_days(&mut input, 80);
    input.0.iter().sum()
}

#[aoc_runner_derive::aoc(day6, part2)]
fn part2(input: &State) -> u64 {
    let mut input = *input;
    simulate_days(&mut input, 256);
    input.0.iter().sum()
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"3,4,3,1,2";

#[test]
fn test_part1_26_days() -> Result<()> {
    let mut values = input_generator(TEST_INPUT_1)?;
    simulate_days(&mut values, 18);
    assert_eq!(26_u64, values.0.iter().sum());
    Ok(())
}

#[test]
fn test_part1_80_days() -> Result<()> {
    let mut values = input_generator(TEST_INPUT_1)?;
    simulate_days(&mut values, 80);
    assert_eq!(5934_u64, values.0.iter().sum());
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day6.txt").trim())?;
    assert_eq!(365862, part1(&values));
    Ok(())
}

#[test]
fn test_part2_256_days() -> Result<()> {
    let mut values = input_generator(TEST_INPUT_1)?;
    simulate_days(&mut values, 256);
    assert_eq!(26984457539_u64, values.0.iter().sum());
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day6.txt").trim())?;
    assert_eq!(1653250886439, part2(&values));
    Ok(())
}
