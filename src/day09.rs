/// TODO Copy task description here
use crate::prelude::*;

#[aoc_runner_derive::aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[aoc_runner_derive::aoc(day9, part1)]
fn part1(input: &[Vec<u32>]) -> u32 {
    let xlen = input.len();
    let ylen = input[0].len();
    let mut total_risk = 0;

    for x in 0..xlen {
        for y in 0..ylen {
            let height = input[x][y];
            let mut is_low_point = true;
            'neighbors: for xdiff in [-1, 0, 1] {
                for ydiff in [-1, 0, 1] {
                    let x2 = x as i32 + xdiff;
                    let y2 = y as i32 + ydiff;
                    if x2 < 0 || x2 >= xlen as i32 || y2 < 0 || y2 >= ylen as i32 {
                        continue;
                    }
                    let neighbor_height = input[x2 as usize][y2 as usize];
                    if height > neighbor_height {
                        is_low_point = false;
                        break 'neighbors;
                    }
                }
            }

            if is_low_point {
                total_risk += height + 1;
            }
        }
    }

    total_risk
}

#[aoc_runner_derive::aoc(day9, part2)]
fn part2(input: &[Vec<u32>]) -> u32 {
    let mut input = input.to_vec();
    let xlen = input.len();
    let ylen = input[0].len();
    let mut basin_sizes = Vec::new();

    for x in 0..xlen {
        for y in 0..ylen {
            let height = input[x][y];
            if height == 9 {
                continue;
            }

            let add_points_to_unvisited =
                |unvisited: &mut Vec<(usize, usize)>, x: usize, y: usize| {
                    for (xdiff, ydiff) in [(0, -1), (1, 0), (0, 1), (-1, 0)].iter() {
                        let x2 = x as i32 + xdiff;
                        let y2 = y as i32 + ydiff;
                        if x2 < 0 || x2 >= xlen as i32 || y2 < 0 || y2 >= ylen as i32 {
                            continue;
                        }
                        unvisited.push((x2 as usize, y2 as usize));
                    }
                };

            let mut basin_size = 0;
            let mut unvisited_points = vec![(x, y)];
            while let Some((x, y)) = unvisited_points.pop() {
                let height = input[x][y];
                if height == 9 {
                    continue;
                }
                basin_size += 1;
                input[x][y] = 9;
                add_points_to_unvisited(&mut unvisited_points, x, y);
            }
            basin_sizes.push(basin_size);
        }
    }

    basin_sizes.sort_by_key(|&x| std::cmp::Reverse(x));
    basin_sizes.iter().take(3).cloned().product()
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"2199943210
3987894921
9856789892
8767896789
9899965678";

#[test]
fn test_part1() -> Result<()> {
    let values = input_generator(TEST_INPUT_1);
    assert_eq!(15, part1(&values));
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day9.txt").trim());
    assert_eq!(502, part1(&values));
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let values = input_generator(TEST_INPUT_1);
    assert_eq!(1134, part2(&values));
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day9.txt").trim());
    assert_eq!(1330560, part2(&values));
    Ok(())
}
