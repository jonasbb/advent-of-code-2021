/// TODO Copy task description here
use crate::prelude::*;
#[cfg(test)]
use pretty_assertions::assert_eq;

#[aoc_runner_derive::aoc_generator(day18)]
fn input_generator(input: &str) -> Vec<SnailfishNumberList> {
    input
        .lines()
        .map(|line| {
            SnailfishNumberList(
                line.chars()
                    .flat_map(|c| match c {
                        '[' => Some(SnailfishElement::Open),
                        ']' => Some(SnailfishElement::Close),
                        '0' => Some(SnailfishElement::Number(0)),
                        '1' => Some(SnailfishElement::Number(1)),
                        '2' => Some(SnailfishElement::Number(2)),
                        '3' => Some(SnailfishElement::Number(3)),
                        '4' => Some(SnailfishElement::Number(4)),
                        '5' => Some(SnailfishElement::Number(5)),
                        '6' => Some(SnailfishElement::Number(6)),
                        '7' => Some(SnailfishElement::Number(7)),
                        '8' => Some(SnailfishElement::Number(8)),
                        '9' => Some(SnailfishElement::Number(9)),
                        ',' => None,
                        _ => panic!("Invalid character: {}", c),
                    })
                    .collect(),
            )
        })
        .collect()
}

#[aoc_runner_derive::aoc(day18, part1)]
fn part1(input: &[SnailfishNumberList]) -> u32 {
    add_numbers(input.to_vec()).magnitude()
}

#[derive(PartialEq, Eq, Clone)]
struct SnailfishNumberList(Vec<SnailfishElement>);

impl SnailfishNumberList {
    fn magnitude(&self) -> u32 {
        // Turn self into a tree
        enum Tree {
            Leaf(u32),
            Pair(Box<Tree>, Box<Tree>),
        }

        impl Tree {
            fn magnitude(&self) -> u32 {
                match self {
                    Tree::Leaf(n) => *n,
                    Tree::Pair(left, right) => left.magnitude() * 3 + right.magnitude() * 2,
                }
            }
        }

        fn parse_tree(i: &mut impl Iterator<Item = SnailfishElement>) -> Tree {
            match i.next() {
                Some(SnailfishElement::Number(n)) => Tree::Leaf(n),
                Some(SnailfishElement::Open) => {
                    let left = parse_tree(i);
                    let right = parse_tree(i);
                    // Consume close
                    i.next();
                    Tree::Pair(Box::new(left), Box::new(right))
                }
                Some(SnailfishElement::Close) => panic!("Unexpected close"),
                None => panic!("Unexpected end of input"),
            }
        }

        parse_tree(&mut self.0.iter().cloned()).magnitude()
    }
}

impl std::fmt::Debug for SnailfishNumberList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut last_was_number = false;
        for e in &self.0 {
            if matches!(e, SnailfishElement::Number(_)) {
                if last_was_number {
                    write!(f, ",")?;
                }
                last_was_number = true;
            } else {
                last_was_number = false;
            }

            write!(f, "{:?}", e)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum SnailfishElement {
    Open,
    Close,
    Number(u32),
}

impl std::fmt::Debug for SnailfishElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "["),
            Self::Close => write!(f, "]"),
            Self::Number(num) => write!(f, "{}", num),
        }
    }
}

impl std::ops::Add for SnailfishNumberList {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Vec::with_capacity(self.0.len() + rhs.0.len() + 2);
        result.push(SnailfishElement::Open);
        result.extend(self.0);
        result.extend(rhs.0);
        result.push(SnailfishElement::Close);

        reduce(Self(result))
    }
}

fn add_numbers(numbers: Vec<SnailfishNumberList>) -> SnailfishNumberList {
    numbers.into_iter().reduce(|l, r| l + r).unwrap()
}

fn reduce(mut sfn: SnailfishNumberList) -> SnailfishNumberList {
    loop {
        // Always restart the loop if one of the options works
        if let Some(new_sfn) = explode(&sfn) {
            sfn = new_sfn;
            continue;
        }
        if let Some(new_sfn) = split(&sfn) {
            sfn = new_sfn;
            continue;
        }

        // No more options, so we're done
        return sfn;
    }
}

fn explode(sfn: &SnailfishNumberList) -> Option<SnailfishNumberList> {
    // Find the first place with a nesting level of 4
    let mut nesting_level = 0;
    let mut nesting_idx = None;

    for (idx, e) in sfn.0.iter().enumerate() {
        match e {
            SnailfishElement::Open => {
                nesting_level += 1;
            }
            SnailfishElement::Close => {
                nesting_level -= 1;
            }
            SnailfishElement::Number(_) => {}
        }
        if nesting_level == 5 {
            nesting_idx = Some(idx);
            break;
        }
    }

    // No number if nested too deeply
    let nesting_idx = nesting_idx?;
    match sfn.0[nesting_idx..nesting_idx + 4] {
        [SnailfishElement::Open, SnailfishElement::Number(left), SnailfishElement::Number(right), SnailfishElement::Close] =>
        {
            let mut result = Vec::with_capacity(sfn.0.len() - 3);
            result.extend_from_slice(&sfn.0[..nesting_idx]);
            // Find the previous number and add the left value
            if let Some(SnailfishElement::Number(num)) = result
                .iter_mut()
                .rev()
                .find(|e| matches!(e, SnailfishElement::Number(_)))
            {
                *num += left;
            }
            // The exploded element turns into a number
            result.push(SnailfishElement::Number(0));
            result.extend_from_slice(&sfn.0[nesting_idx + 4..]);
            // Find the next number and add the left value
            if let Some(SnailfishElement::Number(num)) = (&mut result[nesting_idx + 1..])
                .iter_mut()
                .find(|e| matches!(e, SnailfishElement::Number(_)))
            {
                *num += right;
            }

            Some(SnailfishNumberList(result))
        }
        _ => {
            panic!();
        }
    }
}

fn split(sfn: &SnailfishNumberList) -> Option<SnailfishNumberList> {
    // TODO only split the first number
    if let Some((idx, SnailfishElement::Number(num))) = sfn
        .0
        .iter()
        .enumerate()
        .find(|&(_, &e)| matches!(e, SnailfishElement::Number(num) if num >= 10))
    {
        let mut result = Vec::with_capacity(sfn.0.len() + 3);
        result.extend_from_slice(&sfn.0[..idx]);

        // First, divide by 2 rounded down
        // Second, divide by 2 rounded up
        result.push(SnailfishElement::Open);
        result.push(SnailfishElement::Number(num / 2));
        result.push(SnailfishElement::Number((num + 1) / 2));
        result.push(SnailfishElement::Close);

        result.extend_from_slice(&sfn.0[idx + 1..]);

        Some(SnailfishNumberList(result))
    } else {
        None
    }
}

#[aoc_runner_derive::aoc(day18, part2)]
fn part2(input: &[SnailfishNumberList]) -> u32 {
    let mut max_magnitude = 0;
    for (idx_a, sfn_a) in input.iter().enumerate() {
        for (idx_b, sfn_b) in input.iter().enumerate() {
            if idx_a != idx_b {
                max_magnitude =
                    max_magnitude.max(add_numbers(vec![sfn_a.clone(), sfn_b.clone()]).magnitude());
            }
        }
    }
    max_magnitude
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

#[test]
fn test_step_by_step() -> Result<()> {
    let input = r#"[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]"#;
    let values = input_generator(input);
    let expected = &input_generator("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")[0];

    let mut sfn = {
        let mut result = Vec::with_capacity(values[0].0.len() + values[1].0.len() + 2);
        result.push(SnailfishElement::Open);
        result.extend(&values[0].0);
        result.extend(&values[1].0);
        result.push(SnailfishElement::Close);
        SnailfishNumberList(result)
    };

    dbg!(&sfn);
    sfn = explode(&sfn).unwrap();
    dbg!(&sfn);
    sfn = explode(&sfn).unwrap();
    dbg!(&sfn);
    sfn = split(&sfn).unwrap();
    dbg!(&sfn);
    sfn = split(&sfn).unwrap();
    dbg!(&sfn);
    sfn = explode(&sfn).unwrap();
    dbg!(&sfn);

    assert_eq!(expected, &sfn);
    Ok(())
}

#[test]
fn test_add_a() -> Result<()> {
    let input = r#"[1,1]
[2,2]
[3,3]
[4,4]"#;
    let values = input_generator(input);
    let expected = &input_generator("[[[[1,1],[2,2]],[3,3]],[4,4]]")[0];

    assert_eq!(expected, &add_numbers(values));
    Ok(())
}

#[test]
fn test_add_b() -> Result<()> {
    let input = r#"[1,1]
[2,2]
[3,3]
[4,4]
[5,5]"#;
    let values = input_generator(input);
    let expected = &input_generator("[[[[3,0],[5,3]],[4,4]],[5,5]]")[0];

    assert_eq!(expected, &add_numbers(values));
    Ok(())
}

#[test]
fn test_add_c() -> Result<()> {
    let input = r#"[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]"#;
    let values = input_generator(input);
    let expected = &input_generator("[[[[5,0],[7,4]],[5,5]],[6,6]]")[0];

    assert_eq!(expected, &add_numbers(values));
    Ok(())
}

#[test]
fn test_add_d() -> Result<()> {
    let input = r#"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"#;
    let values = input_generator(input);
    let expected = &input_generator("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")[0];

    assert_eq!(expected, &add_numbers(values));
    Ok(())
}

#[test]
fn test_part1() -> Result<()> {
    let values = input_generator(TEST_INPUT_1);
    assert_eq!(4140, part1(&values));
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day18.txt").trim());
    assert_eq!(4008, part1(&values));
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let values = input_generator(TEST_INPUT_1);
    assert_eq!(3993, part2(&values));
    Ok(())
}

// #[test]
// fn test_part2_solution() -> Result<()> {
//     let values = input_generator(include_str!("../input/2021/day1.txt").trim())?;
//     assert_eq!(todo!(), part2(&values)?);
//     Ok(())
// }
