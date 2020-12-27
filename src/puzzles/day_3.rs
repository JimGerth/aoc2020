use super::input;

pub fn part_1() -> i32 {
    let lines = input::read_lines(3);

    let mut trees = 0;

    for (i, line) in lines.iter().enumerate() {
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
    let lines = input::read_lines(3);

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

    for (i, line) in lines.iter().enumerate() {
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
