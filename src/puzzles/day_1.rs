use std::fs::File;
use std::io::{self, BufRead};

pub fn part_1() {
    let input = read_input();
    for a in input.iter() {
        for b in input.iter() {
            if a + b == 2020 {
                println!("The answer to part 1 is {}.", a * b);
                return;
            }
        }
    }
}

pub fn part_2() {
    let input = read_input();
    for a in input.iter() {
        for b in input.iter() {
            for c in input.iter() {
                if a + b + c == 2020 {
                    println!("The answer to part 2 is {}.", a * b * c);
                    return;
                }
            }
        }
    }
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
