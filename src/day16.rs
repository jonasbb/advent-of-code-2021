/// TODO Copy task description here
use crate::prelude::*;
#[cfg(test)]
pub use pretty_assertions::assert_eq;

#[derive(Debug, PartialEq, Eq)]
struct BitsPacket {
    version: u8,
    ty: BitsType,
}

#[derive(Debug, PartialEq, Eq)]
enum BitsType {
    Lit {
        literal: u128,
    },
    Operator {
        ty: BitsOperatorType,
        op: BitsOperands,
    },
}

#[derive(Debug, PartialEq, Eq)]
enum BitsOperatorType {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

impl TryFrom<u8> for BitsOperatorType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BitsOperatorType::Sum),
            1 => Ok(BitsOperatorType::Product),
            2 => Ok(BitsOperatorType::Minimum),
            3 => Ok(BitsOperatorType::Maximum),
            4 => Ok(BitsOperatorType::Literal),
            5 => Ok(BitsOperatorType::GreaterThan),
            6 => Ok(BitsOperatorType::LessThan),
            7 => Ok(BitsOperatorType::EqualTo),
            _ => Err("Invalid BitsOperatorType"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum BitsOperands {
    LengthDefined {
        length: u16,
        subpackets: Vec<BitsPacket>,
    },
    PacketDefined {
        count: u16,
        subpackets: Vec<BitsPacket>,
    },
}

impl BitsOperands {
    fn subpacket(&self) -> &[BitsPacket] {
        match self {
            BitsOperands::LengthDefined { subpackets, .. }
            | BitsOperands::PacketDefined { subpackets, .. } => subpackets,
        }
    }
}

#[aoc_runner_derive::aoc_generator(day16)]
fn input_generator(input: &str) -> Result<BitsPacket> {
    // First convert the hex input into a bit vector
    let bits: Vec<bool> = input
        .chars()
        .flat_map(|c| match c {
            '0' => [false, false, false, false],
            '1' => [false, false, false, true],
            '2' => [false, false, true, false],
            '3' => [false, false, true, true],
            '4' => [false, true, false, false],
            '5' => [false, true, false, true],
            '6' => [false, true, true, false],
            '7' => [false, true, true, true],
            '8' => [true, false, false, false],
            '9' => [true, false, false, true],
            'A' => [true, false, true, false],
            'B' => [true, false, true, true],
            'C' => [true, true, false, false],
            'D' => [true, true, false, true],
            'E' => [true, true, true, false],
            'F' => [true, true, true, true],
            _ => panic!("Invalid hex character: {}", c),
        })
        .collect();

    let (packet, remaining_bits) = parse_packet(&bits);
    // Ensure all remaining bits are 0
    assert_bits_unset(remaining_bits);
    Ok(packet)
}

#[track_caller]
fn assert_bits_unset(bits: &[bool]) {
    assert!(
        bits.iter().all(|b| !b),
        "The {} bits are not all unset: {:?}",
        bits.len(),
        bits
    );
}

/// Returns one parsed packets and the unprocessed bits
fn parse_packet(mut bits: &[bool]) -> (BitsPacket, &[bool]) {
    let version = bits_to_u8(&bits[..3]);
    bits = &bits[3..];

    let ty = bits_to_u8(&bits[..3]);
    bits = &bits[3..];
    let ty = BitsOperatorType::try_from(ty).unwrap();

    let ty = if ty == BitsOperatorType::Literal {
        let mut literal = 0;
        loop {
            // Shift to make space for the next couple of bits read
            literal <<= 4;
            let literal_bits = &bits[..5];
            bits = &bits[5..];
            match literal_bits {
                [true, literal_bits @ ..] => {
                    literal += bits_to_u8(literal_bits) as u128;
                    // First true means we need to continue processing data
                }
                [false, literal_bits @ ..] => {
                    literal += bits_to_u8(literal_bits) as u128;
                    // First false mean it is the end of the literal value
                    break;
                }
                _ => panic!("Invalid literal bits: {:?}", bits),
            }
        }
        BitsType::Lit { literal }
    } else {
        let bit_op = bits[0];
        bits = &bits[1..];

        let op = if bit_op {
            let count = bits_to_u16(&bits[..11]);
            bits = &bits[11..];
            let mut subpackets = Vec::new();
            for _ in 0..count {
                let (packet, remaining_bits) = parse_packet(bits);
                subpackets.push(packet);
                bits = remaining_bits;
            }
            BitsOperands::PacketDefined { count, subpackets }
        } else {
            let length = bits_to_u16(&bits[..15]);
            bits = &bits[15..];
            let mut subpackets = Vec::new();
            let mut subpacket_bits = &bits[..length as usize];
            bits = &bits[length as usize..];
            while !subpacket_bits.is_empty() {
                let (subpacket, remaining_bits) = parse_packet(subpacket_bits);
                subpackets.push(subpacket);
                subpacket_bits = remaining_bits;
            }
            BitsOperands::LengthDefined { length, subpackets }
        };

        BitsType::Operator { ty, op }
    };

    (BitsPacket { version, ty }, bits)
}

fn bits_to_u8(bits: &[bool]) -> u8 {
    let res = bits_to_u16(bits);
    assert!(res <= u8::MAX as u16);
    res as u8
}

fn bits_to_u16(bits: &[bool]) -> u16 {
    bits.iter()
        .fold(0, |acc, bit| (acc << 1) + if *bit { 1 } else { 0 })
}

#[aoc_runner_derive::aoc(day16, part1)]
fn part1(input: &BitsPacket) -> u32 {
    let mut sum_versions = input.version as _;

    sum_versions += match input {
        BitsPacket {
            ty:
                BitsType::Operator {
                    op:
                        BitsOperands::LengthDefined { subpackets, .. }
                        | BitsOperands::PacketDefined { subpackets, .. },
                    ..
                },
            ..
        } => subpackets.iter().map(|p| part1(p)).sum(),
        _ => 0,
    };

    sum_versions
}

#[aoc_runner_derive::aoc(day16, part2)]
fn part2(input: &BitsPacket) -> u128 {
    match input {
        // Literals are the easy case
        BitsPacket {
            ty: BitsType::Lit { literal },
            ..
        } => *literal,
        BitsPacket {
            ty: BitsType::Operator { ty, op },
            ..
        } => {
            // evaluate subpackets
            let mut subpackets = op.subpacket().iter().map(|p| part2(p));

            match ty {
                BitsOperatorType::Sum => subpackets.sum(),
                BitsOperatorType::Product => subpackets.product(),
                BitsOperatorType::Minimum => subpackets.min().unwrap(),
                BitsOperatorType::Maximum => subpackets.max().unwrap(),
                BitsOperatorType::Literal => unreachable!(),
                BitsOperatorType::GreaterThan
                | BitsOperatorType::LessThan
                | BitsOperatorType::EqualTo => {
                    let left = subpackets.next().unwrap();
                    let right = subpackets.next().unwrap();

                    (match ty {
                        BitsOperatorType::GreaterThan => left > right,
                        BitsOperatorType::LessThan => left < right,
                        BitsOperatorType::EqualTo => left == right,
                        _ => unreachable!(),
                    }) as u128
                }
            }
        }
    }
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"D2FE28";
#[cfg(test)]
static TEST_INPUT_2: &str = r"38006F45291200";
#[cfg(test)]
static TEST_INPUT_3: &str = r"EE00D40C823060";
#[cfg(test)]
static TEST_INPUT_4: &str = r"8A004A801A8002F478";
#[cfg(test)]
static TEST_INPUT_5: &str = r"620080001611562C8802118E34";
#[cfg(test)]
static TEST_INPUT_6: &str = r"C0015000016115A2E0802F182340";
#[cfg(test)]
static TEST_INPUT_7: &str = r"A0016C880162017C3686B18A3D4780";

#[test]
fn test_parse_input1() -> Result<()> {
    use BitsType::*;

    let values = input_generator(TEST_INPUT_1)?;
    let expected = BitsPacket {
        version: 6,
        ty: Lit { literal: 2021 },
    };
    assert_eq!(expected, values);
    Ok(())
}

#[test]
fn test_parse_input2() -> Result<()> {
    use BitsOperands::*;
    use BitsOperatorType::*;
    use BitsType::*;

    let values = input_generator(TEST_INPUT_2)?;
    let expected = BitsPacket {
        version: 1,
        ty: Operator {
            ty: LessThan,
            op: LengthDefined {
                length: 27,
                subpackets: vec![
                    BitsPacket {
                        version: 6,
                        ty: Lit { literal: 10 },
                    },
                    BitsPacket {
                        version: 2,
                        ty: Lit { literal: 20 },
                    },
                ],
            },
        },
    };
    assert_eq!(expected, values);
    Ok(())
}

#[test]
fn test_parse_input3() -> Result<()> {
    use BitsOperands::*;
    use BitsOperatorType::*;
    use BitsType::*;

    let values = input_generator(TEST_INPUT_3)?;
    let expected = BitsPacket {
        version: 7,
        ty: Operator {
            ty: Maximum,
            op: PacketDefined {
                count: 3,
                subpackets: vec![
                    BitsPacket {
                        version: 2,
                        ty: Lit { literal: 1 },
                    },
                    BitsPacket {
                        version: 4,
                        ty: Lit { literal: 2 },
                    },
                    BitsPacket {
                        version: 1,
                        ty: Lit { literal: 3 },
                    },
                ],
            },
        },
    };
    assert_eq!(expected, values);
    Ok(())
}

#[test]
fn test_parse_input4() -> Result<()> {
    use BitsOperands::*;
    use BitsOperatorType::*;
    use BitsType::*;

    let values = input_generator(TEST_INPUT_4)?;
    let expected = BitsPacket {
        version: 4,
        ty: Operator {
            ty: Minimum,
            op: PacketDefined {
                count: 1,
                subpackets: vec![BitsPacket {
                    version: 1,
                    ty: Operator {
                        ty: Minimum,
                        op: PacketDefined {
                            count: 1,
                            subpackets: vec![BitsPacket {
                                version: 5,
                                ty: Operator {
                                    ty: Minimum,
                                    op: LengthDefined {
                                        length: 11,
                                        subpackets: vec![BitsPacket {
                                            version: 6,
                                            ty: Lit { literal: 15 },
                                        }],
                                    },
                                },
                            }],
                        },
                    },
                }],
            },
        },
    };
    assert_eq!(expected, values);
    Ok(())
}

#[test]
fn test_parse_input5() -> Result<()> {
    use BitsOperands::*;
    use BitsOperatorType::*;
    use BitsType::*;

    let values = input_generator(TEST_INPUT_5)?;
    let expected = BitsPacket {
        version: 3,
        ty: Operator {
            ty: Sum,
            op: PacketDefined {
                count: 2,
                subpackets: vec![
                    BitsPacket {
                        version: 0,
                        ty: Operator {
                            ty: Sum,
                            op: LengthDefined {
                                length: 22,
                                subpackets: vec![
                                    BitsPacket {
                                        version: 0,
                                        ty: Lit { literal: 10 },
                                    },
                                    BitsPacket {
                                        version: 5,
                                        ty: Lit { literal: 11 },
                                    },
                                ],
                            },
                        },
                    },
                    BitsPacket {
                        version: 1,
                        ty: Operator {
                            ty: Sum,
                            op: PacketDefined {
                                count: 2,
                                subpackets: vec![
                                    BitsPacket {
                                        version: 0,
                                        ty: Lit { literal: 12 },
                                    },
                                    BitsPacket {
                                        version: 3,
                                        ty: Lit { literal: 13 },
                                    },
                                ],
                            },
                        },
                    },
                ],
            },
        },
    };
    assert_eq!(expected, values);
    Ok(())
}

#[test]
fn test_parse_input6() -> Result<()> {
    use BitsOperands::*;
    use BitsOperatorType::*;
    use BitsType::*;

    let values = input_generator(TEST_INPUT_6)?;
    let expected = BitsPacket {
        version: 6,
        ty: Operator {
            ty: Sum,
            op: LengthDefined {
                length: 84,
                subpackets: vec![
                    BitsPacket {
                        version: 0,
                        ty: Operator {
                            ty: Sum,
                            op: LengthDefined {
                                length: 22,
                                subpackets: vec![
                                    BitsPacket {
                                        version: 0,
                                        ty: Lit { literal: 10 },
                                    },
                                    BitsPacket {
                                        version: 6,
                                        ty: Lit { literal: 11 },
                                    },
                                ],
                            },
                        },
                    },
                    BitsPacket {
                        version: 4,
                        ty: Operator {
                            ty: Sum,
                            op: PacketDefined {
                                count: 2,
                                subpackets: vec![
                                    BitsPacket {
                                        version: 7,
                                        ty: Lit { literal: 12 },
                                    },
                                    BitsPacket {
                                        version: 0,
                                        ty: Lit { literal: 13 },
                                    },
                                ],
                            },
                        },
                    },
                ],
            },
        },
    };
    assert_eq!(expected, values);
    Ok(())
}

#[test]
fn test_parse_input7() -> Result<()> {
    use BitsOperands::*;
    use BitsOperatorType::*;
    use BitsType::*;

    let values = input_generator(TEST_INPUT_7)?;
    let expected = BitsPacket {
        version: 5,
        ty: Operator {
            ty: Sum,
            op: LengthDefined {
                length: 91,
                subpackets: vec![BitsPacket {
                    version: 1,
                    ty: Operator {
                        ty: Sum,
                        op: PacketDefined {
                            count: 1,
                            subpackets: vec![BitsPacket {
                                version: 3,
                                ty: Operator {
                                    ty: Sum,
                                    op: PacketDefined {
                                        count: 5,
                                        subpackets: vec![
                                            BitsPacket {
                                                version: 7,
                                                ty: Lit { literal: 6 },
                                            },
                                            BitsPacket {
                                                version: 6,
                                                ty: Lit { literal: 6 },
                                            },
                                            BitsPacket {
                                                version: 5,
                                                ty: Lit { literal: 12 },
                                            },
                                            BitsPacket {
                                                version: 2,
                                                ty: Lit { literal: 15 },
                                            },
                                            BitsPacket {
                                                version: 2,
                                                ty: Lit { literal: 15 },
                                            },
                                        ],
                                    },
                                },
                            }],
                        },
                    },
                }],
            },
        },
    };
    assert_eq!(expected, values);
    Ok(())
}

#[test]
fn test_part1_input4() -> Result<()> {
    let values = input_generator(TEST_INPUT_4)?;
    assert_eq!(16, part1(&values));
    Ok(())
}

#[test]
fn test_part1_input5() -> Result<()> {
    let values = input_generator(TEST_INPUT_5)?;
    assert_eq!(12, part1(&values));
    Ok(())
}

#[test]
fn test_part1_input6() -> Result<()> {
    let values = input_generator(TEST_INPUT_6)?;
    assert_eq!(23, part1(&values));
    Ok(())
}

#[test]
fn test_part1_input7() -> Result<()> {
    let values = input_generator(TEST_INPUT_7)?;
    assert_eq!(31, part1(&values));
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day16.txt").trim())?;
    assert_eq!(895, part1(&values));
    Ok(())
}

#[test]
fn test_part2_input1() -> Result<()> {
    let values = input_generator("C200B40A82")?;
    assert_eq!(3, part2(&values));
    Ok(())
}

#[test]
fn test_part2_input2() -> Result<()> {
    let values = input_generator("04005AC33890")?;
    assert_eq!(54, part2(&values));
    Ok(())
}

#[test]
fn test_part2_input3() -> Result<()> {
    let values = input_generator("880086C3E88112")?;
    assert_eq!(7, part2(&values));
    Ok(())
}

#[test]
fn test_part2_input4() -> Result<()> {
    let values = input_generator("CE00C43D881120")?;
    assert_eq!(9, part2(&values));
    Ok(())
}

#[test]
fn test_part2_input5() -> Result<()> {
    let values = input_generator("D8005AC2A8F0")?;
    assert_eq!(1, part2(&values));
    Ok(())
}

#[test]
fn test_part2_input6() -> Result<()> {
    let values = input_generator("F600BC2D8F")?;
    assert_eq!(0, part2(&values));
    Ok(())
}

#[test]
fn test_part2_input7() -> Result<()> {
    let values = input_generator("9C005AC2F8F0")?;
    assert_eq!(0, part2(&values));
    Ok(())
}

#[test]
fn test_part2_input8() -> Result<()> {
    let values = input_generator("9C0141080250320F1802104A08")?;
    assert_eq!(1, part2(&values));
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2021/day16.txt").trim())?;
    assert_eq!(1148595959144, part2(&values));
    Ok(())
}
