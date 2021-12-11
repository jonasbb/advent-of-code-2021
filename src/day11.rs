/// TODO Copy task description here
use crate::prelude::*;

#[aoc_runner_derive::aoc_generator(day11)]
fn input_generator(input: &str) -> Result<[[u8; 10]; 10]> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| anyhow::format_err!("Invalid length"))
        })
        .collect::<Result<Vec<_>>>()?
        .try_into()
        .map_err(|_| anyhow::format_err!("Invalid length"))
}

#[aoc_runner_derive::aoc(day11, part1)]
fn part1(input: &[[u8; 10]; 10]) -> u32 {
    let mut input = *input;
    let mut total_flashes = 0;
    for _ in 0..100 {
        total_flashes += step(&mut input);
    }
    total_flashes
}

fn step(input: &mut [[u8; 10]; 10]) -> u32 {
    let mut total_flashes = 0;

    /// Return the number of flashes triggered by this octopus
    fn check_octopus(input: &mut [[u8; 10]; 10], x: usize, y: usize) -> u32 {
        let mut flashes = 0;
        if input[x][y] > 9 {
            input[x][y] = 0;
            flashes += 1;
            for xdiff in -1_isize..=1 {
                for ydiff in -1_isize..=1 {
                    if xdiff == 0 && ydiff == 0 {
                        continue;
                    }
                    let x = (x as isize + xdiff) as usize;
                    let y = (y as isize + ydiff) as usize;
                    if x < 10 && y < 10 {
                        // Only increment if that octopus hasn't flashed this round yet
                        if input[x][y] != 0 {
                            input[x][y] += 1;
                            flashes += check_octopus(input, x, y);
                        }
                    }
                }
            }
        }
        flashes
    }

    // First increment the count for each octopus
    for row in &mut *input {
        for octopus in &mut *row {
            *octopus += 1;
        }
    }

    // Iterate and flash every octopus greater than 9
    for x in 0..10 {
        for y in 0..10 {
            total_flashes += check_octopus(input, x, y);
        }
    }

    total_flashes
}

#[aoc_runner_derive::aoc(day11, part2)]
fn part2(input: &[[u8; 10]; 10]) -> u32 {
    let mut input = *input;
    for i in 1.. {
        if step(&mut input) == 100 {
            return i;
        }
    }
    unreachable!()
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

#[test]
fn test_part1() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(1656, part1(&values));
    Ok(())
}

#[test]
fn test_part1_10steps() -> Result<()> {
    let mut values = input_generator(TEST_INPUT_1)?;
    let flashes = step(&mut values);
    assert_eq!(0, flashes);
    let flashes = step(&mut values);
    assert_eq!(35, flashes);
    let flashes = step(&mut values);
    assert_eq!(45, flashes);
    let flashes = step(&mut values);
    assert_eq!(16, flashes);
    let flashes = step(&mut values);
    assert_eq!(8, flashes);
    let flashes = step(&mut values);
    assert_eq!(1, flashes);
    let flashes = step(&mut values);
    assert_eq!(7, flashes);
    let flashes = step(&mut values);
    assert_eq!(24, flashes);
    let flashes = step(&mut values);
    assert_eq!(39, flashes);
    let flashes = step(&mut values);
    assert_eq!(29, flashes);
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day11.txt").trim())?;
    assert_eq!(1627, part1(&values));
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(195, part2(&values));
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day11.txt").trim())?;
    assert_eq!(329, part2(&values));
    Ok(())
}
