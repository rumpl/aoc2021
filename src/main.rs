use std::{error::Error, fs};

mod day01;
mod day02;

fn main() -> Result<(), Box<dyn Error>> {
    match day01::day01() {
        Ok(()) => {}
        Err(error) => eprintln!("{}", error),
    }

    let contents = fs::read_to_string("./inputs/day02.txt")?;
    match day02::day021(&contents) {
        Ok(s) => println!("day 2, part 1: {}", s),
        Err(error) => eprintln!("{}", error),
    };
    match day02::day022(&contents) {
        Ok(s) => println!("day 2, part 2: {}", s),
        Err(error) => eprintln!("{}", error),
    };

    Ok(())
}
