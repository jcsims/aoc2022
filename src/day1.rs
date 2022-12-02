use crate::util;

fn sum_group(calories: &str) -> i64 {
    let mut accum = 0;
    for line in calories.lines() {
        accum += line.parse::<i64>().unwrap()
    }

    accum
}

fn sum_groups(calories: &str) -> Vec<i64> {
    calories
        .split("\n\n")
        .map(|group| sum_group(group))
        .collect()
}

fn max_calories(filepath: &str) -> i64 {
    sum_groups(&util::slurp(filepath)).into_iter().max().unwrap()
}

fn top_three_calories(filepath: &str) -> i64 {
    let mut groups = sum_groups(&util::slurp(filepath));
    groups.sort_unstable();

    groups[(groups.len() - 3)..].into_iter().sum()
}

pub fn part1() -> i64 {
    max_calories("data/d1.txt")
}

pub fn part2() -> i64 {
    top_three_calories("data/d1.txt")
}

#[test]
fn read_string() {
    assert_eq!(
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
",
        util::slurp("data/d1_test.txt")
    );
}

#[test]
fn part1_basic_test() {
    assert_eq!(24000, max_calories("data/d1_test.txt"));
}

#[test]
fn part1_test() {
    assert_eq!(73211, part1());
}

#[test]
fn sum_group_test() {
    assert_eq!(42, sum_group("10\n8\n24"));
}

#[test]
fn sum_groups_test() {
    assert_eq!(
        vec![42, 10, 42],
        sum_groups(
            "10
8
24

2
3
5

42"
        )
    )
}

#[test]
fn part2_basic_test() {
    assert_eq!(45000, top_three_calories("data/d1_test.txt"));
}

#[test]
fn part2_test() {
    assert_eq!(213958, part2());
}
