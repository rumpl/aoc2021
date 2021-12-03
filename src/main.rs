use std::{error::Error, fs};

mod day01;
mod day02;
mod day03;

macro_rules! day {
    ($day:ident, $part:ident) => {
        let contents = fs::read_to_string(concat!("./inputs/", stringify!($day), ".txt"))?;
        match $day::$part(&contents) {
            Ok(s) => println!(concat!(stringify!($part), ": {}"), s),
            Err(error) => eprintln!("{}", error),
        };
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    day!(day01, day11);
    day!(day01, day12);

    day!(day02, day021);
    day!(day02, day022);

    day!(day03, day031);
    day!(day03, day032);

    Ok(())
}
