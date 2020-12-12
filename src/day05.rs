use itertools::Itertools;

#[derive(Debug)]
pub enum Instruction {
    Front,
    Back,
    Left,
    Right
}

#[derive(Eq, PartialEq, Debug)]
struct SeatPosition {
    row: u64,
    column: u64,
}

impl SeatPosition {
    fn get_id(&self) -> u64 {
        8 * self.row + self.column
    }
}

fn get_seat_position(boarding_card: &[Instruction]) -> SeatPosition {
    let mut rmin = 0;
    let mut rmax = 128;
    let mut cmin = 0;
    let mut cmax = 8;


    for instruction in boarding_card {
        use self::Instruction::*;
        match instruction {
            Front => {
                rmax = (rmax + rmin) / 2;
            },
            Back => {
                rmin = (rmax + rmin) / 2;
            },
            Left => {
                cmax = (cmax + cmin) / 2;
            },
            Right => {
                cmin = (cmax + cmin) / 2;
            }
        }
    }
    SeatPosition {row: rmin, column: cmin}
}

#[aoc_generator(day5)]
pub fn parse_input(i: &str) -> Vec<Vec<Instruction>> {
    i.split('\n')
        .map(|line| parse_boarding_card(line))
        .collect()
}

fn parse_boarding_card(i: &str) -> Vec<Instruction> {
    use self::Instruction::*;
    i.chars()
        .map(|c| match c {
            'L' => Left,
            'R' => Right,
            'F' => Front,
            'B' => Back,
            _ => panic!("Bad input")
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(cards: &[Vec<Instruction>]) -> u64 {
    cards.iter()
        .map(|card| get_seat_position(card).get_id())
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn part2(cards: &[Vec<Instruction>]) -> u64 {
    let filled_seat_ids = cards.iter()
        .map(|card| get_seat_position(card).get_id());

    filled_seat_ids.sorted()
        .tuple_windows()
        .filter(|(s1, s2)| *s2 == s1 + 2)
        .map(|(s1, s2)| s1 + 1)
        .nth(0)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_front() {
        let card = parse_boarding_card("FFFFFFFLLL");
        assert_eq!(SeatPosition {row: 0, column: 0}, get_seat_position(&card));
    }

    #[test]
    fn test_back() {
        let card = parse_boarding_card("BBBBBBBRRR");
        assert_eq!(SeatPosition {row: 127, column: 7}, get_seat_position(&card));
    }

    #[test]
    fn test_example() {
        let card = parse_boarding_card("FBFBBFFRLR");
        let result = get_seat_position(&card);
        assert_eq!(SeatPosition {row: 44, column: 5}, result);
        assert_eq!(357, result.get_id());
    }

}
