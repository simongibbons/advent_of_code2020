fn two_sum(numbers: &[i64], target: i64) -> Option<(i64, i64)> {
    for (i, &x) in numbers.iter().enumerate() {
        for y in numbers[i+1..].iter() {
            if x + y == target {
                return Some((x, *y));
            }
        }
    }
    None
}

#[aoc_generator(day9)]
pub fn parse_numbers(input: &str) -> Vec<i64> {
    input.split('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(numbers: &[i64]) -> i64 {
    for i in 25..numbers.len() {
        match two_sum(&numbers[i - 25..i],numbers[i]) {
            Some(_) => {},
            None => return numbers[i]
        }
    }
    panic!("No solution")
}
