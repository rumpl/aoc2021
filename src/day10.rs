use std::error::Error;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn is_open(c: char) -> bool {
    c == '{' || c == '[' || c == '<' || c == '('
}

fn is_close(c: char) -> bool {
    c == '}' || c == ']' || c == '>' || c == ')'
}

fn is_good(open: char, close: char) -> bool {
    match open {
        '(' => close == ')',
        '[' => close == ']',
        '{' => close == '}',
        '<' => close == '>',
        _ => unreachable!(),
    }
}

fn score(brace: char) -> i64 {
    match brace {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn score2(brace: char) -> i64 {
    match brace {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!(),
    }
}

pub fn day101(input: &str) -> Result<i64, Box<dyn Error>> {
    let lines = parse(input);
    let mut result = 0;

    for line in lines {
        let mut braces: Vec<char> = vec![];
        for brace in line {
            if is_open(brace) {
                braces.push(brace);
            }
            if is_close(brace) {
                let b = braces.pop().unwrap();
                if !is_good(b, brace) {
                    result += score(brace);
                }
            }
        }
    }
    Ok(result)
}

pub fn day102(input: &str) -> Result<i64, Box<dyn Error>> {
    let lines = parse(input);

    let mut results = vec![];

    for line in lines {
        let mut braces: Vec<char> = vec![];
        for brace in line {
            if is_open(brace) {
                braces.push(brace);
            }
            if is_close(brace) {
                let b = braces.pop().unwrap();
                if !is_good(b, brace) {
                    braces = vec![];
                    break;
                }
            }
        }

        if !braces.is_empty() {
            let mut score = 0;
            for b in braces.into_iter().rev() {
                score *= 5;
                score += score2(b);
            }
            results.push(score);
        }
    }
    results.sort_unstable();
    Ok(results[results.len() / 2])
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]] 
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn day101_test() {
        assert_eq!(day101(INPUT).unwrap(), 26397);
    }

    #[test]
    fn day102_test() {
        assert_eq!(day102(INPUT).unwrap(), 288957);
    }
}
