use std::{collections::HashMap, error::Error};

fn parse(input: &str) -> HashMap<u64, u64> {
    input.split(',').fold(HashMap::new(), |mut i, c| {
        *i.entry(c.parse::<u64>().unwrap()).or_insert(0) += 1;
        i
    })
}

fn simulate(fish: HashMap<u64, u64>, days: u64) -> u64 {
    let mut result = fish;
    (0..days).for_each(|_| {
        result = result.iter().fold(HashMap::new(), |mut acc, (&k, v)| {
            if k == 0 {
                *acc.entry(8).or_insert(0) += v;
                *acc.entry(6).or_insert(0) += v;
            } else {
                *acc.entry(k - 1).or_insert(0) += v;
            }
            acc
        })
    });

    result.values().sum()
}

pub fn day061(input: &str) -> Result<u64, Box<dyn Error>> {
    Ok(simulate(parse(input), 80))
}

pub fn day062(input: &str) -> Result<u64, Box<dyn Error>> {
    Ok(simulate(parse(input), 256))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "3,4,3,1,2";

    #[test]
    fn day061_test() {
        assert_eq!(day061(INPUT).unwrap(), 5934);
    }

    #[test]
    fn day062_test() {
        assert_eq!(day062(INPUT).unwrap(), 26984457539);
    }
}
