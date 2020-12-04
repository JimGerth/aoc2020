use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: i32 = args
        .get(1)
        .expect("Specify what day's puzzle to run by typing in a number.")
        .parse()
        .expect("Only use a single number to specify what day's puzzle to run.");

    match day {
        _ => println!("Sorry, this puzzle is not implemented yet."),
    }
}
