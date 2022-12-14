use clap::Parser;
/// scaffold to generate template. see: https://github.com/fspoettel/advent-of-code-rust
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    process,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// day for scaffold
    #[clap(index = 1, value_name = "day")]
    day: u8,

    /// year for scaffold, defaults to 2022
    #[clap(
        default_value_t = 2022,
        index = 2,
        value_name = "year (defaults to 2022)"
    )]
    year: u16,
}

const TEMPLATE: &str = r###"use crate::Problem;
/// problem: https://adventofcode.com/$YEAR/day/$DAY
/// input: https://adventofcode.com/$YEAR/day/$DAY/input

const P: Problem = Problem {
    year: $YEAR,
    day: $DAY,
    name: "TODO",
};

fn pt1(input: &str) -> usize {
    let parsed = parse(input);
    todo!()
}

fn pt2(input: &str) -> usize {
    let parsed = parse(input);
    todo!()
}

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn pt1_example() {
        let input = read_to_string(P.example_path("_1")).expect("no such file");
        assert_eq!(pt1(&input), 0);
    }

    #[test]
    fn pt1_input() {
        let input = read_to_string(P.input_path()).expect("no such file");
        assert_eq!(pt1(&input), 0);
    }

    #[test]
    fn pt2_example() {
        let input = read_to_string(P.example_path("_1")).expect("no such file");
        assert_eq!(pt2(&input), 0);
    }

    #[test]
    fn pt2_input() {
        let input = read_to_string(P.input_path()).expect("no such file");
        assert_eq!(pt2(&input), 0);
    }
}
"###;

/// creates a file with write access. returns an error if the file already exists.
fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

/// gets a file with append access. returns an error if the file already exists.
fn append_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().append(true).open(path)
}

fn main() {
    let args = Args::parse();
    let day = args.day;
    let year = args.year;
    let day_str = format!("day{:02}", day); // e.g. day04

    let input_path = format!("inputs/aoc{}/inputs/{}.txt", year, &day_str);
    let example_path = format!("inputs/aoc{}/examples/{}_1.txt", year, &day_str); // defaults to "_1"
    let src_path = format!("src/aoc{}/{}.rs", year, &day_str);
    let mod_file_path = format!("src/aoc{}.rs", year);

    // make src file
    match create_file(&src_path) {
        Err(e) => {
            eprintln!("Failed to create src file at \"{}\". {}", &src_path, e);
            process::exit(1);
        }
        Ok(mut file) => {
            match file.write_all(
                TEMPLATE
                    .replace("$DAY", &day.to_string())
                    .replace("$YEAR", &year.to_string())
                    .as_bytes(),
            ) {
                Ok(_) => {
                    println!("Created module file \"{}\"", &src_path);
                }
                Err(e) => {
                    eprintln!("Failed to write module contents: {}", e);
                    process::exit(1);
                }
            }
        }
    };

    // update mod file
    match append_file(&mod_file_path) {
        Ok(mut f) => {
            f.write(format!("pub mod {};\n", &day_str).as_bytes());
            println!(
                "Updated mod file \"{}\" with: \"pub mod {};\"",
                &mod_file_path, &day_str
            );
        }
        Err(e) => {
            eprintln!(
                "Failed to append to mod file: \"{}\". {}",
                &mod_file_path, e
            );
            process::exit(1);
        }
    }

    // make empty input file
    match create_file(&input_path) {
        Ok(_) => {
            println!("Created empty input file \"{}\"", &input_path);
        }
        Err(e) => {
            eprintln!("Failed to create input file: \"{}\". {}", &input_path, e);
            process::exit(1);
        }
    }

    // make empty example file 
    match create_file(&example_path) {
        Ok(_) => {
            println!("Created empty example file \"{}\"", &example_path);
        }
        Err(e) => {
            eprintln!(
                "Failed to create example file: \"{}\". {}",
                &example_path, e
            );
            process::exit(1);
        }
    }

    println!("--------------------------------");
    println!("🎄 Scaffold created for {} 🎄", &day_str);
    println!("--------------------------------");
}
