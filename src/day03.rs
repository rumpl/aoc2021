use std::error::Error;

pub fn day031(input: &str) -> Result<usize, Box<dyn Error>> {
    let lines: Vec<&str> = input.lines().collect();
    let numbers: Vec<usize> = lines
        .iter()
        .filter_map(|i| usize::from_str_radix(i, 2).ok())
        .collect();

    let mut gama = 0;
    let mut epsilon = 0;

    let length = lines[0].len();

    for i in 0..length {
        let mut count_ones = 0;
        for l in &numbers {
            if l >> i & 0x1 == 1 {
                count_ones += 1;
            }
        }
        let mut most_common = 0;
        let mut least_common = 1;
        if count_ones > lines.len() / 2 {
            most_common = 1;
            least_common = 0;
        }

        gama |= most_common << i;
        epsilon |= least_common << i;
    }

    Ok(gama * epsilon)
}

fn count(lines: &Vec<&str>, i: usize) -> char {
    let mut count_ones = 0;

    for l in lines {
        if l.chars().nth(i).unwrap() == '1' {
            count_ones += 1;
        }
    }

    if 2 * count_ones >= lines.len() {
        return '1';
    }
    return '0';
}

pub fn day032(input: &str) -> Result<usize, Box<dyn Error>> {
    let lines: Vec<&str> = input.lines().collect();
    let mut o2 = lines.clone();
    let mut co2 = lines.clone();

    let length = lines[0].len();
    for i in 0..length {
        if o2.len() > 1 {
            let c = count(&o2, i);
            o2 = o2
                .into_iter()
                .filter(|a| a.chars().nth(i).unwrap() == c)
                .collect();
        }

        if co2.len() > 1 {
            let c = count(&co2, i);
            co2 = co2
                .into_iter()
                .filter(|a| a.chars().nth(i).unwrap() != c)
                .collect();
        }
    }

    Ok(usize::from_str_radix(o2[0], 2).unwrap() * usize::from_str_radix(co2[0], 2).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day031_test() {
        assert_eq!(
            day031(
                "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            )
            .unwrap(),
            198
        );
    }

    #[test]
    fn day032_test() {
        assert_eq!(
            day032(
                "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            )
            .unwrap(),
            230
        );
    }
}
