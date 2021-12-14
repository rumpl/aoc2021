use std::{collections::HashMap, error::Error};

#[derive(Debug)]
struct Input {
    template: String,
    rules: HashMap<String, String>,
}

fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let template = lines.next().unwrap();
    lines.next();

    let mut i = Input {
        template: String::from(template),
        rules: HashMap::new(),
    };

    for l in lines {
        let n: Vec<&str> = l.split(" -> ").collect();
        i.rules
            .insert(String::from(n[0].trim()), String::from(n[1].trim()));
    }

    i
}

fn simulate(input: Input, times: u64) -> u64 {
    let mut mm = HashMap::new();

    for i in 0..=(input.template.len() - 2) {
        mm.insert(
            format!(
                "{}{}",
                input.template.as_bytes()[i] as char,
                input.template.as_bytes()[i + 1] as char,
            ),
            1,
        );
    }

    (0..times).for_each(|_| {
        let mut cur = HashMap::new();
        for n in mm.keys() {
            let rule = input.rules.get(n).unwrap();

            let key1 = format!("{}{}", n.as_bytes()[0] as char, rule);
            *cur.entry(key1).or_insert(0) += mm.get(n).unwrap_or(&0);

            let key2 = format!("{}{}", rule, n.as_bytes()[1] as char);
            *cur.entry(key2).or_insert(0) += mm.get(n).unwrap_or(&0);
        }
        mm = cur.clone();
    });

    let mut res = HashMap::new();
    for (d, times) in mm {
        let mut ch = d.chars();
        let first = ch.next().unwrap();
        *res.entry(first).or_insert(0) += times;
    }

    *res.entry(input.template.as_bytes()[input.template.len() - 1] as char)
        .or_insert(0) += 1;

    res.values().into_iter().max().unwrap() - res.values().into_iter().min().unwrap()
}

pub fn day141(input: &str) -> Result<u64, Box<dyn Error>> {
    Ok(simulate(parse(input), 10))
}

pub fn day142(input: &str) -> Result<u64, Box<dyn Error>> {
    Ok(simulate(parse(input), 40))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    // 6, 2

    #[test]
    fn day141_test() {
        assert_eq!(day141(INPUT).unwrap(), 1588);
    }

    #[test]
    fn day142_test() {
        assert_eq!(day142(INPUT).unwrap(), 2188189693529);
    }
}
