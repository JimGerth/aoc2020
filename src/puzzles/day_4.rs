use regex::Regex;

use super::input;

pub fn part_1() -> i32 {
    let passports = input::read_paragraphs(4);

    let mut valids = 0;

    for passport in passports {
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

    valids
}

pub fn part_2() -> i32 {
    let passports = input::read_paragraphs(4);

    let mut valids = 0;

    for passport in passports {
        let fields: Vec<&str> = passport.split_whitespace().collect();

        if check_fields(&fields) {
            valids += 1;
        }
    }

    valids
}

fn check_fields(fields: &[&str]) -> bool {
    let mut byr = false;
    let mut iyr = false;
    let mut eyr = false;
    let mut hgt = false;
    let mut hcl = false;
    let mut ecl = false;
    let mut pid = false;

    for field in fields.iter() {
        match &field[..3] {
            "byr" => {
                if let Ok(value) = field[4..].parse::<i32>() {
                    byr = (1920..=2002).contains(&value)
                }
            }
            "iyr" => {
                if let Ok(value) = field[4..].parse::<i32>() {
                    iyr = (2010..=2020).contains(&value)
                }
            }
            "eyr" => {
                if let Ok(value) = field[4..].parse::<i32>() {
                    eyr = (2020..=2030).contains(&value)
                }
            }
            "hgt" => match &field[field.len() - 2..] {
                "cm" => {
                    if let Ok(value) = field[4..field.len() - 2].parse::<i32>() {
                        hgt = (150..=193).contains(&value)
                    }
                }
                "in" => {
                    if let Ok(value) = field[4..field.len() - 2].parse::<i32>() {
                        hgt = (59..=76).contains(&value)
                    }
                }
                _ => {}
            },
            "hcl" => {
                hcl = Regex::new(r"^#[0-9 a-f]{6}$")
                    .unwrap()
                    .is_match(&field[4..])
            }
            "ecl" => match &field[4..] {
                "amb" => ecl = true,
                "blu" => ecl = true,
                "brn" => ecl = true,
                "gry" => ecl = true,
                "grn" => ecl = true,
                "hzl" => ecl = true,
                "oth" => ecl = true,
                _ => {}
            },
            "pid" => pid = Regex::new(r"^[0-9]{9}$").unwrap().is_match(&field[4..]),
            _ => {}
        }
    }

    byr && iyr && eyr && hgt && hcl && ecl && pid
}
