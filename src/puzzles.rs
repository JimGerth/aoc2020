mod day_1;

pub fn run(day: i32) {
    match day {
        1 => {
            day_1::part_1();
            day_1::part_2();
        }
        _ => println!("Sorry, this puzzle is not implemented yet."),
    }
}
