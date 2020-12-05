use std::fs::File;
use std::io::{self, BufRead};

use regex::Regex;

pub fn part_1() {
    let file = File::open("src/inputs/day_2.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();

    // Format of the input lines with capture groups ("(...)")
    // to extract the passowrd rule and passowrd.
    let regex = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();

    let mut matches = 0;

    while let Some(Ok(line)) = lines.next() {
        print!("{}", line);

        // Extract password rule and password from line.
        let caps = regex.captures(line.as_str()).unwrap();
        let letter = caps.get(3).unwrap().as_str();
        let min = caps.get(1).unwrap().as_str();
        let max = caps.get(2).unwrap().as_str();
        let password = caps.get(4).unwrap().as_str();

        // Create regex ^([^<letter>]*<letter>[^<letter>]*){<min>,<max>}$.
        let mut a = String::new();
        a.push_str("^([^");
        a.push_str(letter);
        a.push_str("]*");
        a.push_str(letter);
        a.push_str("[^");
        a.push_str(letter);
        a.push_str("]*){");
        a.push_str(min);
        a.push_str(",");
        a.push_str(max);
        a.push_str("}$");
        let r = Regex::new(&a).unwrap();

        // Check if the password matches the rule.
        if r.is_match(password) {
            println!(" is a match!");
            matches += 1;
        } else {
            println!(" is not a match...");
        }
    }

    println!("The answer to part 1 is: There are {} matches.", matches);
}

pub fn part_2() {
    let file = File::open("src/inputs/day_2.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();

    // Format of the input lines with capture groups ("(...)")
    // to extract the passowrd rule and passowrd.
    let regex = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();

    let mut matches = 0;

    while let Some(Ok(line)) = lines.next() {
        print!("{}", line);

        // Extract password rule and password from line.
        let caps = regex.captures(line.as_str()).unwrap();
        let letter = caps.get(3).unwrap().as_str().chars().next().unwrap();
        let first: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let second: usize = caps.get(2).unwrap().as_str().parse().unwrap();
        let password: Vec<char> = caps.get(4).unwrap().as_str().chars().collect();

        // Check if the passowrd matches the rule.
        if (password[first - 1] == letter && password[second - 1] != letter)
            || (password[first - 1] != letter && password[second - 1] == letter)
        {
            println!(" is a match!");
            matches += 1;
        } else {
            println!(" is not a match...");
        }
    }

    println!("The answer to part 2 is: There are {} matches.", matches);
}
