use std::{error::Error, usize, vec};

#[derive(Debug)]
struct Number {
    n: u32,
    seen: bool,
}

#[derive(Debug)]
struct Board {
    done: bool,
    numbers: Vec<Number>,
    width: u32,
    height: u32,
}

impl Board {
    fn new_number(&mut self, n: u32) {
        for number in self.numbers.iter_mut() {
            if n == number.n {
                number.seen = true
            }
        }
    }

    fn bingo(&self) -> bool {
        let mut win = true;
        for i in 0..self.width {
            win = true;
            for j in 0..self.height {
                let m = &self.numbers[(i * self.width + j) as usize];
                if !m.seen {
                    win = false;
                }
            }
            if win {
                return true;
            }
        }

        for i in 0..self.width {
            win = true;
            for j in 0..self.height {
                let m = &self.numbers[(i + j * self.height) as usize];
                if !m.seen {
                    win = false;
                }
            }
            if win {
                return true;
            }
        }

        win
    }

    fn result(&self, number: u32) -> usize {
        let not_seen: Vec<&Number> = self.numbers.iter().filter(|n| !n.seen).collect();
        let sum: u32 = not_seen.iter().map(|n| n.n).sum();
        (sum * number) as usize
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            done: Default::default(),
            numbers: Default::default(),
            width: 5,
            height: 5,
        }
    }
}

#[derive(Debug)]
struct Game {
    boards: Vec<Board>,
}

impl Game {
    fn new_number(&mut self, n: u32) {
        for board in self.boards.iter_mut() {
            board.new_number(n);
        }
    }

    fn bingo(&mut self) -> Vec<&Board> {
        let mut done: Vec<&Board> = vec![];
        for board in self.boards.iter_mut() {
            if board.done {
                continue;
            }

            if board.bingo() {
                board.done = true;
                done.push(board);
            }
        }

        done
    }
}

fn parse(input: &str) -> Result<(Vec<u32>, Vec<Board>), Box<dyn Error>> {
    let mut lines: Vec<&str> = input.lines().collect();
    let numbers: Vec<u32> = lines[0]
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = vec![];
    let board_numbers: Vec<&str> = lines.drain(2..lines.len()).collect();

    let mut board = Default::default();

    for line in board_numbers {
        if line.is_empty() {
            boards.push(board);
            board = Default::default();
        } else {
            let ns: Vec<u32> = line
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.trim().parse::<u32>().unwrap())
                .collect();
            let mut a: Vec<Number> = ns.iter().map(|n| Number { n: *n, seen: false }).collect();
            board.numbers.append(&mut a);
        }
    }

    boards.push(board);

    Ok((numbers, boards))
}

pub fn day041(input: &str) -> Result<usize, Box<dyn Error>> {
    let (numbers, boards) = parse(input)?;

    let mut game = Game { boards };

    for number in numbers {
        game.new_number(number);
        let boards = game.bingo();
        if boards.len() == 1 {
            return Ok(boards[0].result(number));
        }
    }

    Ok(0)
}

pub fn day042(input: &str) -> Result<usize, Box<dyn Error>> {
    let (numbers, boards) = parse(input)?;
    let len = boards.len();
    let mut game = Game { boards };
    let mut winners = 0;

    for number in numbers {
        game.new_number(number);
        let boards = game.bingo();
        if !boards.is_empty() {
            winners += boards.len();
            if winners == len {
                return Ok(boards.last().unwrap().result(number));
            }
        }
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day041_test() {
        assert_eq!(
            day041(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"
            )
            .unwrap(),
            4512
        );
    }

    #[test]
    fn day042_test() {
        assert_eq!(
            day042(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"
            )
            .unwrap(),
            1924
        );
    }
}
