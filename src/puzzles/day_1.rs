use super::input;

pub fn part_1() -> i32 {
    let input = input::read_numbers(1);
    for a in input.iter() {
        for b in input.iter() {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    0
}

pub fn part_2() -> i32 {
    let input = input::read_numbers(1);
    for a in input.iter() {
        for b in input.iter() {
            for c in input.iter() {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    0
}
