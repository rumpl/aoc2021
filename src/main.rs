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
    let mut count = 0;
    for i in 1..input.len() {
        if input[i - 1] < input[i] {
            count = count + 1;
        }
    }

    println!("{}", count);
}

fn day12(input: &Vec<u64>) {
    let mut sums: Vec<u64> = vec![];

    for i in 0..input.len() - 2 {
        sums.push(input[i] + input[i + 1] + input[i + 2]);
    }

    day11(&sums);
}
