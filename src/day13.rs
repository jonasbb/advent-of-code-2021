/// TODO Copy task description here
use crate::prelude::*;
use anyhow::anyhow;

#[derive(Copy, Clone)]
enum Fold {
    X,
    Y,
}

#[aoc_runner_derive::aoc_generator(day13)]
fn input_generator(input: &str) -> Result<(Vec<Vec<bool>>, Vec<(Fold, usize)>)> {
    let (points, folds) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Missing empty separator line"))?;
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    let points = points
        .lines()
        .map(|line| -> Result<_> {
            let mut parts = line.splitn(2, ',');
            let x = parts.next().unwrap().parse()?;
            let y = parts.next().unwrap().parse()?;
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            Ok((x, y))
        })
        .collect::<Result<Vec<(usize, usize)>>>()?;
    let mut grid = vec![vec![false; max_x + 1]; max_y + 1];
    for (x, y) in points {
        grid[y][x] = true;
    }

    let folds = folds
        .lines()
        .map(|line| {
            let (direction, place) = line
                .strip_prefix("fold along ")
                .unwrap()
                .split_once('=')
                .unwrap();
            let place = place.parse::<usize>().unwrap();
            let direction = if direction == "x" { Fold::X } else { Fold::Y };
            (direction, place)
        })
        .collect::<Vec<_>>();
    Ok((grid, folds))
}

fn grid_to_string(grid: &[Vec<bool>]) -> String {
    let mut res = String::new();
    res.push('\n');
    for row in grid {
        for cell in row {
            res.push(if *cell { '#' } else { '.' });
        }
        res.push('\n');
    }
    res
}

fn fold_grid(grid: &[Vec<bool>], fold: Fold, place: usize) -> Vec<Vec<bool>> {
    match fold {
        Fold::X => {
            let mut new_grid = vec![vec![false; place]; grid.len()];
            for y in 0..grid.len() {
                for x in 0..grid[0].len() {
                    if x == place {
                        continue;
                    }
                    let mut new_x = x;
                    if x >= place {
                        new_x = place - (x - place);
                    }
                    new_grid[y][new_x] |= grid[y][x];
                }
            }
            new_grid
        }
        Fold::Y => {
            let mut new_grid = vec![vec![false; grid[0].len()]; place];
            for y in 0..grid.len() {
                if y == place {
                    continue;
                }
                for x in 0..grid[0].len() {
                    let mut new_y = y;
                    if y >= place {
                        new_y = place - (y - place);
                    }
                    new_grid[new_y][x] |= grid[y][x];
                }
            }
            new_grid
        }
    }
}

#[aoc_runner_derive::aoc(day13, part1)]
fn part1((grid, folds): &(Vec<Vec<bool>>, Vec<(Fold, usize)>)) -> usize {
    let mut grid = grid.clone();

    for &(direction, place) in folds.iter().take(1) {
        grid = fold_grid(&grid, direction, place);
    }

    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&x| x)
        .count()
}

#[aoc_runner_derive::aoc(day13, part2)]
fn part2((grid, folds): &(Vec<Vec<bool>>, Vec<(Fold, usize)>)) -> String {
    let mut grid = grid.clone();

    for &(direction, place) in folds {
        grid = fold_grid(&grid, direction, place);
    }
    grid_to_string(&grid)
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

#[test]
fn test_part1() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(17, part1(&values));
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day13.txt").trim())?;
    assert_eq!(763, part1(&values));
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    let solution = r"
#####
#...#
#...#
#...#
#####
.....
.....
";
    assert_eq!(solution, part2(&values));
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day13.txt").trim())?;
    let solution = r"
###..#..#..##..#....###...##..###...##..
#..#.#..#.#..#.#....#..#.#..#.#..#.#..#.
#..#.####.#..#.#....#..#.#....#..#.#..#.
###..#..#.####.#....###..#....###..####.
#.#..#..#.#..#.#....#.#..#..#.#.#..#..#.
#..#.#..#.#..#.####.#..#..##..#..#.#..#.
";
    assert_eq!(solution, part2(&values));
    Ok(())
}
