//! # Day 4: Giant Squid
//!
//! ## --- Part One ---
//!
//! You're already almost 1.5km (almost a mile) below the surface of the ocean, already so deep that you can't see any sunlight. What you _can_ see, however, is a giant squid that has attached itself to the outside of your submarine.
//!
//! Maybe it wants to play [bingo](https://en.wikipedia.org/wiki/Bingo_(American_version))?
//!
//! Bingo is played on a set of boards each consisting of a 5x5 grid of numbers. Numbers are chosen at random, and the chosen number is _marked_ on all boards on which it appears. (Numbers may not appear on all boards.) If all numbers in any row or any column of a board are marked, that board _wins_. (Diagonals don't count.)
//!
//! The submarine has a _bingo subsystem_ to help passengers (currently, you and the giant squid) pass the time. It automatically generates a random order in which to draw numbers and a random set of boards (your puzzle input). For example:
//!
//! ```text
//! 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
//!
//! 22 13 17 11  0
//!  8  2 23  4 24
//! 21  9 14 16  7
//!  6 10  3 18  5
//!  1 12 20 15 19
//!
//!  3 15  0  2 22
//!  9 18 13 17  5
//! 19  8  7 25 23
//! 20 11 10 24  4
//! 14 21 16 12  6
//!
//! 14 21 17 24  4
//! 10 16 15  9 19
//! 18  8 23 26 20
//! 22 11 13  6  5
//!  2  0 12  3  7
//! ```
//!
//! After the first five numbers are drawn (`7`, `4`, `9`, `5`, and `11`), there are no winners, but the boards are marked as follows (shown here adjacent to each other to save space):
//!
//! ```text
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//! ```
//!
//! After the next six numbers are drawn (`17`, `23`, `2`, `0`, `14`, and `21`), there are still no winners:
//!
//! ```text
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//! ```
//!
//! Finally, `24` is drawn:
//!
//! ```text
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//! ```
//!
//! At this point, the third board _wins_ because it has at least one complete row or column of marked numbers (in this case, the entire top row is marked: _`14 21 17 24 4`_).
//!
//! The _score_ of the winning board can now be calculated. Start by finding the _sum of all unmarked numbers_ on that board; in this case, the sum is `188`. Then, multiply that sum by _the number that was just called_ when the board won, `24`, to get the final score, `188 * 24 = 4512`.
//!
//! To guarantee victory against the giant squid, figure out which board will win first. _What will your final score be if you choose that board?_
//!
//! ## --- Part Two ---
//!
//! On the other hand, it might be wise to try a different strategy: <span title="That's 'cuz a submarine don't pull things' antennas out of their sockets when they lose. Giant squid are known to do that.">let the giant squid win</span>.
//!
//! You aren't sure how many bingo boards a giant squid could play at once, so rather than waste time counting its arms, the safe thing to do is to _figure out which board will win last_ and choose that one. That way, no matter which boards it picks, it will win for sure.
//!
//! In the above example, the second board is the last to win, which happens after `13` is eventually called and its middle column is completely marked. If you were to keep playing until this point, the second board would have a sum of unmarked numbers equal to `148` for a final score of `148 * 13 = 1924`.
//!
//! Figure out which board will win last. _Once it wins, what would its final score be?_

use crate::prelude::*;

#[derive(Clone)]
struct Input {
    numbers: Vec<i32>,
    boards: Vec<Board>,
}

#[derive(Clone, Debug)]
struct Board {
    fields: Vec<Vec<i32>>,
}

impl Board {
    fn mark(&mut self, value: i32) {
        for row in &mut self.fields {
            for field in row {
                if *field == value {
                    *field = -1;
                }
            }
        }
    }

    fn sum_unmarked(&self) -> i32 {
        self.fields.iter().flatten().filter(|&&x| x >= 0).sum()
    }

    /// All fields in a row or column need to be marked
    fn is_solved(&self) -> bool {
        self.fields.iter().any(|row| row.iter().all(|&x| x == -1))
            || (0..self.fields[0].len())
                .any(|col_idx| self.fields.iter().all(|row| row[col_idx] == -1))
    }
}

#[aoc_runner_derive::aoc_generator(day4)]
fn input_generator(input: &str) -> Result<Input> {
    // let mut lines = input.lines();
    // let numbers = lines.next().unwrap().split(',').map(|s| Ok(s.parse()?)).collect::<Result<_>>()?;
    // // skip empty line
    // lines.next();
    let mut input = input.split("\n\n");
    let numbers = input
        .next()
        .unwrap()
        .split(',')
        .map(|s| Ok(s.parse()?))
        .collect::<Result<_>>()?;
    let boards = input
        .map(|board| {
            let fields = board
                .split('\n')
                .map(|row| {
                    (0..row.len())
                        .step_by(3)
                        .map(|i| Ok(row[i..i + 2].trim().parse()?))
                        .collect::<Result<_>>()
                })
                .collect::<Result<_>>()?;
            Ok(Board { fields })
        })
        .collect::<Result<_>>()?;

    Ok(Input { numbers, boards })
}

#[aoc_runner_derive::aoc(day4, part1)]
fn part1(input: &Input) -> i32 {
    let mut input = input.clone();
    for value in &input.numbers {
        for board in &mut input.boards {
            board.mark(*value);
            if board.is_solved() {
                return board.sum_unmarked() * value;
            }
        }
    }

    todo!()
}

#[aoc_runner_derive::aoc(day4, part2)]
fn part2(input: &Input) -> i32 {
    let mut input = input.clone();
    for value in &input.numbers {
        for board_idx in (0..input.boards.len()).rev() {
            input.boards[board_idx].mark(*value);
            if input.boards[board_idx].is_solved() {
                if input.boards.len() == 1 {
                    // Last winning board
                    return input.boards[board_idx].sum_unmarked() * value;
                }
                input.boards.remove(board_idx);
            }
        }
    }
    todo!()
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

#[test]
fn test_part1() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(4512, part1(&values));
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day4.txt").trim())?;
    assert_eq!(58412, part1(&values));
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(1924, part2(&values));
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day4.txt").trim())?;
    assert_eq!(10030, part2(&values));
    Ok(())
}
