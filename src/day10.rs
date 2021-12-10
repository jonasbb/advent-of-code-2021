/// TODO Copy task description here
use crate::prelude::*;

// #[aoc_runner_derive::aoc_generator(day10)]
// fn input_generator(input: &str) -> Result<Vec<u32>> {
//     todo!()
// }

#[aoc_runner_derive::aoc(day10, part1)]
fn part1(input: &str) -> Result<u32> {
    let mut total_penalty = 0;
    for line in input.lines() {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => {
                    if let Some(opening) = stack.pop() {
                        total_penalty += match (opening, c) {
                            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => continue,
                            (_, ')') => 3,
                            (_, ']') => 57,
                            (_, '}') => 1197,
                            (_, '>') => 25137,
                            _ => anyhow::bail!("Unexpected character {}", c),
                        };
                    }
                }
            }
        }
    }
    Ok(total_penalty)
}

#[aoc_runner_derive::aoc(day10, part2)]
fn part2(input: &str) -> Result<u64> {
    let mut completion_scores = Vec::new();
    'line: for line in input.lines() {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => {
                    if let Some(opening) = stack.pop() {
                        match (opening, c) {
                            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => continue,
                            (_, ')' | ']' | '}' | '>') => continue 'line,
                            _ => anyhow::bail!("Unexpected character {}", c),
                        };
                    }
                }
            }
        }

        if stack.is_empty() {
            continue 'line;
        }

        let mut completion_score = 0_u64;
        while let Some(c) = stack.pop() {
            completion_score *= 5;
            completion_score += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => anyhow::bail!("Unexpected character {}", c),
            }
        }
        completion_scores.push(completion_score);
    }
    // Take the median completion_scores value
    completion_scores.sort_unstable();
    Ok(completion_scores[completion_scores.len() / 2])
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

#[test]
fn test_part1() -> Result<()> {
    assert_eq!(26397, part1(TEST_INPUT_1)?);
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    assert_eq!(
        344193,
        part1(include_str!("../input/2021/day10.txt").trim())?
    );
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    assert_eq!(288957, part2(TEST_INPUT_1)?);
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    assert_eq!(
        3241238967,
        part2(include_str!("../input/2021/day10.txt").trim())?
    );
    Ok(())
}
