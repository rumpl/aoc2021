use std::{collections::HashMap, error::Error};

fn parse(input: &str) -> Vec<Vec<usize>> {
    let lines: Vec<&str> = input.lines().collect();

    let numbers: Vec<&str> = lines
        .into_iter()
        .map(|line| line.split('|').collect::<Vec<&str>>()[1])
        .collect();

    numbers
        .into_iter()
        .map(|n| {
            n.trim()
                .split(' ')
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|n| n.len())
                .collect()
        })
        .collect()
}

fn parse_sorted(input: &str) -> Vec<String> {
    input
        .trim()
        .split(' ')
        .map(|s| {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort();
            chars.iter().collect()
        })
        .collect()
}

fn parse2(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let lines: Vec<&str> = input.lines().collect();

    lines
        .into_iter()
        .map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            (parse_sorted(parts[0]), parse_sorted(parts[1]))
        })
        .collect()
}

pub fn day081(input: &str) -> Result<usize, Box<dyn Error>> {
    let numbers = parse(input);

    let mut n = 0;
    for num in numbers {
        n += num
            .iter()
            .filter(|&&n| n == 2 || n == 3 || n == 4 || n == 7)
            .count()
    }

    Ok(n)
}

fn calc(first: &Vec<String>, second: &Vec<String>) -> usize {
    let mut known = HashMap::new();
    let mut one = "";
    let mut four = "";

    first
        .iter()
        .filter(|s| s.len() != 5 && s.len() != 6)
        .for_each(|num| {
            if num.len() == 2 {
                known.insert(num, 1);
                one = num;
            }

            if num.len() == 4 {
                known.insert(num, 4);
                four = num;
            }

            if num.len() == 3 {
                known.insert(num, 7);
            }

            if num.len() == 7 {
                known.insert(num, 8);
            }
        });

    first.iter().for_each(|num| {
        if num.len() == 5 {
            if one.chars().filter(|s| num.contains(*s)).count() == 2 {
                known.insert(num, 3);
            } else if four.chars().filter(|s| num.contains(*s)).count() == 2 {
                known.insert(num, 2);
            } else {
                known.insert(num, 5);
            }
        }

        if num.len() == 6 {
            if four.chars().all(|s| num.contains(s)) {
                known.insert(num, 9);
            } else if one.chars().all(|s| num.contains(s)) {
                known.insert(num, 0);
            } else {
                known.insert(num, 6);
            }
        }
    });

    let mut res = 0;
    for n in second {
        res = res * 10;
        res += known.get(n).unwrap();
    }

    res
}

pub fn day082(input: &str) -> Result<usize, Box<dyn Error>> {
    let numbers = parse2(input);

    let n = numbers.iter().map(|(f, s)| calc(f, s));

    Ok(n.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn day081_test() {
        assert_eq!(day081(INPUT).unwrap(), 26);
    }

    #[test]
    fn day082_test() {
        assert_eq!(day082(INPUT).unwrap(), 61229);
    }
}
