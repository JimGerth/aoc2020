mod day_1;
mod day_2;
mod day_3;
mod day_4;

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
        _ => println!("Sorry, this puzzle is not implemented yet."),
    }
}

mod tests {
    #[test]
    fn test_day_1() {
        assert_eq!(super::day_1::part_1(), 32064);
        assert_eq!(super::day_1::part_2(), 193598720);
    }

    #[test]
    fn test_day_2() {
        assert_eq!(super::day_2::part_1(), 640);
        assert_eq!(super::day_2::part_2(), 472);
    }

    #[test]
    fn test_day_3() {
        assert_eq!(super::day_3::part_1(), 187);
        assert_eq!(super::day_3::part_2(), 4723283400);
    }

    #[test]
    fn test_day_4() {
        assert_eq!(super::day_4::part_1(), 213);
        assert_eq!(super::day_4::part_2(), 147);
    }
}
