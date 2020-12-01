#[aoc_generator(day1)]
pub fn parse_ints(input: &str) -> Vec<u64> {
    let mut result = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();
    result.sort();
    result
}

fn two_sum(sorted_input: &[u64], target: u64) -> Option<(u64, u64)> {
    let mut i = 0;
    let mut j = sorted_input.len() - 1;
    while i < j {
        let x = sorted_input[i] + sorted_input[j];
        if x == target {
            return Some((sorted_input[i], sorted_input[j]));
        } else if x > target {
            j -= 1;
        } else {
            i += 1;
        }
    }
    None
}

fn three_sum(sorted_input: &[u64], target: u64) -> Option<(u64, u64, u64)> {
    for (i, &x) in sorted_input.iter().enumerate() {
        let result = two_sum(&sorted_input[(i+1)..], target - x);
        match result {
            Some((y, z)) => return Some((x, y, z)),
            None => {},
        }
    }
    None
}

#[aoc(day1, part1)]
pub fn part1(sorted_input: &[u64]) -> u64 {
    let (x, y) = two_sum(sorted_input, 2020).unwrap();
    x * y
}

#[aoc(day1, part2)]
pub fn part2(sorted_input: &[u64]) -> u64 {
    let (x, y, z) = three_sum(sorted_input, 2020).unwrap();
    x * y * z
}
