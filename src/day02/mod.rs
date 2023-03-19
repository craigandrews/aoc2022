use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

const WIN_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;
const LOSE_SCORE: i32 = 0;

#[derive(Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Choice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Choice::Rock),
            "B" => Ok(Choice::Paper),
            "C" => Ok(Choice::Scissors),
            "X" => Ok(Choice::Rock),
            "Y" => Ok(Choice::Paper),
            "Z" => Ok(Choice::Scissors),
            _ => Err(s.to_string()),
        }
    }
}

trait Score {
    fn to_score(&self, opponent: Choice) -> i32;
}

impl Score for Choice {
    fn to_score(&self, opponent: Choice) -> i32 {
        match self {
            Choice::Rock => {
                1 + match opponent {
                    Choice::Rock => DRAW_SCORE,
                    Choice::Paper => LOSE_SCORE,
                    Choice::Scissors => WIN_SCORE,
                }
            }
            Choice::Paper => {
                2 + match opponent {
                    Choice::Rock => WIN_SCORE,
                    Choice::Paper => DRAW_SCORE,
                    Choice::Scissors => LOSE_SCORE,
                }
            }
            Choice::Scissors => {
                3 + match opponent {
                    Choice::Rock => LOSE_SCORE,
                    Choice::Paper => WIN_SCORE,
                    Choice::Scissors => DRAW_SCORE,
                }
            }
        }
    }
}

enum DesiredOutcome {
    Win,
    Lose,
    Draw,
}

impl FromStr for DesiredOutcome {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(DesiredOutcome::Lose),
            "Y" => Ok(DesiredOutcome::Draw),
            "Z" => Ok(DesiredOutcome::Win),
            _ => Err(s.to_string()),
        }
    }

    type Err = String;
}

trait Choose {
    fn to_choice(&self, opponent: Choice) -> Choice;
}

impl Choose for DesiredOutcome {
    fn to_choice(&self, opponent: Choice) -> Choice {
        match self {
            DesiredOutcome::Win => match opponent {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
            },
            DesiredOutcome::Lose => match opponent {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
            },
            DesiredOutcome::Draw => opponent,
        }
    }
}

pub fn part1(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let strategy = io::BufReader::new(file).lines().map(|s| {
        (
            s.as_ref().unwrap()[0..1].parse::<Choice>().unwrap(),
            s.as_ref().unwrap()[2..3].parse::<Choice>().unwrap(),
        )
    });

    return strategy
        .into_iter()
        .map(|turn| turn.1.to_score(turn.0))
        .sum::<i32>();
}

pub fn part2(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let strategy = io::BufReader::new(file).lines().map(|s| {
        (
            s.as_ref().unwrap()[0..1].parse::<Choice>().unwrap(),
            s.as_ref().unwrap()[2..3].parse::<DesiredOutcome>().unwrap(),
        )
    });

    return strategy
        .into_iter()
        .map(|turn| turn.1.to_choice(turn.0.clone()).to_score(turn.0))
        .sum::<i32>();
}
