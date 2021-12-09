use std::{cmp::Ordering, collections::HashSet, error::Error, usize};

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|n| n.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn get(input: &Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
    *input
        .get(i as usize)
        .unwrap_or(&vec![])
        .get(j as usize)
        .unwrap_or(&i32::MAX)
}

pub fn day091(input: &str) -> Result<i32, Box<dyn Error>> {
    let numbers = parse(input);

    let mut res = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers[i].len() {
            let ii = i as i32;
            let jj = j as i32;
            if numbers[i][j] < get(&numbers, ii - 1, jj - 1)
                && numbers[i][j] < get(&numbers, ii - 1, jj)
                && numbers[i][j] < get(&numbers, ii - 1, jj + 1)
                && numbers[i][j] < get(&numbers, ii, jj - 1)
                && numbers[i][j] < get(&numbers, ii, jj + 1)
                && numbers[i][j] < get(&numbers, ii + 1, jj - 1)
                && numbers[i][j] < get(&numbers, ii + 1, jj)
                && numbers[i][j] < get(&numbers, ii + 1, jj + 1)
            {
                res += numbers[i][j] + 1;
            }
        }
    }

    Ok(res)
}

fn basin_size(numbers: &Vec<Vec<i32>>, i: i32, j: i32) -> HashSet<String> {
    let mut set = HashSet::new();

    set.insert(format!("{}-{}", i, j));

    if numbers[i as usize][j as usize] < get(numbers, i - 1, j - 1)
        && get(numbers, i - 1, j - 1) < 9
    {
        set.extend(basin_size(numbers, i - 1, j - 1));
    }
    if numbers[i as usize][j as usize] < get(numbers, i - 1, j) && get(numbers, i - 1, j) < 9 {
        set.extend(basin_size(numbers, i - 1, j));
    }
    if numbers[i as usize][j as usize] < get(numbers, i - 1, j + 1)
        && get(numbers, i - 1, j + 1) < 9
    {
        set.extend(basin_size(numbers, i - 1, j + 1));
    }

    if numbers[i as usize][j as usize] < get(numbers, i, j - 1) && get(numbers, i, j - 1) < 9 {
        set.extend(basin_size(numbers, i, j - 1));
    }
    if numbers[i as usize][j as usize] < get(numbers, i, j + 1) && get(numbers, i, j + 1) < 9 {
        set.extend(basin_size(numbers, i, j + 1));
    }

    if numbers[i as usize][j as usize] < get(numbers, i + 1, j - 1)
        && get(numbers, i + 1, j - 1) < 9
    {
        set.extend(basin_size(numbers, i + 1, j - 1));
    }
    if numbers[i as usize][j as usize] < get(numbers, i + 1, j) && get(numbers, i + 1, j) < 9 {
        set.extend(basin_size(numbers, i + 1, j));
    }
    if numbers[i as usize][j as usize] < get(numbers, i + 1, j + 1)
        && get(numbers, i + 1, j + 1) < 9
    {
        set.extend(basin_size(numbers, i + 1, j + 1));
    }

    set
}

pub fn day092(input: &str) -> Result<usize, Box<dyn Error>> {
    let numbers = parse(input);

    let mut basins: Vec<usize> = vec![];
    for i in 0..numbers.len() {
        for j in 0..numbers[i].len() {
            let ii = i as i32;
            let jj = j as i32;
            if numbers[i][j] < get(&numbers, ii - 1, jj - 1)
                && numbers[i][j] < get(&numbers, ii - 1, jj)
                && numbers[i][j] < get(&numbers, ii - 1, jj + 1)
                && numbers[i][j] < get(&numbers, ii, jj - 1)
                && numbers[i][j] < get(&numbers, ii, jj + 1)
                && numbers[i][j] < get(&numbers, ii + 1, jj - 1)
                && numbers[i][j] < get(&numbers, ii + 1, jj)
                && numbers[i][j] < get(&numbers, ii + 1, jj + 1)
            {
                let set = basin_size(&numbers, ii, jj);
                basins.push(set.len());
            }
        }
    }

    basins.sort_by(|a, b| {
        if a > b {
            Ordering::Less
        } else if a < b {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    Ok(basins[0] * basins[1] * basins[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn day091_test() {
        assert_eq!(day091(INPUT).unwrap(), 15);
    }

    #[test]
    fn day092_test() {
        assert_eq!(day092(INPUT).unwrap(), 1134);
    }
}
