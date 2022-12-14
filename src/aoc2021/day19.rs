/// problem: https://adventofcode.com/2021/day/19
/// input: "https://adventofcode.com/2021/day/19/input"
use crate::Problem;
const P: Problem = Problem {
    year: 2021,
    day: 19,
    name: "Beacon Scanner",
};

use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::{Add, Sub},
    str::FromStr,
};

fn rotations() -> Vec<([usize; 3], [isize; 3])> {
    // is there an easier way to do this bullshitfn rotate([x,y,z]: [i32;3], rot: u8) -> [i32;3] {
    // can't collect into  Vec<[usize;3]> not sure why. need to try into with a map
    let axes: Vec<[usize; 3]> = [0usize, 1, 2]
        .into_iter()
        .permutations(3)
        .map(|v| {
            v.try_into().unwrap_or_else(|v: Vec<usize>| {
                panic!("Expected a Vec of length 3 but it was {}", v.len())
            })
        })
        .collect();
    let dirs: Vec<[isize; 3]> = [[-1, 1], [-1, 1], [-1, 1]]
        .into_iter()
        .multi_cartesian_product()
        .map(|v| {
            v.try_into().unwrap_or_else(|v: Vec<isize>| {
                panic!("Expected a Vec of length 3 but it was {}", v.len())
            })
        })
        .collect();
    axes.into_iter().cartesian_product(dirs).collect()
}

// impl FromStr for Probe {
//     type Err = anyhow::Error;

//     fn from_str(s: &str) -> anyhow::Result<Self> {
//         s.split(',').map(|c| c.parse::<isize>().unwrap()).collect()
//     }
// }

// impl ToString for Probe {
//     fn to_string(&self) -> String {
//         [self.0.to_string(), self.1.to_string(), self.2.to_string()]
//             .iter()
//             .join(",") // needs itertools
//     }
// }

// impl std::ops::Add for Probe {
//     type Output = Probe;

//     fn add(self, rhs: Self) -> Self::Output {
//         Probe(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
//     }
// }

// impl std::ops::Sub for Probe {
//     type Output = Probe;

//     fn sub(self, rhs: Self) -> Self::Output {
//         Probe(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
//     }
// }

// impl std::ops::Sub for &Probe {
//     type Output = Probe;

//     fn sub(self, rhs: Self) -> Self::Output {
//         Probe(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
//     }
// }

// #[derive(Debug, Clone)]
// pub struct Scanner {
//     // max 1000 units away
//     pub origin: Probe,
//     pub probes: Vec<Probe>,
// }

// impl FromIterator<Probe> for Scanner {
//     fn from_iter<T: IntoIterator<Item = Probe>>(iter: T) -> Self {
//         Scanner {
//             origin: Probe(0, 0, 0),
//             probes: iter.into_iter().collect::<Vec<Probe>>(),
//         }
//     }
// }

// fn manhattan(p1: Probe, p2: Probe) -> usize {
//     ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs()) as usize
// }

// pub fn part_1_solution(mut scanners: Vec<Scanner>) -> usize {
//     //
//     //  1. check distances to find matching probes
//     //  2. for 3 matching probes get vector between them
//     //      these 3 probes will be invariant
//     //  3. check rotations
//     //      pick one as anchor
//     //      rotate others until vecs between 3 target probes are the same
//     //  4. rotate all sensors to be correct
//     //  5. hashset the sensor probes
//     //  6. return len of hashset

//     let mut master_probes: HashSet<Probe> = scanners
//         .remove(0)
//         .probes
//         .into_iter()
//         .collect::<HashSet<_>>();
//     while !scanners.is_empty() {
//         for i in (0..scanners.len()).rev() {
//             println!("master_probes {}", master_probes.len());
//             let scanner = &scanners[i];
//             // find match
//             'rot: for rot_i in (0..24) {
//                 let rotated_scanner_probes = scanner
//                     .probes
//                     .iter()
//                     .map(|p| p.rotation(rot_i))
//                     .collect::<Vec<_>>();
//                 let distances = master_probes
//                     .iter()
//                     .cartesian_product(&rotated_scanner_probes)
//                     .map(|(&p1, &p2)| p2 - p1);

//                 for d in distances {
//                     let translated = rotated_scanner_probes.iter().map(|&p| p + d);
//                     if translated
//                         .clone()
//                         .filter(|v| master_probes.contains(v))
//                         .count()
//                         > 11
//                     {
//                         println!("translated: {}", translated.len());
//                         master_probes.extend(translated);
//                         scanners.swap_remove(i);
//                         break 'rot;
//                     }
//                 }
//             }
//         }
//         // rotate
//     }

//     master_probes.len()
// }
// pub fn part_2_solution(mut scanners: Vec<Scanner>) -> usize {
//     //
//     //  1. check distances to find matching probes
//     //  2. for 3 matching probes get vector between them
//     //      these 3 probes will be invariant
//     //  3. check rotations
//     //      pick one as anchor
//     //      rotate others until vecs between 3 target probes are the same
//     //  4. rotate all sensors to be correct
//     //  5. hashset the sensor probes
//     //  6. return len of hashset

//     let mut master_probes: HashSet<Probe> = scanners
//         .remove(0)
//         .probes
//         .into_iter()
//         .collect::<HashSet<_>>();
//     while !scanners.is_empty() {
//         for i in (0..scanners.len()).rev() {
//             println!("master_probes {}", master_probes.len());
//             let scanner = &scanners[i];
//             // find match
//             'rot: for rot_i in (0..24) {
//                 let rotated_scanner_probes = scanner
//                     .probes
//                     .iter()
//                     .map(|p| p.rotation(rot_i))
//                     .collect::<Vec<_>>();
//                 let distances = master_probes
//                     .iter()
//                     .cartesian_product(&rotated_scanner_probes)
//                     .map(|(&p1, &p2)| p2 - p1);

//                 for d in distances {
//                     let translated = rotated_scanner_probes.iter().map(|&p| p + d);
//                     if translated
//                         .clone()
//                         .filter(|v| master_probes.contains(v))
//                         .count()
//                         > 11
//                     {
//                         println!("translated: {}", translated.len());
//                         master_probes.extend(translated);
//                         scanners.swap_remove(i);
//                         break 'rot;
//                     }
//                 }
//             }
//         }
//         // rotate
//     }

//     master_probes
//         .iter()
//         .tuple_combinations()
//         .map(|(&p1, &p2)| manhattan(p1, p2))
//         .max()
//         .unwrap()
// }
// fn parse(input: &str) -> Vec<usize> {
//     input.lines().map(|l| l.parse().unwrap()).collect()
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use std::fs::read_to_string;

//     fn read_day19(path: impl AsRef<std::path::Path>) -> Vec<Scanner> {
//         let mut scanners = Vec::new();
//         let mut probes: Vec<Probe> = Vec::new();

//         for line in read_to_string(path).unwrap().lines() {
//             if line.is_empty() {
//                 scanners.push(probes.drain(..).into_iter().collect::<Scanner>());
//                 continue;
//             }
//             if line.contains("scanner") {
//                 println!("scanner {}", line);
//                 continue;
//             }
//             probes.push(Probe::from_str(line).expect("unexpected probe format"));
//         }
//         scanners.push(probes.drain(..).into_iter().collect::<Scanner>());
//         scanners
//     }

//     #[test]
//     fn test_rotations() {
//         let p = Probe(2, 3, 5);
//         let rotations: Vec<Probe> = (0..24).into_iter().map(|i| p.rotation(i)).collect();
//         assert!(rotations.len() == 24);
//         let h: HashSet<Probe> = HashSet::from_iter(rotations.clone());
//         assert!(h.len() == 24);
//     }

//     #[test]
//     fn example_part_1() {
//         let scanners = read_day19(P.example_path("_1"));
//         // let input = read_to_one_per_line::<T: FromStr>("src/aoc2021/data/x.example");
//         let expected = 79;
//         // part_1_solution(scanners);
//         assert_eq!(expected, part_1_solution(scanners))
//     }

//     #[test]
//     fn example_simple_part_1() {
//         let scanners = read_day19(P.example_path("_simple"));
//         for s in scanners.iter() {
//             println!("{:?}", s);
//         }
//         // let input = read_to_one_per_line::<T: FromStr>("src/aoc2021/data/x.example");
//         let expected = 1;
//         part_1_solution(scanners);
//         // assert_eq!(expected, part_1_solution(scanners))
//     }

//     #[test]
//     fn example_2d_part_1() {
//         let scanners = read_day19(P.example_path("_2D"));
//         for s in scanners.iter() {
//             println!("{:?}", s);
//         }
//         part_1_solution(scanners);
//         // let input = read_to_one_per_line::<T: FromStr>("src/aoc2021/data/x.example");
//         // let expected = 1;
//         // part_1_solution(scanners);
//         // assert_eq!(expected, part_1_solution(scanners))
//     }

//     #[test]
//     fn input_part_1() {
//         let scanners = read_day19(P.input_path());
//         // let input = read_to_one_per_line::<T: FromStr>("src/aoc2021/data/x.example");
//         let expected = 79;
//         // part_1_solution(scanners);
//         assert_eq!(expected, part_1_solution(scanners))
//     }

//     #[test]
//     fn example_part_2() {
//         let scanners = read_day19(P.example_path("_1"));
//         // let input = read_to_one_per_line::<T: FromStr>("src/aoc2021/data/x.example");
//         let expected = 3621;
//         // part_1_solution(scanners);
//         assert_eq!(expected, part_2_solution(scanners))
//     }

//     #[test]
//     fn input_part_2() {
//         let scanners = read_day19(P.input_path());
//         // let input = read_to_one_per_line::<T: FromStr>("src/aoc2021/data/x.example");
//         let expected = 16793;
//         // part_1_solution(scanners);
//         assert_eq!(expected, part_2_solution(scanners))
//     }
// }

// /*

// // Unfortunately, there's a second problem: the scanners
//     // also don't know their rotation or facing direction. Due
//     // to magnetic alignment, each scanner is rotated some integer
//     // number of 90-degree turns around all of the x, y, and z axes.
//     // That is, one scanner might call a direction positive x,
//     // while another scanner might call that direction negative y.

//     // Or, two scanners might agree on which direction is positive x,
//     // but one scanner might be upside-down from the perspective of
//     // the other scanner. In total, each scanner could be in any of
//     // 24 different orientations: facing positive or negative x, y, or z,
//     // and considering any of four directions "up" from that facing.
// let (rotx, roty, rotz) = [
//     (x, y, z),   // current
//     (x, -z, y),  // rot about x
//     (x, -y, -z), // rot about x*2
//     (x, z, -y),  // rot about x*3
//     (z, y, -x),  // rot about y
//     (-x, y, -z), // rot about y*2
//     (-z, y, x),  // rot about y*3
//     (-y, x, z),  // rot about z
//     (-x, -y, z), // rot about z*2
//     (y, -x, z),  // rot about z*3
//     // transpose maintaining rh rule
//     (y, z, x),
//     // (y, -x, z), // rot about x  duplicated above
//     (y, -z, -x), // rot about x*2
//     (y, x, -z),  // rot about x*3
//     // (x, z, -y), // rot about y  duplicated above
//     (-y, z, -x), // rot about y*2
//     (-x, z, y),  // rot about y*3
//     // (-z, y, x), // rot about z  duplicated above
//     (-y, -z, x), // rot about z*2
//     (z, -y, x),  // rot about z*3
//     // transpose maintaining rh rule
//     (z, x, y),
//     // (z, -y, x), // rot about x  duplicated above
//     (z, -x, -y), // rot about x*2
//     // (z, y, -x), // rot about x*3 duplicated above
//     // (y, x, -z), // rot about y  duplicated above
//     (-z, x, -y), // rot about y*2
//     // (-y, x, z), // rot about y*3  duplicated above
//     // (-x, z, y), // rot about z  duplicated above
//     (-z, -x, y), // rot about z*2
//     // (x, -z, y), // rot about z*3  duplicated above

//     // negatives (rot y y x)
//     (-z, -y, -x), // transpose maintaining rhr
//     (-x, -z, -y), // transpose maintaining rhr
//     (-y, -x, -z), // transpose maintaining rhr

//     */
