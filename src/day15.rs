use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    error::Error,
};

#[derive(Debug)]
struct Graph {
    edges: HashMap<(i32, i32), HashMap<(i32, i32), u32>>,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Truc {
    u: (i32, i32),
    weight: u32,
}

impl Ord for Truc {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Truc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Graph {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: (i32, i32), v: (i32, i32), weight: u32) {
        *self
            .edges
            .entry(u)
            .or_insert_with(HashMap::new)
            .entry(v)
            .or_insert(0) = weight;
    }

    fn shortest(&self, source: (i32, i32)) -> HashMap<(i32, i32), u32> {
        let mut dist: HashMap<(i32, i32), u32> = HashMap::new();
        let mut lala = BinaryHeap::new();

        for k in self.edges.keys() {
            dist.insert(*k, u32::MAX);
        }

        dist.insert(source, 0);
        lala.push(Truc {
            u: source,
            weight: 0,
        });

        while let Some(Truc { u, weight: _ }) = lala.pop() {
            for v in &self.edges[&u] {
                let alt = dist[&u] + v.1;
                if alt < dist[v.0] {
                    dist.insert(*v.0, alt);
                    lala.push(Truc {
                        u: *v.0,
                        weight: alt,
                    });
                }
            }
        }

        dist
    }
}

fn parse(input: &str) -> (Graph, i32, i32) {
    let mut g = Graph::new();

    let mut j = 0;

    let lines: Vec<&str> = input.lines().collect();
    let li = lines.len() as i32;

    for (i, line) in lines.iter().enumerate() {
        j = 0;
        for c in line.chars() {
            let i = i as i32;
            if i >= 1 {
                g.add_edge((i - 1, j), (i, j), c.to_digit(10).unwrap());
            }
            if i < lines.len() as i32 - 1 {
                g.add_edge((i + 1, j), (i, j), c.to_digit(10).unwrap());
            }

            if j >= 1 {
                g.add_edge((i, j - 1), (i, j), c.to_digit(10).unwrap());
            }
            if j < line.len() as i32 - 1 {
                g.add_edge((i, j + 1), (i, j), c.to_digit(10).unwrap());
            }

            j += 1;
        }
    }

    (g, li - 1, j - 1)
}

pub fn day151(input: &str) -> Result<u32, Box<dyn Error>> {
    let (g, i, j) = parse(input);
    let s = g.shortest((0, 0));
    let n = s.get(&(i, j)).unwrap();
    Ok(*n)
}

fn bleh(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let size = lines.len();
    let new_size = size * 5;

    let input = input.replace("\n", "");
    let input = input.as_bytes();

    let mut s = String::new();
    for n in 0..new_size {
        for m in 0..new_size {
            let v = input[(n % size) * size + (m % size)] as char;
            let n = v.to_digit(10).unwrap() as u32
                + (m as u32 / size as u32)
                + (n as u32 / size as u32);
            s = format!("{}{}", s, wrap(n));
        }
        s = format!("{}\n", s);
    }

    s
}

fn wrap(v: u32) -> u32 {
    if v > 9 {
        v - 9
    } else {
        v
    }
}

pub fn day152(input: &str) -> Result<u32, Box<dyn Error>> {
    let (g, i, j) = parse(&bleh(input));
    let s = g.shortest((0, 0));
    let n = s.get(&(i, j)).unwrap();
    Ok(*n)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

    #[test]
    fn day151_test() {
        assert_eq!(day151(INPUT).unwrap(), 40);
    }

    #[test]
    fn day152_test() {
        assert_eq!(day152(INPUT).unwrap(), 315);
    }
}
