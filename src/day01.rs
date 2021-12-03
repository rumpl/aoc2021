use std::error::Error;

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|i| i.parse::<usize>().unwrap()).collect()
}

pub fn day11(input: &str) -> Result<usize, Box<dyn Error>> {
    Ok(parse(input).windows(2).filter(|w| w[0] < w[1]).count())
}

pub fn day12(input: &str) -> Result<usize, Box<dyn Error>> {
    Ok(parse(input).windows(4).filter(|a| a[0] < a[3]).count())
}
