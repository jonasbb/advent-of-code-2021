/// TODO Copy task description here
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
