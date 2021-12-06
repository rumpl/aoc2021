use std::{collections::HashMap, error::Error};

#[derive(Debug)]
struct Pos {
    x: u32,
    y: u32,
}

impl Pos {
    fn key(&self) -> String {
        format!("{}-{}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Cloud {
    start: Pos,
    end: Pos,
}

impl Cloud {
    fn is_hor_ver(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn positions(&self) -> Vec<Pos> {
        if self.start.x == self.end.x {
            return self.rows().map(|y| Pos { x: self.start.x, y }).collect();
        }

        if self.start.y == self.end.y {
            return self.columns().map(|x| Pos { x, y: self.start.y }).collect();
        }

        let mut c;
        let mut cc;
        let cols: &mut dyn Iterator<Item = u32> = if self.start.x <= self.end.x {
            c = self.columns();
            &mut c
        } else {
            cc = self.columns().rev();
            &mut cc
        };

        let mut r;
        let mut rr;
        let rows: &mut dyn Iterator<Item = u32> = if self.start.y <= self.end.y {
            r = self.rows();
            &mut r
        } else {
            rr = self.rows().rev();
            &mut rr
        };

        cols.zip(rows).map(|(x, y)| Pos { x, y }).collect()
    }

    fn columns(&self) -> impl DoubleEndedIterator<Item = u32> + '_ {
        if self.start.x < self.end.x {
            self.start.x..self.end.x + 1
        } else {
            self.end.x..self.start.x + 1
        }
    }

    fn rows(&self) -> impl DoubleEndedIterator<Item = u32> + '_ {
        if self.start.y < self.end.y {
            self.start.y..self.end.y + 1
        } else {
            self.end.y..self.start.y + 1
        }
    }
}

fn parse_points(start: &str, end: &str) -> Cloud {
    let mut a = start.split(',');
    let mut b = end.split(',');

    Cloud {
        start: Pos {
            x: a.next().unwrap().parse().unwrap(),
            y: a.next().unwrap().parse().unwrap(),
        },
        end: Pos {
            x: b.next().unwrap().parse().unwrap(),
            y: b.next().unwrap().parse().unwrap(),
        },
    }
}

fn parse(input: &str) -> impl Iterator<Item = Cloud> + '_ {
    input
        .lines()
        .map(|line| line.split(" -> "))
        .map(|mut points| parse_points(points.next().unwrap(), points.next().unwrap()))
}

fn histo<T>(hor_ver: T) -> Result<usize, Box<dyn Error>>
where
    T: Iterator<Item = Cloud>,
{
    let mut ccs = HashMap::new();
    for c in hor_ver {
        ccs = c.positions().iter().fold(ccs, |mut acc, p| {
            *acc.entry(p.key()).or_insert(0) += 1;
            acc
        });
    }

    Ok(ccs.values().filter(|&v| *v > 1).count())
}

pub fn day051(input: &str) -> Result<usize, Box<dyn Error>> {
    histo(parse(input).filter(|cloud| cloud.is_hor_ver()))
}

pub fn day052(input: &str) -> Result<usize, Box<dyn Error>> {
    histo(parse(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn day051_test() {
        assert_eq!(day051(INPUT).unwrap(), 5);
    }

    #[test]
    fn day052_test() {
        assert_eq!(day052(INPUT).unwrap(), 12);
    }
}
