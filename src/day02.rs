use std::{error::Error, str::FromStr};

enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

#[derive(Debug, Clone)]
struct CommandParseError;

impl FromStr for Command {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        match parts[0] {
            "forward" => Ok(Command::Forward(parts[1].parse::<usize>().unwrap())),
            "up" => Ok(Command::Up(parts[1].parse::<usize>().unwrap())),
            "down" => Ok(Command::Down(parts[1].parse::<usize>().unwrap())),
            _ => Err(CommandParseError),
        }
    }

    type Err = CommandParseError;
}

struct Submarine {
    horizontal: usize,
    dept: usize,
}

pub fn day02(input: String) -> Result<usize, Box<dyn Error>> {
    let commands: Vec<Result<Command, CommandParseError>> =
        input.lines().map(|line| line.parse::<Command>()).collect();

    let cmds: Vec<&Command> = commands
        .iter()
        .filter(|c| !c.is_err())
        .map(|a| a.as_ref().unwrap())
        .collect();

    Ok(day021(cmds))
}

fn day021(commands: Vec<&Command>) -> usize {
    let mut submarine = Submarine {
        horizontal: 0,
        dept: 0,
    };

    for command in commands {
        match command {
            Command::Forward(d) => submarine.horizontal += d,
            Command::Up(d) => submarine.dept -= d,
            Command::Down(d) => submarine.dept += d,
        }
    }

    submarine.dept * submarine.horizontal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            day02(String::from(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            ))
            .unwrap(),
            150
        );
    }
}
