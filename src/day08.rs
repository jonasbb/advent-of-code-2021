/// TODO Copy task description here
use crate::prelude::*;

struct Input {
    samples: [SegmentDisplay; 10],
    digits: [SegmentDisplay; 4],
}

impl Input {
    fn infer_mapping(&self) -> Map<SegmentDisplay, usize> {
        let mut mapping = Map::new();
        let mut digit_to_sd = Map::new();
        // Add the basic simple entries which have a unique count
        for (segment_count, digit) in [(2, 1), (4, 4), (3, 7), (7, 8)] {
            if let Some(&sd) = self
                .samples
                .iter()
                .find(|sd| sd.segment_count() == segment_count)
            {
                mapping.insert(sd, digit);
                digit_to_sd.insert(digit, sd);
            }
        }

        // Find the 1 and then infer what the 6 is.
        // The 6 has all but one segment set, one of the segments overlaps with the 1.
        // 9 and 0 also have all but one segment set, but use both of the 1 segments.
        let one = digit_to_sd[&1];
        let six = *self
            .samples
            .iter()
            .filter(|sd| sd.segment_count() == 6)
            .find(|&&sd| (sd & one).segment_count() == 1)
            .unwrap();
        mapping.insert(six, 6);
        // 5 and 6 are the only two entries which are missing the top right segment
        // 5 is a subset of 6, so anding them is the identity operation
        let five = *self
            .samples
            .iter()
            .filter(|sd| sd.segment_count() == 5)
            .find(|&&sd| (sd & six) == sd)
            .unwrap();
        mapping.insert(five, 5);
        // 5 is also a subset of 9, but not of 9, but not of 0
        // The same property holds for 6, so don't match that again
        let nine = *self
            .samples
            .iter()
            .filter(|sd| sd.segment_count() == 6)
            .filter(|&&sd| sd != six)
            .find(|&&sd| (sd & five) == five)
            .unwrap();
        mapping.insert(nine, 9);
        // 0 is the only six digit entry which is not in mapping yet
        let zero = *self
            .samples
            .iter()
            .filter(|sd| sd.segment_count() == 6)
            .find(|&&sd| mapping.get(&sd).is_none())
            .unwrap();
        mapping.insert(zero, 0);
        // 3 is a subset of 9, similar to how 5 is a subset of 6
        // 5 is also a subset of 9, but we can ignore it
        let three = *self
            .samples
            .iter()
            .filter(|sd| sd.segment_count() == 5)
            .filter(|&&sd| sd != five)
            .find(|&&sd| (sd & nine) == sd)
            .unwrap();
        mapping.insert(three, 3);
        // 2 is the only entry we have not assigned yet
        let two = *self
            .samples
            .iter()
            .filter(|sd| sd.segment_count() == 5)
            .find(|&&sd| mapping.get(&sd).is_none())
            .unwrap();
        mapping.insert(two, 2);
        debug_assert_eq!(mapping.len(), 10, "{:?}", mapping.values());
        mapping
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SegmentDisplay([bool; 7]);

impl SegmentDisplay {
    fn segment_count(&self) -> usize {
        self.0.iter().filter(|&&b| b).count()
    }
}

impl FromStr for SegmentDisplay {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut digits = [false; 7];
        for digit in s.chars() {
            anyhow::ensure!(
                ('a'..='g').contains(&digit),
                "Only a-g are valid digits, but found {}",
                digit
            );
            digits[(digit as u8 - b'a') as usize] = true;
        }
        Ok(SegmentDisplay(digits))
    }
}

impl std::ops::BitAnd for SegmentDisplay {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        let mut result = [false; 7];
        result
            .iter_mut()
            .zip(self.0.iter().zip(rhs.0.iter()))
            .for_each(|(result_digit, (&self_digit, &rhs_digit))| {
                *result_digit = self_digit && rhs_digit;
            });
        SegmentDisplay(result)
    }
}

#[aoc_runner_derive::aoc_generator(day8)]
fn input_generator(input: &str) -> Result<Vec<Input>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" | ");
            let samples = parts
                .next()
                .unwrap()
                .split(' ')
                .map(|s| SegmentDisplay::from_str(s))
                .collect::<Result<Vec<SegmentDisplay>>>()?
                .try_into()
                .map_err(|err: Vec<SegmentDisplay>| {
                    anyhow::anyhow!(
                        "Failed to create array, because input Vec is only length {}",
                        err.len()
                    )
                })?;
            let digits = parts
                .next()
                .unwrap()
                .split(' ')
                .map(|s| SegmentDisplay::from_str(s))
                .collect::<Result<Vec<SegmentDisplay>>>()?
                .try_into()
                .map_err(|err: Vec<SegmentDisplay>| {
                    anyhow::anyhow!(
                        "Failed to create array, because input Vec is only length {}",
                        err.len()
                    )
                })?;
            Ok(Input { samples, digits })
        })
        .collect()
}

#[aoc_runner_derive::aoc(day8, part1)]
fn part1(input: &[Input]) -> usize {
    input
        .iter()
        .flat_map(|i| i.digits.iter())
        .filter(|sd| matches!(sd.segment_count(), 2 | 3 | 4 | 7))
        .count()
}

#[aoc_runner_derive::aoc(day8, part2)]
fn part2(input: &[Input]) -> usize {
    input
        .iter()
        .map(|i| {
            let mapping = i.infer_mapping();
            i.digits
                .iter()
                .map(|d| mapping[d])
                .fold(0, |acc, d| acc * 10 + d)
        })
        .sum()
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

#[test]
fn test_part1() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(26, part1(&values));
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day8.txt").trim())?;
    assert_eq!(344, part1(&values));
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let values = input_generator(TEST_INPUT_1)?;
    assert_eq!(61229, part2(&values));
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day8.txt").trim())?;
    assert_eq!(1048410, part2(&values));
    Ok(())
}
