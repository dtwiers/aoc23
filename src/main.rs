mod day1;

use inquire::Select;

enum Day {
    One,
    Two,
}

fn main() {
    let day = Select::new("Which day?").items(&["One", "Two"]).interact().unwrap();
    match day {
        0 => day1::run(),
        1 => println!("Day 2"),
        _ => println!("Invalid day"),
    }
}
