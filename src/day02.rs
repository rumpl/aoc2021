use std::{error::Error, str::FromStr};

enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

#[derive(Debug, Clone)]
struct CommandParseError {
    message: String,
}

impl FromStr for Command {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        if parts.len() != 2 {
            return Err(CommandParseError {
                message: "not enough parts".into(),
            });
        }

        match parts[1].parse::<usize>() {
            Ok(amount) => match parts[0] {
                "forward" => Ok(Command::Forward(amount)),
                "up" => Ok(Command::Up(amount)),
                "down" => Ok(Command::Down(amount)),
                _ => Err(CommandParseError {
                    message: "unknown command".into(),
                }),
            },
            Err(_) => Err(CommandParseError {
                message: "unable to parse the amount".into(),
            }),
        }
    }

    type Err = CommandParseError;
}

#[derive(Default)]
struct Submarine {
    horizontal: usize,
    dept: usize,
    aim: usize,
}

fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .filter_map(|l| l.parse::<Command>().ok())
        .collect()
}

pub fn day021(input: &str) -> Result<usize, Box<dyn Error>> {
    let commands = parse(input);

    let mut submarine: Submarine = Default::default();

    for command in commands {
        match command {
            Command::Forward(d) => submarine.horizontal += d,
            Command::Up(d) => submarine.dept -= d,
            Command::Down(d) => submarine.dept += d,
        }
    }

    Ok(submarine.dept * submarine.horizontal)
}

pub fn day022(input: &str) -> Result<usize, Box<dyn Error>> {
    let commands = parse(input);
    let mut submarine: Submarine = Default::default();

    for command in commands {
        match command {
            Command::Forward(d) => {
                submarine.horizontal += d;
                submarine.dept += submarine.aim * d;
            }
            Command::Up(d) => submarine.aim -= d,
            Command::Down(d) => submarine.aim += d,
        }
    }

    Ok(submarine.dept * submarine.horizontal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day021_test() {
        assert_eq!(
            day021(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            )
            .unwrap(),
            150
        );
    }

    #[test]
    fn day022_test() {
        assert_eq!(
            day022(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            )
            .unwrap(),
            900
        );
    }
}
