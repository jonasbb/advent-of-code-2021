/// TODO Copy task description here
use crate::prelude::*;
use pathfinding::prelude::absdiff;

#[aoc_runner_derive::aoc_generator(day15)]
fn input_generator(input: &str) -> Result<Vec<Vec<u8>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Ok(c.to_digit(10).ok_or_else(|| anyhow!("Invalid digit"))? as u8))
                .collect()
        })
        .collect()
}

#[aoc_runner_derive::aoc(day15, part1)]
fn part1(input: &[Vec<u8>]) -> u32 {
    let mut openlist = Vec::new();

    let max_x = input[0].len();
    let max_y = input.len();
    let mut costs = vec![vec![u32::MAX; max_x]; max_y];
    costs[0][0] = 0;

    openlist.push((0, 0));

    while let Some((x, y)) = openlist.pop() {
        // Iterate over all neighbors and find the lowest cost
        for (xdiff, ydiff) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let neigh_x = x as isize + xdiff;
            let neigh_y = y as isize + ydiff;

            if neigh_x < 0 || neigh_x >= max_x as isize || neigh_y < 0 || neigh_y >= max_y as isize
            {
                continue;
            }

            let neigh_x = neigh_x as usize;
            let neigh_y = neigh_y as usize;

            let neigh_cost = costs[y][x] + input[neigh_y][neigh_x] as u32;
            if neigh_cost < costs[neigh_y][neigh_x] {
                costs[neigh_y][neigh_x] = neigh_cost;
                openlist.push((neigh_x, neigh_y));
            }
        }
    }

    costs[max_y - 1][max_x - 1]
}

#[aoc_runner_derive::aoc(day15, part1, astar)]
fn part1_astar(input: &[Vec<u8>]) -> u32 {
    search_astar(input)
}

fn search_astar(input: &[Vec<u8>]) -> u32 {
    let max_x = input[0].len();
    let max_y = input.len();

    let start = (0, 0);
    let goal = (max_x as isize - 1, max_y as isize - 1);

    let solution = pathfinding::directed::astar::astar(
        &start,
        |&(x, y): &(isize, isize)| -> Vec<((isize, isize), u32)> {
            [(-1, 0), (0, -1), (1, 0), (0, 1)]
                .into_iter()
                .filter_map(|(xdiff, ydiff)| {
                    let neigh_x = x + xdiff;
                    let neigh_y = y + ydiff;

                    if neigh_x < 0
                        || neigh_x >= max_x as isize
                        || neigh_y < 0
                        || neigh_y >= max_y as isize
                    {
                        None
                    } else {
                        Some((
                            (neigh_x, neigh_y),
                            input[neigh_y as usize][neigh_x as usize] as u32,
                        ))
                    }
                })
                .collect()
        },
        |&(x, y)| (absdiff(x, goal.0) + absdiff(y, goal.1)) as u32,
        |&p| p == goal,
    );

    solution.unwrap().1
}

#[aoc_runner_derive::aoc(day15, part2)]
fn part2(input: &[Vec<u8>]) -> u32 {
    let max_x = input[0].len();
    let max_y = input.len();

    // make the input 5 times larger for each dimension
    let mut larger_input = vec![vec![0; max_x * 5]; max_y * 5];
    for x in 0..(max_x * 5) {
        for y in 0..(max_y * 5) {
            let mut cost = input[y % max_y][x % max_x] + (y / max_y) as u8 + (x / max_x) as u8;
            while cost > 9 {
                // 10 wraps around to 1
                cost -= 9;
            }
            larger_input[y][x] = cost;
        }
    }

    for row in &larger_input {
        for field in row {
            print!("{}", field);
        }
        println!();
    }

    search_astar(&larger_input)
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

#[test]
fn test_part1() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(40, part1(&values));
    Ok(())
}

#[test]
fn test_part1_astar() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(40, part1_astar(&values));
    Ok(())
}

#[test]
fn test_part1_astar_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day15.txt").trim())?;
    assert_eq!(714, part1_astar(&values));
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(315, part2(&values));
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day15.txt").trim())?;
    assert_eq!(2948, part2(&values));
    Ok(())
}
