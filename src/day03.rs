/// TODO Copy task description here
use crate::prelude::*;

#[aoc_runner_derive::aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '1').collect::<Vec<bool>>())
        .collect()
}

#[aoc_runner_derive::aoc(day3, part1)]
fn part1(input: &[Vec<bool>]) -> u32 {
    // Initialize all counts with 0
    // true increases the count, false decreases it
    let mut value_counter = vec![0; input[0].len()];
    for number in input {
        for (i, value) in number.iter().enumerate() {
            value_counter[i] += if *value { 1 } else { -1 };
        }
    }

    let gamma_rate = bits_to_number(value_counter.iter().map(|&x| x >= 0).collect());
    let epsilon_rate = bits_to_number(value_counter.iter().map(|&x| x < 0).collect());
    gamma_rate * epsilon_rate
}

#[aoc_runner_derive::aoc(day3, part2)]
fn part2(input: &[Vec<bool>]) -> u32 {
    let mut input_oxygen = input.to_vec();
    let mut input_co2 = input.to_vec();

    /// Find the most common bit at position `pos`
    fn most_common(i: &[Vec<bool>], pos: usize) -> bool {
        i.iter().map(|v| if v[pos] { 1 } else { -1 }).sum::<i32>() >= 0
    }

    /// Only keep values which have `bit` at `pos`
    fn keep_with_bit(i: Vec<Vec<bool>>, pos: usize, bit: bool) -> Vec<Vec<bool>> {
        i.iter()
            .filter_map(|v| if v[pos] == bit { Some(v.clone()) } else { None })
            .collect()
    }

    // Keep most common
    let mut pos = 0;
    while input_oxygen.len() > 1 {
        let bit = most_common(&input_oxygen, pos);
        input_oxygen = keep_with_bit(input_oxygen, pos, bit);
        pos += 1;
    }
    // Keep least common
    let mut pos = 0;
    while input_co2.len() > 1 {
        let bit = most_common(&input_co2, pos);
        input_co2 = keep_with_bit(input_co2, pos, !bit);
        pos += 1;
    }

    let oxygen_generator_rating = dbg!(bits_to_number(input_oxygen[0].clone()));
    let co2_scrubber_rating = dbg!(bits_to_number(input_co2[0].clone()));
    oxygen_generator_rating * co2_scrubber_rating
}

/// Turn the bits into a number
fn bits_to_number(i: Vec<bool>) -> u32 {
    let mut res = 0;
    for v in i {
        res *= 2;
        if v {
            res += 1
        };
    }
    res
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

#[test]
fn test_part1() -> Result<()> {
    let values = input_generator(TEST_INPUT_1);
    assert_eq!(198, part1(&values));
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day3.txt").trim());
    assert_eq!(4006064, part1(&values));
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let values = input_generator(TEST_INPUT_1);
    assert_eq!(230, part2(&values));
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day3.txt").trim());
    assert_eq!(5941884, part2(&values));
    Ok(())
}
