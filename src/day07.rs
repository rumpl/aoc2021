use std::error::Error;

fn parse(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect()
}

fn run(input: Vec<i64>, fun: fn(i64, i64, i64) -> i64) -> i64 {
    let &min = input.iter().min().unwrap();
    let &max = input.iter().max().unwrap();

    (min..max)
        .map(|n| input.iter().fold(0, |a, b| fun(n, a, *b)))
        .min()
        .unwrap()
}

pub fn day071(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(run(parse(input), |n, a, b| a + (b - n).abs()))
}

pub fn day072(input: &str) -> Result<i64, Box<dyn Error>> {
    Ok(run(parse(input), |n, a, b| {
        let b = (b - n).abs();
        a + (b * (b + 1)) / 2
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn day071_test() {
        assert_eq!(day071(INPUT).unwrap(), 37);
    }

    #[test]
    fn day072_test() {
        assert_eq!(day072(INPUT).unwrap(), 168);
    }
}
