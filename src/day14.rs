/// TODO Copy task description here
use crate::prelude::*;
use anyhow::anyhow;

struct Input {
    polymer_template: Vec<char>,
    pair_insertions: Map<(char, char), char>,
}

#[aoc_runner_derive::aoc_generator(day14)]
fn input_generator(input: &str) -> Result<Input> {
    let (template, insertions) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Missing delimiter"))?;
    Ok(Input {
        polymer_template: template.chars().collect(),
        pair_insertions: insertions
            .lines()
            .map(|line| {
                let (pair, insertion) = line
                    .split_once(" -> ")
                    .ok_or_else(|| anyhow!("Missing delimiter"))?;
                let mut pair = pair.chars();
                let pair_a = pair.next().ok_or_else(|| anyhow!("Missing char"))?;
                let pair_b = pair.next().ok_or_else(|| anyhow!("Missing char"))?;
                let insertion = insertion
                    .chars()
                    .next()
                    .ok_or_else(|| anyhow!("Missing char"))?;
                Ok(((pair_a, pair_b), insertion))
            })
            .collect::<Result<_>>()?,
    })
}

fn insert_polymers(polymer_template: &[char], insertions: &Map<(char, char), char>) -> Vec<char> {
    // Never part of a pair
    let mut prev_char = '-';
    polymer_template
        .iter()
        .cloned()
        .flat_map(|char| {
            if let Some(&insert_char) = insertions.get(&(prev_char, char)) {
                prev_char = char;
                vec![insert_char, char]
            } else {
                prev_char = char;
                vec![char]
            }
        })
        .collect()
}

fn insert_polymers_pairs(
    polymer_pairs: &Map<(char, char), usize>,
    insertions: &Map<(char, char), char>,
) -> Map<(char, char), usize> {
    let mut res = Map::new();

    for (&(a, b), &count) in polymer_pairs {
        if let Some(&insert_char) = insertions.get(&(a, b)) {
            *res.entry((a, insert_char)).or_insert(0) += count;
            *res.entry((insert_char, b)).or_insert(0) += count;
        } else {
            *res.entry((a, b)).or_insert(0) += count;
        }
    }

    res
}

fn quantity_diff(polymer_template: &[char]) -> usize {
    // Count chars
    let mut counts = Map::new();
    for char in polymer_template {
        *counts.entry(char).or_insert(0) += 1;
    }
    let mut counts: Vec<usize> = counts.values().cloned().collect();
    counts.sort_unstable();
    // Take highest quantity and substract lowest quantity
    counts[counts.len() - 1] - counts[0]
}

fn quantity_diff_pairs(polymer_pairs: &Map<(char, char), usize>) -> usize {
    // Count chars, only count the first char in each pair, to avoid double counting
    let mut counts = Map::new();
    for ((a, _), count) in polymer_pairs {
        *counts.entry(a).or_insert(0) += count;
    }
    let mut counts: Vec<usize> = counts.values().cloned().collect();
    counts.sort_unstable();
    // Take highest quantity and substract lowest quantity
    counts[counts.len() - 1] - counts[0]
}

#[aoc_runner_derive::aoc(day14, part1, sequence)]
fn part1_sequence(input: &Input) -> usize {
    let mut polymer_template = input.polymer_template.clone();
    for _ in 0..10 {
        polymer_template = insert_polymers(&polymer_template, &input.pair_insertions);
    }
    quantity_diff(&polymer_template)
}

#[aoc_runner_derive::aoc(day14, part1, pairs)]
fn part1_pairs(input: &Input) -> usize {
    let mut polymer_pairs = Map::new();
    for pair in input
        .polymer_template
        .windows(2)
        .map(|pair| (pair[0], pair[1]))
    {
        *polymer_pairs.entry(pair).or_default() += 1;
    }
    // Add a pair for the last char too such that the counts are right afterwards
    *polymer_pairs
        .entry((
            input.polymer_template[input.polymer_template.len() - 1],
            '!',
        ))
        .or_default() += 1;

    for _ in 0..10 {
        polymer_pairs = insert_polymers_pairs(&polymer_pairs, &input.pair_insertions);
    }
    quantity_diff_pairs(&polymer_pairs)
}

#[aoc_runner_derive::aoc(day14, part2)]
fn part2(input: &Input) -> usize {
    let mut polymer_pairs = Map::new();
    for pair in input
        .polymer_template
        .windows(2)
        .map(|pair| (pair[0], pair[1]))
    {
        *polymer_pairs.entry(pair).or_default() += 1;
    }
    // Add a pair for the last char too such that the counts are right afterwards
    *polymer_pairs
        .entry((
            input.polymer_template[input.polymer_template.len() - 1],
            '!',
        ))
        .or_default() += 1;

    for _ in 0..40 {
        polymer_pairs = insert_polymers_pairs(&polymer_pairs, &input.pair_insertions);
    }
    quantity_diff_pairs(&polymer_pairs)
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

#[test]
fn test_part1_sequence() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(1588, part1_sequence(&values));
    Ok(())
}

#[test]
fn test_part1_pairs() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(1588, part1_pairs(&values));
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day14.txt").trim())?;
    assert_eq!(2967, part1_sequence(&values));
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(2188189693529, part2(&values));
    Ok(())
}

// #[test]
// fn test_part2_solution() -> Result<()> {
//     let values = input_generator(include_str!("../input/2021/day1.txt").trim())?;
//     assert_eq!(todo!(), part2(&values)?);
//     Ok(())
// }
