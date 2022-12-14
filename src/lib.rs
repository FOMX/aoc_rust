pub mod aoc2021;
pub mod aoc2022;
pub mod error;

use crate::error::Error;
use error::ParseError;
use std::{fs::read_to_string, path::Path, str::FromStr};

pub struct Problem {
    pub year: usize,
    pub day: usize,
    pub name: &'static str,
}

impl Problem {
    pub fn input_path(&self) -> impl AsRef<Path> {
        format!("./inputs/aoc{}/inputs/day{:02}.txt", self.year, self.day)
    }

    pub fn example_path(&self, id: &str) -> impl AsRef<Path> {
        format!("./inputs/aoc{}/examples/day{:02}{}.txt", self.year, self.day, id)
    }

}
pub trait Solution {
    type OutputPt1;
    type OutputPt2;
    fn pt1(data: &str) -> Self::OutputPt1;
    fn pt2(data: &str) -> Self::OutputPt2;
}

pub fn read_to_one_per_line<T>(path: impl AsRef<Path>) -> Result<Vec<T>, Error>
where
    T: FromStr,
{
    read_to_string(path)?
        .lines()
        .map(|l| match l.parse::<T>() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::ParseError(ParseError::FromStrError)),
        })
        .collect()
}

pub fn read_to_vec_per_line<T, F>(path: impl AsRef<Path>, f: F) -> Result<Vec<Vec<T>>, Error>
where
    T: FromStr,
    F: Fn(char) -> Option<T>,
{
    read_to_string(path)?
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match f(c) {
                    Some(v) => Ok(v),
                    None => Err(Error::ParseError(ParseError::FromStrError)),
                })
                .collect()
        })
        .collect()
}

pub fn read_to_chunks(path: impl AsRef<Path>) -> Result<Vec<String>, Error> {
    read_to_string(path)?
        .split("\r\n\r\n")
        .map(|s| Ok(s.to_owned()))
        .collect()
}
