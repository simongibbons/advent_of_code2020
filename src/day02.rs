use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct Input {
    policy: Policy,
    password: String
}

#[derive(Debug)]
pub struct Policy {
    letter: char,
    position1: usize,
    position2: usize
}

impl Policy {
    fn password_is_valid_part1(&self, password: &str) -> bool {
        let count = password.chars().filter(|&c| c == self.letter).count();
        count >= self.position1 && count <= self.position2
    }

    fn password_is_valid_part2(&self, password: &str) -> bool {
        char_at_matches(password, self.position1 - 1, self.letter) ^
            char_at_matches(password, self.position2 - 1, self.letter)
    }
}

fn char_at_matches(s: &str, index: usize, target: char) -> bool {
    s.chars().nth(index)
        .map(|c| c == target)
        .unwrap_or(false)
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<Input> {
    input.split("\n")
        .map(|l| parse_one_input(l).unwrap())
        .collect()
}

fn parse_one_input(input: &str) -> Option<Input> {
    lazy_static! {
        static ref INPUT_REGEX : Regex = Regex::new(
                r"(\d+)-(\d+) (.): (.+)"
            ).unwrap();
    }
    let caps = INPUT_REGEX.captures(input)?;
    Some(Input {
        password: caps[4].to_string(),
        policy: Policy {
            position1: caps[1].parse().unwrap(),
            position2: caps[2].parse().unwrap(),
            letter: caps[3].chars().nth(0).unwrap(),
        }
    })
}

#[aoc(day2, part1)]
pub fn part1(input: &[Input]) -> usize {
    input.iter()
        .filter(|i| i.policy.password_is_valid_part1(&i.password))
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Input]) -> usize {
    input.iter()
        .filter(|i| i.policy.password_is_valid_part2(&i.password))
        .count()
}
