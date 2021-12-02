use std::{error::Error, fs};

pub fn day01() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("./inputs/day01.txt")?;

    let input: Vec<u64> = contents
        .lines()
        .filter(|i| !i.is_empty())
        .map(|i| i.parse::<u64>().unwrap())
        .collect();

    day11(&input);
    day12(&input);

    Ok(())
}

fn day11(input: &[u64]) {
    println!("{}", input.windows(2).filter(|w| w[0] < w[1]).count());
}

fn day12(input: &[u64]) {
    println!("{}", input.windows(4).filter(|a| a[0] < a[3]).count());
}
