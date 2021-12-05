/// TODO Copy task description here
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
