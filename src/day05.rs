//! # Day 5: Hydrothermal Venture
//!
//! ## --- Part One ---
//!
//! You come across a field of [hydrothermal vents](https://en.wikipedia.org/wiki/Hydrothermal_vent) on the ocean floor! These vents constantly produce large, opaque clouds, so it would be best to avoid them if possible.
//!
//! They tend to form in _lines_; the submarine helpfully produces a list of nearby <span title="Maybe they're Bresenham vents.">lines of vents</span> (your puzzle input) for you to review. For example:
//!
//! ```text
//! 0,9 -> 5,9
//! 8,0 -> 0,8
//! 9,4 -> 3,4
//! 2,2 -> 2,1
//! 7,0 -> 7,4
//! 6,4 -> 2,0
//! 0,9 -> 2,9
//! 3,4 -> 1,4
//! 0,0 -> 8,8
//! 5,5 -> 8,2
//! ```
//!
//! Each line of vents is given as a line segment in the format `x1,y1 -> x2,y2` where `x1`,`y1` are the coordinates of one end the line segment and `x2`,`y2` are the coordinates of the other end. These line segments include the points at both ends. In other words:
//!
//! * An entry like `1,1 -> 1,3` covers points `1,1`, `1,2`, and `1,3`.
//! * An entry like `9,7 -> 7,7` covers points `9,7`, `8,7`, and `7,7`.
//!
//! For now, _only consider horizontal and vertical lines_: lines where either `x1 = x2` or `y1 = y2`.
//!
//! So, the horizontal and vertical lines from the above list would produce the following diagram:
//!
//! ```text
//! .......1..
//! ..1....1..
//! ..1....1..
//! .......1..
//! .112111211
//! ..........
//! ..........
//! ..........
//! ..........
//! 222111....
//! ```
//!
//! In this diagram, the top left corner is `0,0` and the bottom right corner is `9,9`. Each position is shown as _the number of lines which cover that point_ or `.` if no line covers that point. The top-left pair of `1`s, for example, comes from `2,2 -> 2,1`; the very bottom row is formed by the overlapping lines `0,9 -> 5,9` and `0,9 -> 2,9`.
//!
//! To avoid the most dangerous areas, you need to determine _the number of points where at least two lines overlap_. In the above example, this is anywhere in the diagram with a `2` or larger - a total of _`5`_ points.
//!
//! Consider only horizontal and vertical lines. _At how many points do at least two lines overlap?_
//!
//! ## --- Part Two ---
//!
//! Unfortunately, considering only horizontal and vertical lines doesn't give you the full picture; you need to also consider _diagonal lines_.
//!
//! Because of the limits of the hydrothermal vent mapping system, the lines in your list will only ever be horizontal, vertical, or a diagonal line at exactly 45 degrees. In other words:
//!
//! * An entry like `1,1 -> 3,3` covers points `1,1`, `2,2`, and `3,3`.
//! * An entry like `9,7 -> 7,9` covers points `9,7`, `8,8`, and `7,9`.
//!
//! Considering all lines from the above example would now produce the following diagram:
//!
//! ```text
//! 1.1....11.
//! .111...2..
//! ..2.1.111.
//! ...1.2.2..
//! .112313211
//! ...1.2....
//! ..1...1...
//! .1.....1..
//! 1.......1.
//! 222111....
//! ```
//!
//! You still need to determine _the number of points where at least two lines overlap_. In the above example, this is still anywhere in the diagram with a `2` or larger - now a total of _`12`_ points.
//!
//! Consider all of the lines. _At how many points do at least two lines overlap?_

use crate::prelude::*;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[aoc_runner_derive::aoc_generator(day5)]
fn input_generator(input: &str) -> Result<Vec<(Point, Point)>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ").flat_map(|point| point.split(','));
            let startx = parts.next().unwrap().parse::<u32>()?;
            let starty = parts.next().unwrap().parse::<u32>()?;
            let endx = parts.next().unwrap().parse::<u32>()?;
            let endy = parts.next().unwrap().parse::<u32>()?;
            Ok((
                Point {
                    x: startx,
                    y: starty,
                },
                Point { x: endx, y: endy },
            ))
        })
        .collect()
}

#[aoc_runner_derive::aoc(day5, part1)]
fn part1(input: &[(Point, Point)]) -> usize {
    let mut grid = [[0_u8; 999]; 999];
    for (start, end) in input {
        if start.x == end.x {
            for y in start.y.min(end.y)..=start.y.max(end.y) {
                grid[start.x as usize][y as usize] += 1;
            }
        } else if start.y == end.y {
            for x in start.x.min(end.x)..=start.x.max(end.x) {
                grid[x as usize][start.y as usize] += 1;
            }
        }
    }

    grid.iter()
        .map(|row| row.iter().filter(|&&x| x >= 2).count())
        .sum()
}

#[aoc_runner_derive::aoc(day5, part2)]
fn part2(input: &[(Point, Point)]) -> usize {
    let mut grid = [[0_u8; 999]; 999];

    let range = |start, end| {
        if start <= end {
            Box::new(start..=end) as Box<dyn Iterator<Item = u32>>
        } else {
            Box::new((end..=start).rev())
        }
    };

    for (start, end) in input {
        if start.x == end.x {
            for y in start.y.min(end.y)..=start.y.max(end.y) {
                grid[start.x as usize][y as usize] += 1;
            }
        } else if start.y == end.y {
            for x in start.x.min(end.x)..=start.x.max(end.x) {
                grid[x as usize][start.y as usize] += 1;
            }
        } else {
            // Diagonals are always at an 45 degree angle, thus both iterators are the same length
            for (x, y) in range(start.x, end.x).zip(range(start.y, end.y)) {
                grid[x as usize][y as usize] += 1;
            }
        }
    }

    grid.iter()
        .map(|row| row.iter().filter(|&&x| x >= 2).count())
        .sum()
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

#[test]
fn test_part1() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(5, part1(&values));
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day5.txt").trim())?;
    assert_eq!(5197, part1(&values));
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(12, part2(&values));
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day5.txt").trim())?;
    assert_eq!(18605, part2(&values));
    Ok(())
}
