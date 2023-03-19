use std::fs::File;
use std::io::{self, BufRead};

pub fn part1(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut max = 0;
    let mut group_sum = 0;

    for line in lines {
        let line = line.unwrap();

        if !line.is_empty() {
            let n = line.parse::<i32>().unwrap();
            group_sum += n;
            continue;
        }

        if group_sum > max {
            max = group_sum;
        }

        group_sum = 0;
    }

    if group_sum > max {
        max = group_sum;
    }

    return max;
}

pub fn part2(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut groups = Vec::new();
    let mut group_sum = 0;

    for line in lines {
        let line = line.unwrap();

        if !line.is_empty() {
            let n = line.parse::<i32>().unwrap();
            group_sum += n;
            continue;
        }

        if group_sum > 0 {
            groups.push(group_sum);
        }

        group_sum = 0;
    }

    if group_sum != 0 {
        groups.push(group_sum);
    }

    groups.sort();

    let total = groups
        .iter()
        .rev()
        .take(3)
        .copied()
        .reduce(|acc, x| acc + x)
        .unwrap();

    return total;
}
