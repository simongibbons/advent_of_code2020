use std::collections::HashMap;

pub struct Passport {
    data: HashMap<String, String>
}

impl Passport {
    pub fn has_expected_keys(&self) -> bool {
        let expected_keys = [
            "byr",
            "iyr",
            "eyr",
            "hgt",
            "hcl",
            "ecl",
            "pid"
        ];
        expected_keys.iter().all(|&k| self.data.contains_key(k))
    }

    pub fn is_valid(&self) -> bool {
        self.valid_year_field("byr", 1920, 2002) &&
            self.valid_year_field("iyr", 2010, 2020) &&
            self.valid_year_field("eyr", 2020, 2030)
    }

    fn valid_year_field(&self, k: &str, min: u64, max: u64) -> bool {
        let year = match self.get_int(k) {
            Some(v) => v,
            None => return false
        };

        year >= min && year <= max
    }

    fn valid_height(&self) {
        // T
    }

    fn get_int(&self, k: &str) -> Option<u64> {
        let v = self.data.get(k)?;
        match v.parse() {
            Ok(v) => Some(v),
            Err(_) => None
        }
    }
}


#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<Passport> {
    input.split("\n\n")
        .map(|s| parse_one(s))
        .collect()
}

fn parse_one(input: &str) -> Passport {
    let mut result = HashMap::new();
    for s in input.split_whitespace() {
        let pair = s.split(":").collect::<Vec<_>>();
        result.insert(pair[0].to_string(), pair[1].to_string());
    }
    Passport {data: result}
}

#[aoc(day4, part1)]
pub fn part1(input: &[Passport]) -> usize {
    input.iter()
        .filter(|p| p.has_expected_keys())
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Passport]) -> usize {
    input.iter()
        .filter(|p| p.is_valid())
        .count()
}
