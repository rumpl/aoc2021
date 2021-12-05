use std::{collections::HashMap, error::Error, ops::Range};

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
            return self
                .rows()
                .into_iter()
                .map(|y| Pos { x: self.start.x, y })
                .collect();
        }
        if self.start.y == self.end.y {
            return self
                .columns()
                .into_iter()
                .map(|x| Pos { x, y: self.start.y })
                .collect();
        }
        let cols: Box<dyn Iterator<Item = u32>> = if self.start.x > self.end.x {
            Box::new(self.columns().rev())
        } else {
            Box::new(self.columns())
        };
        let rows: Box<dyn Iterator<Item = u32>> = if self.start.y > self.end.y {
            Box::new(self.rows().rev())
        } else {
            Box::new(self.rows())
        };

        cols.zip(rows)
            .into_iter()
            .map(|(x, y)| Pos { x, y })
            .collect()
    }

    fn columns(&self) -> Range<u32> {
        if self.start.x < self.end.x {
            self.start.x..self.end.x + 1
        } else {
            self.end.x..self.start.x + 1
        }
    }

    fn rows(&self) -> Range<u32> {
        if self.start.y < self.end.y {
            self.start.y..self.end.y + 1
        } else {
            self.end.y..self.start.y + 1
        }
    }
}

fn parse_points(start: &str, end: &str) -> Cloud {
    let a = start.split(',').collect::<Vec<&str>>();
    let b = end.split(',').collect::<Vec<&str>>();

    Cloud {
        start: Pos {
            x: a[0].parse().unwrap(),
            y: a[1].parse().unwrap(),
        },
        end: Pos {
            x: b[0].parse().unwrap(),
            y: b[1].parse().unwrap(),
        },
    }
}

fn parse(input: &str) -> Vec<Cloud> {
    input
        .lines()
        .map(|line| line.split(" -> ").collect::<Vec<&str>>())
        .map(|points| parse_points(points[0], points[1]))
        .collect()
}

pub fn day051(input: &str) -> Result<usize, Box<dyn Error>> {
    let clouds = parse(input);

    let hor_ver: Vec<Cloud> = clouds
        .into_iter()
        .filter(|cloud| cloud.is_hor_ver())
        .collect();

    let mut ccs = HashMap::new();
    for c in hor_ver {
        for p in c.positions() {
            let key = p.key();
            match &ccs.get(&key) {
                Some(&v) => &ccs.insert(key, v + 1),
                None => &ccs.insert(key, 1),
            };
        }
    }

    Ok(ccs.values().into_iter().filter(|&v| *v > 1).count())
}

pub fn day052(input: &str) -> Result<usize, Box<dyn Error>> {
    let clouds = parse(input);

    let mut ccs = HashMap::new();
    for c in clouds {
        for p in c.positions() {
            let key = p.key();
            match &ccs.get(&key) {
                Some(&v) => &ccs.insert(key, v + 1),
                None => &ccs.insert(key, 1),
            };
        }
    }

    Ok(ccs.values().into_iter().filter(|&v| *v > 1).count())
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
