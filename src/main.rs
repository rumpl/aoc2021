use std::{error::Error, fs};

fn main() {
    match day1() {
        Ok(()) => {}
        Err(error) => eprintln!("{}", error),
    }
}

fn day1() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("./day1.txt")?;

    let input: Vec<u64> = contents
        .split('\n')
        .filter(|i| i.len() != 0)
        .map(|i| i.parse::<u64>().unwrap())
        .collect();

    day11(&input);
    day12(&input);

    Ok(())
}

fn day11(input: &Vec<u64>) {
    println!("{}", input.windows(2).filter(|w| w[0] < w[1]).count());
}

fn day12(input: &Vec<u64>) {
    day11(&input.windows(3).map(|a| a[0] + a[1] + a[2]).collect());
}
