use std::{error::Error, fs};

mod day01;
mod day02;

fn main() -> Result<(), Box<dyn Error>> {
    match day01::day01() {
        Ok(()) => {}
        Err(error) => eprintln!("{}", error),
    }

    let contents = fs::read_to_string("./inputs/day02.txt")?;
    match day02::day02(contents) {
        Ok(s) => println!("{}", s),
        Err(error) => eprintln!("{}", error),
    };

    Ok(())
}
