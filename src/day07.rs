/// TODO Copy task description here
use crate::prelude::*;

#[aoc_runner_derive::aoc_generator(day7)]
fn input_generator(input: &str) -> Result<Vec<u32>> {
    let mut res = Vec::new();
    for v in input.split(',') {
        let v = usize::from_str(v)?;
        if v >= res.len() {
            res.resize(v + 1, 0);
        }
        res[v] += 1;
    }
    Ok(res)
}

#[aoc_runner_derive::aoc(day7, part1)]
fn part1(input: &[u32]) -> u32 {
    // calculate the cost of moving everything to 0
    let mut curr_cost = input
        .iter()
        .enumerate()
        .map(|(i, &count)| i as u32 * count)
        .sum();
    let mut min_cost = curr_cost;
    let mut before_idx = 0;
    let mut at_idx = input[0];
    let mut after_idx = input.iter().sum::<u32>() - at_idx;

    for count in &input[1..] {
        before_idx += at_idx;
        at_idx = *count;
        after_idx -= at_idx;

        curr_cost = curr_cost + before_idx - after_idx - at_idx;
        if curr_cost > min_cost {
            return min_cost;
        } else {
            min_cost = curr_cost;
        }
    }

    todo!()
}

#[aoc_runner_derive::aoc(day7, part1, naive)]
fn part1_naive(input: &[u32]) -> u32 {
    let mut min_cost = u32::MAX;
    for i in 0..input.len() {
        let curr_cost = input
            .iter()
            .enumerate()
            .map(|(pos, &count)| {
                let diff = (pos as i32 - i as i32).abs() as u32;
                diff * count
            })
            .sum();
        if curr_cost > min_cost {
            return min_cost;
        } else {
            min_cost = curr_cost;
        }
    }

    todo!()
}

#[aoc_runner_derive::aoc(day7, part2)]
fn part2(input: &[u32]) -> u32 {
    let mut min_cost = u32::MAX;
    for i in 0..input.len() {
        let curr_cost = input
            .iter()
            .enumerate()
            .map(|(pos, &count)| {
                let diff = (pos as i32 - i as i32).abs() as u32;
                let cost = (diff + 1) * diff / 2;
                cost * count
            })
            .sum();
        if curr_cost > min_cost {
            return min_cost;
        } else {
            min_cost = curr_cost;
        }
    }

    todo!()
}

#[cfg(test)]
static TEST_INPUT_1: &str = "16,1,2,0,4,2,7,1,2,14";

#[test]
fn test_part1() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(37, part1(&values));
    Ok(())
}

#[test]
fn test_part1_naive() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(37, part1_naive(&values));
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day7.txt").trim())?;
    assert_eq!(352331, part1(&values));
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(168, part2(&values));
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day7.txt").trim())?;
    assert_eq!(99266250, part2(&values));
    Ok(())
}
