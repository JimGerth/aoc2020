use std::fs::File;
use std::io::{self, BufRead};

pub fn part_1() -> i32 {
    let file = File::open("src/inputs/day_3.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines().enumerate();

    let mut trees = 0;

    while let Some((i, Ok(line))) = lines.next() {
        if line.chars().nth(i * 3 % line.len()).unwrap() == '#' {
            trees += 1;
        }
    }

    trees
}

struct Slope {
    right: usize,
    down: usize,
    trees: usize,
}

pub fn part_2() -> i64 {
    let file = File::open("src/inputs/day_3.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines().enumerate();

    let mut slopes = vec![
        Slope {
            right: 1,
            down: 1,
            trees: 0,
        },
        Slope {
            right: 3,
            down: 1,
            trees: 0,
        },
        Slope {
            right: 5,
            down: 1,
            trees: 0,
        },
        Slope {
            right: 7,
            down: 1,
            trees: 0,
        },
        Slope {
            right: 1,
            down: 2,
            trees: 0,
        },
    ];

    while let Some((i, Ok(line))) = lines.next() {
        for slope in slopes.iter_mut() {
            if i % slope.down == 0
                && line
                    .chars()
                    .nth(i * slope.right / slope.down % line.len())
                    .unwrap()
                    == '#'
            {
                slope.trees += 1;
            }
        }
    }

    slopes
        .iter()
        .fold(1, |product, slope| product * slope.trees as i64)
}
