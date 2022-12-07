use crate::util;

#[derive(Copy, Clone, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

#[derive(Copy, Clone)]
struct Round {
    my_play: Shape,
    their_play: Shape,
}

impl Outcome {
    fn parse(outcome: &str) -> Outcome {
        match outcome {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Unknown outcome: {}", outcome),
        }
    }

    fn score(self) -> i64 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

impl Shape {
    fn parse(shape: &str) -> Shape {
        match shape {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => panic!("Unknown shape: {}", shape),
        }
    }

    fn score(self) -> i64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }
}

fn decide_round(round: Round) -> Outcome {
    match (round.my_play, round.their_play) {
        (Shape::Rock, Shape::Scissor) => Outcome::Win,
        (Shape::Paper, Shape::Rock) => Outcome::Win,
        (Shape::Scissor, Shape::Paper) => Outcome::Win,
        (x, y) if x == y => Outcome::Draw,
        (_, _) => Outcome::Lose,
    }
}

fn score_round(outcome: Outcome, round: Round) -> i64 {
    outcome.score() + round.my_play.score()
}

fn parse_round(round: &str) -> Round {
    let r = round.split_once(' ');

    match r {
        Some((t, m)) => Round {
            my_play: Shape::parse(m),
            their_play: Shape::parse(t),
        },
        _ => panic!("Can't parse round: {}", round),
    }
}

fn play_strategy_guide(filepath: &str) -> i64 {
    util::slurp(filepath)
        .lines()
        .map(parse_round)
        .map(|r| score_round(decide_round(r), r))
        .sum()
}

pub fn part1() -> i64 {
    play_strategy_guide("data/d2.txt")
}

fn parse_expected_play(round: &str) -> Round {
    let r = round.split_once(' ');

    match r {
        Some((t, o)) => match Outcome::parse(o) {
            Outcome::Win => match Shape::parse(t) {
                Shape::Paper => Round {
                    my_play: Shape::Scissor,
                    their_play: Shape::Paper,
                },
                Shape::Rock => Round {
                    my_play: Shape::Paper,
                    their_play: Shape::Rock,
                },
                Shape::Scissor => Round {
                    my_play: Shape::Rock,
                    their_play: Shape::Scissor,
                },
            },
            Outcome::Draw => {
                let shape = Shape::parse(t);
                Round {
                    my_play: shape,
                    their_play: shape,
                }
            }
            Outcome::Lose => match Shape::parse(t) {
                Shape::Paper => Round {
                    my_play: Shape::Rock,
                    their_play: Shape::Paper,
                },
                Shape::Rock => Round {
                    my_play: Shape::Scissor,
                    their_play: Shape::Rock,
                },
                Shape::Scissor => Round {
                    my_play: Shape::Paper,
                    their_play: Shape::Scissor,
                },
            },
        },
        _ => panic!("Unable to parse expected play: {}", round),
    }
}

fn play_to_ending(filepath: &str) -> i64 {
    util::slurp(filepath)
        .lines()
        .map(parse_expected_play)
        .map(|r| score_round(decide_round(r), r))
        .sum()
}

pub fn part2() -> i64 {
    play_to_ending("data/d2.txt")
}

#[test]
fn test_play() {
    assert_eq!(15, play_strategy_guide("data/d2_test.txt"));
}

#[test]
fn test_expected_play() {
    assert_eq!(12, play_to_ending("data/d2_test.txt"));
}

#[test]
fn part1_test() {
    assert_eq!(10624, part1())
}

#[test]
fn part2_test() {
    assert_eq!(14060, part2())
}
