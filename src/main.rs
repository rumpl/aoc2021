use std::{error::Error, fs};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

mod day14;

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

    day!(day04, day041);
    day!(day04, day042);

    day!(day05, day051);
    day!(day05, day052);

    day!(day06, day061);
    day!(day06, day062);

    day!(day07, day071);
    day!(day07, day072);

    day!(day08, day081);
    day!(day08, day082);

    day!(day09, day091);
    day!(day09, day092);

    day!(day10, day101);
    day!(day10, day102);

    day!(day14, day141);
    day!(day14, day142);

    Ok(())
}
