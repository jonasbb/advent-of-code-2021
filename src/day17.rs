#![allow(clippy::comparison_chain)]
/// TODO Copy task description here
use crate::prelude::*;

struct TargetArea {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

#[aoc_runner_derive::aoc_generator(day17)]
fn input_generator(input: &str) -> Result<TargetArea> {
    // target area: x=20..30, y=-10..-5
    let (xvalues, yvalues) = input
        .strip_prefix("target area: x=")
        .unwrap()
        .split_once(", y=")
        .unwrap();
    let xvalues: Vec<_> = xvalues
        .split("..")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let yvalues: Vec<_> = yvalues
        .split("..")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    Ok(TargetArea {
        min_x: xvalues[0].min(xvalues[1]),
        min_y: yvalues[0].min(yvalues[1]),
        max_x: xvalues[0].max(xvalues[1]),
        max_y: yvalues[0].max(yvalues[1]),
    })
}

#[aoc_runner_derive::aoc(day17, part1)]
fn part1(input: &TargetArea) -> i32 {
    let mut highest_y = 0;

    for initial_x in 0..input.max_x {
        for initial_y in 0..500 {
            // Simulate shooting the probe from the initial 0,0 position with initial delta
            let mut curr_x = 0;
            let mut curr_y = 0;
            let mut delta_x = initial_x;
            let mut delta_y = initial_y;
            let mut highest_y_of_shot = 0;

            while curr_y >= input.min_y {
                curr_x += delta_x;
                curr_y += delta_y;
                highest_y_of_shot = curr_y.max(highest_y_of_shot);
                if delta_x > 0 {
                    delta_x -= 1;
                } else if delta_x < 0 {
                    delta_x += 1;
                }
                delta_y -= 1;

                if (input.min_x..=input.max_x).contains(&curr_x)
                    && (input.min_y..=input.max_y).contains(&curr_y)
                    && highest_y_of_shot > highest_y
                {
                    highest_y = highest_y_of_shot;
                    eprintln!(
                        "Found new highest y at {} for initial {},{}",
                        highest_y, initial_x, initial_y
                    );
                }
            }
        }
    }
    highest_y
}

#[aoc_runner_derive::aoc(day17, part2)]
fn part2(input: &TargetArea) -> usize {
    let mut initial_configurations = Set::new();

    for initial_x in 0..input.max_x + 5 {
        for initial_y in input.min_y..500 {
            // Simulate shooting the probe from the initial 0,0 position with initial delta
            let mut curr_x = 0;
            let mut curr_y = 0;
            let mut delta_x = initial_x;
            let mut delta_y = initial_y;

            while curr_y >= input.min_y {
                curr_x += delta_x;
                curr_y += delta_y;
                if delta_x > 0 {
                    delta_x -= 1;
                } else if delta_x < 0 {
                    delta_x += 1;
                }
                delta_y -= 1;

                if (input.min_x..=input.max_x).contains(&curr_x)
                    && (input.min_y..=input.max_y).contains(&curr_y)
                {
                    initial_configurations.insert((initial_x, initial_y));
                }
            }
        }
    }
    dbg!(initial_configurations).len()
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"target area: x=20..30, y=-10..-5";

#[test]
fn test_part1() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(45, part1(&values));
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day17.txt").trim())?;
    assert_eq!(10011, part1(&values));
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(112, part2(&values));
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day17.txt").trim())?;
    assert_eq!(2994, part2(&values));
    Ok(())
}
