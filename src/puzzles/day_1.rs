use std::fs::File;
use std::io::{self, BufRead};

pub fn part_1() {
    println!("Sorry, this puzzle is not implemented yet.");
}

pub fn part_2() {
    println!("Sorry, this puzzle is not implemented yet.");
}

fn read_input() -> Vec<i32> {
    let file = File::open("src/inputs/day_1.txt")
        .expect("Missing input file! Expected at src/inputs/day_1.txt");
    let lines: Vec<i32> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .parse()
                .expect("Non numeric value in input file!")
        })
        .collect();
    return lines;
}
