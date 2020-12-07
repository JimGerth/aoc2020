use std::fs;

pub fn part_1() {
    let input = fs::read_to_string("src/inputs/day_4.txt").unwrap();
    let mut passports = input.split("\n\n");

    let mut valids = 0;

    while let Some(passport) = passports.next() {
        let fields: Vec<&str> = passport.split_whitespace().collect();

        if fields.len() == 8 {
            valids += 1;
        } else if fields.len() == 7 {
            valids += 1;
            for field in fields.iter() {
                if field.starts_with("cid") {
                    valids -= 1;
                }
            }
        }
    }

    println!("The answer to part 1 is {}", valids);
}

pub fn part_2() {
    println!("Sorry, this puzzle is not implemented yet.");
}
