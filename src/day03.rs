//! # Day 3: Binary Diagnostic
//!
//! ## --- Part One ---
//!
//! The submarine has been making some <span title="Turns out oceans are heavy.">odd creaking noises</span>, so you ask it to produce a diagnostic report just in case.
//!
//! The diagnostic report (your puzzle input) consists of a list of binary numbers which, when decoded properly, can tell you many useful things about the conditions of the submarine. The first parameter to check is the _power consumption_.
//!
//! You need to use the binary numbers in the diagnostic report to generate two new binary numbers (called the _gamma rate_ and the _epsilon rate_). The power consumption can then be found by multiplying the gamma rate by the epsilon rate.
//!
//! Each bit in the gamma rate can be determined by finding the _most common bit in the corresponding position_ of all numbers in the diagnostic report. For example, given the following diagnostic report:
//!
//! ```text
//! 00100
//! 11110
//! 10110
//! 10111
//! 10101
//! 01111
//! 00111
//! 11100
//! 10000
//! 11001
//! 00010
//! 01010
//! ```
//!
//! Considering only the first bit of each number, there are five `0` bits and seven `1` bits. Since the most common bit is `1`, the first bit of the gamma rate is `1`.
//!
//! The most common second bit of the numbers in the diagnostic report is `0`, so the second bit of the gamma rate is `0`.
//!
//! The most common value of the third, fourth, and fifth bits are `1`, `1`, and `0`, respectively, and so the final three bits of the gamma rate are `110`.
//!
//! So, the gamma rate is the binary number `10110`, or _`22`_ in decimal.
//!
//! The epsilon rate is calculated in a similar way; rather than use the most common bit, the least common bit from each position is used. So, the epsilon rate is `01001`, or _`9`_ in decimal. Multiplying the gamma rate (`22`) by the epsilon rate (`9`) produces the power consumption, _`198`_.
//!
//! Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate, then multiply them together. _What is the power consumption of the submarine?_ (Be sure to represent your answer in decimal, not binary.)
//!
//! ## --- Part Two ---
//!
//! Next, you should verify the _life support rating_, which can be determined by multiplying the _oxygen generator rating_ by the _CO2 scrubber rating_.
//!
//! Both the oxygen generator rating and the CO2 scrubber rating are values that can be found in your diagnostic report - finding them is the tricky part. Both values are located using a similar process that involves filtering out values until only one remains. Before searching for either rating value, start with the full list of binary numbers from your diagnostic report and _consider just the first bit_ of those numbers. Then:
//!
//! * Keep only numbers selected by the _bit criteria_ for the type of rating value for which you are searching. Discard numbers which do not match the bit criteria.
//! * If you only have one number left, stop; this is the rating value for which you are searching.
//! * Otherwise, repeat the process, considering the next bit to the right.
//!
//! The _bit criteria_ depends on which type of rating value you want to find:
//!
//! * To find _oxygen generator rating_, determine the _most common_ value (`0` or `1`) in the current bit position, and keep only numbers with that bit in that position. If `0` and `1` are equally common, keep values with a _`1`_ in the position being considered.
//! * To find _CO2 scrubber rating_, determine the _least common_ value (`0` or `1`) in the current bit position, and keep only numbers with that bit in that position. If `0` and `1` are equally common, keep values with a _`0`_ in the position being considered.
//!
//! For example, to determine the _oxygen generator rating_ value using the same example diagnostic report from above:
//!
//! * Start with all 12 numbers and consider only the first bit of each number. There are more `1` bits (7) than `0` bits (5), so keep only the 7 numbers with a `1` in the first position: `11110`, `10110`, `10111`, `10101`, `11100`, `10000`, and `11001`.
//! * Then, consider the second bit of the 7 remaining numbers: there are more `0` bits (4) than `1` bits (3), so keep only the 4 numbers with a `0` in the second position: `10110`, `10111`, `10101`, and `10000`.
//! * In the third position, three of the four numbers have a `1`, so keep those three: `10110`, `10111`, and `10101`.
//! * In the fourth position, two of the three numbers have a `1`, so keep those two: `10110` and `10111`.
//! * In the fifth position, there are an equal number of `0` bits and `1` bits (one each). So, to find the _oxygen generator rating_, keep the number with a `1` in that position: `10111`.
//! * As there is only one number left, stop; the _oxygen generator rating_ is `10111`, or _`23`_ in decimal.
//!
//! Then, to determine the _CO2 scrubber rating_ value from the same example above:
//!
//! * Start again with all 12 numbers and consider only the first bit of each number. There are fewer `0` bits (5) than `1` bits (7), so keep only the 5 numbers with a `0` in the first position: `00100`, `01111`, `00111`, `00010`, and `01010`.
//! * Then, consider the second bit of the 5 remaining numbers: there are fewer `1` bits (2) than `0` bits (3), so keep only the 2 numbers with a `1` in the second position: `01111` and `01010`.
//! * In the third position, there are an equal number of `0` bits and `1` bits (one each). So, to find the _CO2 scrubber rating_, keep the number with a `0` in that position: `01010`.
//! * As there is only one number left, stop; the _CO2 scrubber rating_ is `01010`, or _`10`_ in decimal.
//!
//! Finally, to find the life support rating, multiply the oxygen generator rating (`23`) by the CO2 scrubber rating (`10`) to get _`230`_.
//!
//! Use the binary numbers in your diagnostic report to calculate the oxygen generator rating and CO2 scrubber rating, then multiply them together. _What is the life support rating of the submarine?_ (Be sure to represent your answer in decimal, not binary.)

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
