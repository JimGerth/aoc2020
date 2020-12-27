mod input;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

mod tests;

pub fn run(day: i32) {
    match day {
        1 => {
            println!("Running puzzle from day 1...");
            println!("The answer to part 1 is {}.", day_1::part_1());
            println!("The answer to part 2 is {}.", day_1::part_2());
        }
        2 => {
            println!("Running puzzle from day 2...");
            println!("The answer to part 1 is {}.", day_2::part_1());
            println!("The answer to part 2 is {}.", day_2::part_2());
        }
        3 => {
            println!("Running puzzle from day 3...");
            println!("The answer to part 1 is {}.", day_3::part_1());
            println!("The answer to part 2 is {}.", day_3::part_2());
        }
        4 => {
            println!("Running puzzle from day 4...");
            println!("The answer to part 1 is {}.", day_4::part_1());
            println!("The answer to part 2 is {}.", day_4::part_2());
        }
        5 => {
            println!("Running puzzle from day 5...");
            println!("The answer to part 1 is {}.", day_5::part_1());
            println!("The answer to part 2 is {}.", day_5::part_2().expect("<Failed to compute result>"));
        }
        _ => println!("Sorry, this puzzle is not implemented yet."),
    }
}
