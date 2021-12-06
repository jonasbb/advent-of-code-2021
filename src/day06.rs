//! # Day 6: Lanternfish
//!
//! ## --- Part One ---
//!
//! The sea floor is getting steeper. Maybe the sleigh keys got carried this way?
//!
//! A massive school of glowing [lanternfish](https://en.wikipedia.org/wiki/Lanternfish) swims past. They must spawn quickly to reach such large numbers - maybe _exponentially_ quickly? You should model their growth rate to be sure.
//!
//! Although you know nothing about this specific species of lanternfish, you make some guesses about their attributes. Surely, <span title="I heard you like lanternfish.">each lanternfish creates a new lanternfish</span> once every _7_ days.
//!
//! However, this process isn't necessarily synchronized between every lanternfish - one lanternfish might have 2 days left until it creates another lanternfish, while another might have 4\. So, you can model each fish as a single number that represents _the number of days until it creates a new lanternfish_.
//!
//! Furthermore, you reason, a _new_ lanternfish would surely need slightly longer before it's capable of producing more lanternfish: two more days for its first cycle.
//!
//! So, suppose you have a lanternfish with an internal timer value of `3`:
//!
//! * After one day, its internal timer would become `2`.
//! * After another day, its internal timer would become `1`.
//! * After another day, its internal timer would become `0`.
//! * After another day, its internal timer would reset to `6`, and it would create a _new_ lanternfish with an internal timer of `8`.
//! * After another day, the first lanternfish would have an internal timer of `5`, and the second lanternfish would have an internal timer of `7`.
//!
//! A lanternfish that creates a new fish resets its timer to `6`, _not `7`_ (because `0` is included as a valid timer value). The new lanternfish starts with an internal timer of `8` and does not start counting down until the next day.
//!
//! Realizing what you're trying to do, the submarine automatically produces a list of the ages of several hundred nearby lanternfish (your puzzle input). For example, suppose you were given the following list:
//!
//! ```text
//! 3,4,3,1,2
//! ```
//!
//! This list means that the first fish has an internal timer of `3`, the second fish has an internal timer of `4`, and so on until the fifth fish, which has an internal timer of `2`. Simulating these fish over several days would proceed as follows:
//!
//! ```text
//! Initial state: 3,4,3,1,2
//! After  1 day:  2,3,2,0,1
//! After  2 days: 1,2,1,6,0,8
//! After  3 days: 0,1,0,5,6,7,8
//! After  4 days: 6,0,6,4,5,6,7,8,8
//! After  5 days: 5,6,5,3,4,5,6,7,7,8
//! After  6 days: 4,5,4,2,3,4,5,6,6,7
//! After  7 days: 3,4,3,1,2,3,4,5,5,6
//! After  8 days: 2,3,2,0,1,2,3,4,4,5
//! After  9 days: 1,2,1,6,0,1,2,3,3,4,8
//! After 10 days: 0,1,0,5,6,0,1,2,2,3,7,8
//! After 11 days: 6,0,6,4,5,6,0,1,1,2,6,7,8,8,8
//! After 12 days: 5,6,5,3,4,5,6,0,0,1,5,6,7,7,7,8,8
//! After 13 days: 4,5,4,2,3,4,5,6,6,0,4,5,6,6,6,7,7,8,8
//! After 14 days: 3,4,3,1,2,3,4,5,5,6,3,4,5,5,5,6,6,7,7,8
//! After 15 days: 2,3,2,0,1,2,3,4,4,5,2,3,4,4,4,5,5,6,6,7
//! After 16 days: 1,2,1,6,0,1,2,3,3,4,1,2,3,3,3,4,4,5,5,6,8
//! After 17 days: 0,1,0,5,6,0,1,2,2,3,0,1,2,2,2,3,3,4,4,5,7,8
//! After 18 days: 6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8
//! ```
//!
//! Each day, a `0` becomes a `6` and adds a new `8` to the end of the list, while each other number decreases by 1 if it was present at the start of the day.
//!
//! In this example, after 18 days, there are a total of `26` fish. After 80 days, there would be a total of _`5934`_.
//!
//! Find a way to simulate lanternfish. _How many lanternfish would there be after 80 days?_
//!
//! ## --- Part Two ---
//!
//! Suppose the lanternfish live forever and have unlimited food and space. Would they take over the entire ocean?
//!
//! After 256 days in the example above, there would be a total of _`26984457539`_ lanternfish!
//!
//! _How many lanternfish would there be after 256 days?_

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
