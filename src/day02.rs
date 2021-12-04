//! # Day 2: Dive!
//!
//! ## --- Part One ---
//!
//! Now, you need to figure out how to <span title="Tank, I need a pilot program for a B212 helicopter.">pilot this thing</span>.
//!
//! It seems like the submarine can take a series of commands like `forward 1`, `down 2`, or `up 3`:
//!
//! * `forward X` increases the horizontal position by `X` units.
//! * `down X` _increases_ the depth by `X` units.
//! * `up X` _decreases_ the depth by `X` units.
//!
//! Note that since you're on a submarine, `down` and `up` affect your _depth_, and so they have the opposite result of what you might expect.
//!
//! The submarine seems to already have a planned course (your puzzle input). You should probably figure out where it's going. For example:
//!
//! ```text
//! forward 5
//! down 5
//! forward 8
//! up 3
//! down 8
//! forward 2
//! ```
//!
//! Your horizontal position and depth both start at `0`. The steps above would then modify them as follows:
//!
//! * `forward 5` adds `5` to your horizontal position, a total of `5`.
//! * `down 5` adds `5` to your depth, resulting in a value of `5`.
//! * `forward 8` adds `8` to your horizontal position, a total of `13`.
//! * `up 3` decreases your depth by `3`, resulting in a value of `2`.
//! * `down 8` adds `8` to your depth, resulting in a value of `10`.
//! * `forward 2` adds `2` to your horizontal position, a total of `15`.
//!
//! After following these instructions, you would have a horizontal position of `15` and a depth of `10`. (Multiplying these together produces _`150`_.)
//!
//! Calculate the horizontal position and depth you would have after following the planned course. _What do you get if you multiply your final horizontal position by your final depth?_
//!
//! ## --- Part Two ---
//!
//! Based on your calculations, the planned course doesn't seem to make any sense. You find the submarine manual and discover that the process is actually slightly more complicated.
//!
//! In addition to horizontal position and depth, you'll also need to track a third value, _aim_, which also starts at `0`. The commands also mean something entirely different than you first thought:
//!
//! * `down X` _increases_ your aim by `X` units.
//! * `up X` _decreases_ your aim by `X` units.
//! * `forward X` does two things:
//!     * It increases your horizontal position by `X` units.
//!     * It increases your depth by your aim _multiplied by_ `X`.
//!
//! Again note that since you're on a submarine, `down` and `up` do the opposite of what you might expect: "down" means aiming in the positive direction.
//!
//! Now, the above example does something different:
//!
//! * `forward 5` adds `5` to your horizontal position, a total of `5`. Because your aim is `0`, your depth does not change.
//! * `down 5` adds `5` to your aim, resulting in a value of `5`.
//! * `forward 8` adds `8` to your horizontal position, a total of `13`. Because your aim is `5`, your depth increases by `8*5=40`.
//! * `up 3` decreases your aim by `3`, resulting in a value of `2`.
//! * `down 8` adds `8` to your aim, resulting in a value of `10`.
//! * `forward 2` adds `2` to your horizontal position, a total of `15`. Because your aim is `10`, your depth increases by `2*10=20` to a total of `60`.
//!
//! After following these new instructions, you would have a horizontal position of `15` and a depth of `60`. (Multiplying these produces _`900`_.)
//!
//! Using this new interpretation of the commands, calculate the horizontal position and depth you would have after following the planned course. _What do you get if you multiply your final horizontal position by your final depth?_

use crate::prelude::*;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Deserialize, Recap)]
#[recap(regex = r#"(?P<direction>\S+)\s(?P<distance>\d+)"#)]
struct Command {
    direction: Direction,
    distance: usize,
}

#[aoc_runner_derive::aoc_generator(day2)]
fn input_generator(input: &str) -> Result<Vec<Command>> {
    input.lines().map(|line| Ok(line.parse()?)).collect()
}

#[aoc_runner_derive::aoc(day2, part1)]
fn part1(input: &[Command]) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for cmd in input {
        match cmd.direction {
            Direction::Up => depth -= cmd.distance,
            Direction::Down => depth += cmd.distance,
            Direction::Forward => horizontal += cmd.distance,
        }
    }
    (horizontal * depth) as _
}

#[aoc_runner_derive::aoc(day2, part2)]
fn part2(input: &[Command]) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for cmd in input {
        match cmd.direction {
            Direction::Up => aim -= cmd.distance,
            Direction::Down => aim += cmd.distance,
            Direction::Forward => {
                horizontal += cmd.distance;
                depth += aim * cmd.distance;
            }
        }
    }
    (horizontal * depth) as _
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"forward 5
down 5
forward 8
up 3
down 8
forward 2";

#[test]
fn test_part1() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(150, part1(&values));
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day2.txt").trim())?;
    assert_eq!(1507611, part1(&values));
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(900, part2(&values));
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day2.txt").trim())?;
    assert_eq!(1880593125, part2(&values));
    Ok(())
}
