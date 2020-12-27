use std::fs::{self, File};
use std::io::{self, BufRead};

/// Reads input file into a single String.
///
/// This reads the file at "src/inputs/day_[day].txt" which should be
/// downloaded from "https://adventofcode.com/2020/day/[day]/input".
pub fn read(day: i32) -> String {
    fs::read_to_string(format!("src/inputs/day_{}.txt", day)).expect(&format!(
        "Problems reading the input file for day {}. \
            Make sure src/inputs/day_{}.txt exists. \
            If not get it from https://adventofcode.com/2020/day/{}/input",
        day, day, day
    ))
}

/// Read input file of specific challenge.
///
/// This reads the file at "src/inputs/day_[day].txt" which should be
/// downloaded from "https://adventofcode.com/2020/day/[day]/input".
fn read_file(day: i32) -> File {
    File::open(format!("src/inputs/day_{}.txt", day)).expect(&format!(
        "Problems reading the input file for day {}. \
            Make sure src/inputs/day_{}.txt exists. \
            If not get it from https://adventofcode.com/2020/day/{}/input",
        day, day, day
    ))
}

/// Reads input file into a list of lines.
///
/// This reads the file at "src/inputs/day_[day].txt" which should be
/// downloaded from "https://adventofcode.com/2020/day/[day]/input".
pub fn read_lines(day: i32) -> Vec<String> {
    io::BufReader::new(read_file(day))
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

/// Parses numbers from input file per line.
///
/// This reads the file at "src/inputs/day_[day].txt" which should be
/// downloaded from "https://adventofcode.com/2020/day/[day]/input".
/// It is expected to only contain one number per line.
pub fn read_numbers(day: i32) -> Vec<i32> {
    io::BufReader::new(read_file(day))
        .lines()
        .map(|line| {
            line.unwrap()
                .parse()
                .expect(&format!(
                    "Problems while parsing input file src/inputs/day_{}.txt. \
                    Non numeric value in file. \
                    Consider replacing with official input file from https://adventofcode.com/2020/day/{}/input.",
                    day,
                    day
                ))
        })
        .collect()
}
