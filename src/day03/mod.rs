use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

const LC_A_ORD: i32 = 'a' as i32;
const UC_A_ORD: i32 = 'A' as i32;

pub fn part1(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    return io::BufReader::new(file)
        .lines()
        .map(|s| {
            let s = s.unwrap();
            let l = s.len() / 2;
            let first: HashSet<char, RandomState> = HashSet::from_iter(s[..l].chars().into_iter());
            let second: HashSet<char, RandomState> = HashSet::from_iter(s[l..].chars().into_iter());
            let c = Iterator::next(&mut first.intersection(&second))
                .unwrap()
                .clone();
            match c {
                'a'..='z' => c as i32 - LC_A_ORD + 1,
                'A'..='Z' => c as i32 - UC_A_ORD + 27,
                _ => unreachable!(),
            }
        })
        .sum::<i32>();
}

pub fn part2(path: &str) -> i32 {
    return 0;
}
