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

fn find_range_that_sums(numbers: &[i64], target: i64) -> Option<&[i64]> {
    let mut lo = 0;
    let mut hi = 0;
    let mut sum = 0;

    loop {
        if sum == target {
            return Some(&numbers[lo..hi]);
        } else if sum < target {
            hi += 1;
            if hi > numbers.len() {
                break;
            }
            sum += numbers[hi - 1];
        } else {
            lo += 1;
            sum -= numbers[lo - 1];
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

#[aoc(day9, part2)]
pub fn part2(numbers: &[i64]) -> i64 {
    let target = part1(numbers);
    let result = find_range_that_sums(numbers, target).unwrap();
    result.iter().min().unwrap() + result.iter().max().unwrap()
}
