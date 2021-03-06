use regex::Regex;

use super::input;

pub fn part_1() -> i32 {
    let lines = input::read_lines(2);

    // Format of the input lines with capture groups ("(...)")
    // to extract the passowrd rule and passowrd.
    let regex = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();

    let mut matches = 0;

    for line in lines {
        // Extract password rule and password from line.
        let caps = regex.captures(line.as_str()).unwrap();
        let letter = caps.get(3).unwrap().as_str();
        let min = caps.get(1).unwrap().as_str();
        let max = caps.get(2).unwrap().as_str();
        let password = caps.get(4).unwrap().as_str();

        // Create regex ^([^<letter>]*<letter>[^<letter>]*){<min>,<max>}$.
        let a = String::new()
            + "^([^"
            + letter
            + "]*"
            + letter
            + "[^"
            + letter
            + "]*){"
            + min
            + ","
            + max
            + "}$";
        let r = Regex::new(&a).unwrap();

        // Check if the password matches the rule.
        if r.is_match(password) {
            matches += 1;
        }
    }

    matches
}

pub fn part_2() -> i32 {
    let lines = input::read_lines(2);

    // Format of the input lines with capture groups ("(...)")
    // to extract the passowrd rule and passowrd.
    let regex = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();

    let mut matches = 0;

    for line in lines {
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
            matches += 1;
        }
    }

    matches
}
