/// TODO Copy task description here
use crate::prelude::*;

#[aoc_runner_derive::aoc_generator(day12)]
fn input_generator(input: &str) -> Result<Map<String, Vec<String>>> {
    let values = input.lines().map(|line| -> Result<_> {
        let mut parts = line.splitn(2, '-');
        let key = parts.next().unwrap().to_string();
        let value = parts.next().unwrap().to_string();
        Ok((key, value))
    });
    let mut res = Map::new();
    for x in values {
        let (key, value) = x?;
        // caves are connected in both directions
        res.entry(key.clone())
            .or_insert_with(Vec::new)
            .push(value.clone());
        res.entry(value).or_insert_with(Vec::new).push(key);
    }
    Ok(res)
}

#[aoc_runner_derive::aoc(day12, part1)]
fn part1(input: &Map<String, Vec<String>>) -> usize {
    let mut paths = Set::new();
    let mut partial_paths = vec![vec!["start".to_string()]];

    while let Some(path) = partial_paths.pop() {
        for reachable_cave in &input[&path[path.len() - 1]] {
            // Small caves (lowercase) may only be visited once
            if reachable_cave.chars().next().unwrap().is_ascii_lowercase()
                && path.contains(reachable_cave)
            {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(reachable_cave.clone());

            if reachable_cave == "end" {
                paths.insert(new_path);
            } else {
                partial_paths.push(new_path);
            }
        }
    }

    paths.len()
}

#[aoc_runner_derive::aoc(day12, part2)]
fn part2(input: &Map<String, Vec<String>>) -> usize {
    let mut paths = Set::new();
    let mut partial_paths = vec![(vec!["start".to_string()], false)];

    while let Some((path, has_double_small_cave)) = partial_paths.pop() {
        for reachable_cave in &input[&path[path.len() - 1]] {
            // start and end are never allowed to be visited twice
            if reachable_cave == "start" {
                continue;
            }
            // Only one small cave can be visited twice
            let is_small_cave = reachable_cave.chars().next().unwrap().is_ascii_lowercase();
            let double_cave = path.contains(reachable_cave);
            if is_small_cave && has_double_small_cave && double_cave {
                continue;
            }
            let visited_twice = has_double_small_cave || (is_small_cave && double_cave);

            let mut new_path = path.clone();
            new_path.push(reachable_cave.clone());

            if reachable_cave == "end" {
                paths.insert(new_path);
            } else {
                partial_paths.push((new_path, visited_twice));
            }
        }
    }

    paths.len()
}

#[cfg(test)]
static TEST_INPUT_SMALL: &str = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end";

#[cfg(test)]
static TEST_INPUT_MEDIUM: &str = r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

#[cfg(test)]
static TEST_INPUT_LARGE: &str = r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

#[test]
fn test_part1_small() -> Result<()> {
    let values = input_generator(TEST_INPUT_SMALL)?;
    assert_eq!(10, part1(&values));
    Ok(())
}

#[test]
fn test_part1_medium() -> Result<()> {
    let values = input_generator(TEST_INPUT_MEDIUM)?;
    assert_eq!(19, part1(&values));
    Ok(())
}

#[test]
fn test_part1_large() -> Result<()> {
    let values = input_generator(TEST_INPUT_LARGE)?;
    assert_eq!(226, part1(&values));
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day12.txt").trim())?;
    assert_eq!(4011, part1(&values));
    Ok(())
}

#[test]
fn test_part2_small() -> Result<()> {
    let values = input_generator(TEST_INPUT_SMALL)?;
    assert_eq!(36, part2(&values));
    Ok(())
}

#[test]
fn test_part2_medium() -> Result<()> {
    let values = input_generator(TEST_INPUT_MEDIUM)?;
    assert_eq!(103, part2(&values));
    Ok(())
}

#[test]
fn test_part2_large() -> Result<()> {
    let values = input_generator(TEST_INPUT_LARGE)?;
    assert_eq!(3509, part2(&values));
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day12.txt").trim())?;
    assert_eq!(108035, part2(&values));
    Ok(())
}
