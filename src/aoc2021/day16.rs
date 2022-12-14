/// problem: https://adventofcode.com/2021/day/16
/// input: "https://adventofcode.com/2021/day/16/input"
use crate::Problem;
const P: Problem = Problem {
    year: 2021,
    day: 16,
    name: "Packet Decoder",
};

use itertools::Itertools;
use std::str::{Chars, FromStr};

#[derive(Debug)]
pub enum TypeID {
    Literal,
    Operation(u8),
}

pub struct Packet {
    length: usize,
    type_id: TypeID,
    version: u8,
    value: usize,
    subpackets: Option<Vec<Packet>>,
}

impl Packet {
    pub fn sum_versions(&self) -> usize {
        match self.type_id {
            TypeID::Literal => self.version as usize,
            TypeID::Operation(_) => {
                self.version as usize
                    + match &self.subpackets {
                        Some(subpackets) => {
                            subpackets.iter().map(|p| p.sum_versions()).sum::<usize>()
                        }
                        None => 0,
                    }
            }
        }
    }

    pub fn from_chars(chars: &mut Chars) -> anyhow::Result<Self> {
        // let b = &mut bin_chars; // TODO: why the fuck do i need to do this on a ref
        let version = u8::from_str_radix(chars.take(3).collect::<String>().as_str(), 2)
            .expect("unable to parse version from binary chars");
        let type_id = match u8::from_str_radix(chars.take(3).collect::<String>().as_str(), 2)
            .expect("unable to parse type_id from binary chars")
        {
            4 => TypeID::Literal,
            v => TypeID::Operation(v),
        };

        let mut length = 6; // length of header
        let subpackets = match type_id {
            TypeID::Literal => None,
            TypeID::Operation(_) => {
                length += 1;
                match chars.next().expect("unexpected end of string") {
                    '0' => {
                        let length_of_remaining_data =
                            usize::from_str_radix(chars.take(15).collect::<String>().as_str(), 2)?;
                        length += 15 + length_of_remaining_data;

                        let mut idx = 0;
                        let mut packets = Vec::new();
                        while idx < length_of_remaining_data {
                            let next_packet = Packet::from_chars(chars)?;
                            idx += next_packet.length;
                            packets.push(next_packet);
                        }
                        Some(packets)
                    }
                    '1' => {
                        let num_of_sub_packets =
                            usize::from_str_radix(chars.take(11).collect::<String>().as_str(), 2)
                                .unwrap();
                        length += 11;
                        let mut packets = Vec::with_capacity(num_of_sub_packets);
                        for _ in 0..num_of_sub_packets {
                            let next_packet = Packet::from_chars(chars)?;
                            length += next_packet.length;
                            packets.push(next_packet);
                        }
                        Some(packets)
                    }
                    c => panic!("unexpected char found in binary string: {c}"),
                }
            }
        };
        let value = match type_id {
            TypeID::Literal => {
                let mut val = String::new();
                while chars.next().expect("unexpected end of string") != '0' {
                    val.extend(chars.take(4));
                    length += 5;
                }
                // consume last packet
                val.extend(chars.take(4));
                length += 5;
                usize::from_str_radix(val.as_str(), 2).expect("unable to parse data into digit")
            }
            TypeID::Operation(op) => match op {
                0 => subpackets.as_ref().unwrap().iter().map(|p| p.value).sum(),
                1 => subpackets
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|p| p.value)
                    .product(),
                2 => subpackets
                    .as_ref()
                    .unwrap()
                    .iter()
                    .fold(usize::MAX, |min, p| min.min(p.value)),
                3 => subpackets
                    .as_ref()
                    .unwrap()
                    .iter()
                    .fold(0, |max, p| max.max(p.value)),
                4 => unreachable!("Literal should have already been captured"),
                5 => {
                    let mut ps = subpackets.as_ref().unwrap().iter().map(|p| p.value);
                    usize::from(ps.next() > ps.next())
                }
                6 => {
                    let mut ps = subpackets.as_ref().unwrap().iter().map(|p| p.value);
                    usize::from(ps.next() < ps.next())
                }
                7 => {
                    let mut ps = subpackets.as_ref().unwrap().iter().map(|p| p.value);
                    usize::from(ps.next() == ps.next())
                }
                _ => unreachable!("there shouldn't be an int heres"),
            },
        };

        Ok(Self {
            length,
            version,
            type_id,
            value,
            subpackets,
        })
    }
}

impl FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(hex_string: &str) -> anyhow::Result<Self> {
        let bin_string = hex_string
            .chars()
            .map(|v| format!("{:04b}", v.to_digit(16).expect("char not hex char.")))
            .join("");
        let chars = &mut bin_string.chars();
        Packet::from_chars(chars)
    }
}

pub fn part_1_solution(hex_string: String) -> usize {
    let encoding =
        Packet::from_str(hex_string.as_str()).expect("unable to build encoding from hex_string");
    encoding.sum_versions()
}
pub fn part_2_solution(hex_string: String) -> usize {
    let encoding =
        Packet::from_str(hex_string.as_str()).expect("unable to build encoding from hex_string");
    encoding.value
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn example1_part_1() {
        let hex_string = read_to_string(P.example_path("_1"))
            .unwrap()
            .trim() // get rid of trailing blank line
            .to_owned();

        let expected = 6;
        assert_eq!(expected, part_1_solution(hex_string))
    }

    #[test]
    fn example2_part_1() {
        let hex_string = read_to_string(P.example_path("_2"))
            .unwrap()
            .trim() // get rid of trailing blank line
            .to_owned();

        let expected = 9;
        assert_eq!(expected, part_1_solution(hex_string))
    }

    #[test]
    fn example3_part_1() {
        let hex_string = read_to_string(P.example_path("_3"))
            .unwrap()
            .trim() // get rid of trailing blank line
            .to_owned();

        let expected = 14;
        assert_eq!(expected, part_1_solution(hex_string))
    }
    #[test]
    fn example_others_part_1() {
        let hex_string = "8A004A801A8002F478".to_owned();
        let expected = 16;
        assert_eq!(expected, part_1_solution(hex_string));

        let hex_string = "620080001611562C8802118E34".to_owned();
        let expected = 12;
        assert_eq!(expected, part_1_solution(hex_string));

        let hex_string = "C0015000016115A2E0802F182340".to_owned();
        let expected = 23;
        assert_eq!(expected, part_1_solution(hex_string));

        let hex_string = "A0016C880162017C3686B18A3D4780".to_owned();
        let expected = 31;
        assert_eq!(expected, part_1_solution(hex_string));

        let hex_string = "9C005AC2F8F0".to_owned();
        let expected = 16;
        assert_eq!(expected, part_1_solution(hex_string));
    }

    #[test]
    fn input_part_1() {
        let hex_string = read_to_string(P.input_path())
            .unwrap()
            .trim() // get rid of trailing blank line
            .to_owned();

        let expected = 953;
        assert_eq!(expected, part_1_solution(hex_string))
    }

    #[test]
    fn example_part_2() {
        let hex_string = "C200B40A82".to_owned();
        let expected = 3;
        assert_eq!(expected, part_2_solution(hex_string));

        let hex_string = "04005AC33890".to_owned();
        let expected = 54;
        assert_eq!(expected, part_2_solution(hex_string));

        let hex_string = "880086C3E88112".to_owned();
        let expected = 7;
        assert_eq!(expected, part_2_solution(hex_string));

        let hex_string = "CE00C43D881120".to_owned();
        let expected = 9;
        assert_eq!(expected, part_2_solution(hex_string));

        let hex_string = "D8005AC2A8F0".to_owned();
        let expected = 1;
        assert_eq!(expected, part_2_solution(hex_string));

        let hex_string = "F600BC2D8F".to_owned();
        let expected = 0;
        assert_eq!(expected, part_2_solution(hex_string));

        let hex_string = "9C005AC2F8F0".to_owned();
        let expected = 0;
        assert_eq!(expected, part_2_solution(hex_string));

        let hex_string = "9C0141080250320F1802104A08".to_owned();
        let expected = 1;
        assert_eq!(expected, part_2_solution(hex_string));
    }

    #[test]
    fn input_part_2() {
        let hex_string = read_to_string(P.input_path())
            .unwrap()
            .trim() // get rid of trailing blank line
            .to_owned();

        let expected = 246225449979;
        assert_eq!(expected, part_2_solution(hex_string))
    }

    #[test]
    fn hex_bin() {
        let hex_string = "2";
        let bin_string = hex_string
            .chars()
            .map(|v| format!("{:b}", v.to_digit(16).expect("char not hex char.")))
            .join("");

        println!("hex {}", hex_string);
        println!("bin {}", bin_string);
    }
}
