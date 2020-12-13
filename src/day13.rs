use self::Bus::*;
use itertools::Itertools;
use modinverse::modinverse;

#[derive(Debug)]
enum Bus {
    InService(u64),
    OutOfService
}

impl Bus {
    fn get_id(&self) -> u64 {
        match self {
            InService(id) => *id,
            OutOfService => panic!("No id")
        }
    }
}

pub struct Input {
    arrival_at_seaport_time: u64,
    buses: Vec<Bus>
}

#[aoc_generator(day13)]
pub fn parse_input(s: &str) -> Input {
    let lines = s.split('\n').collect::<Vec<_>>();

    Input {
        arrival_at_seaport_time: lines[0].parse().unwrap(),
        buses: parse_buses(lines[1])
    }
}

fn parse_buses(s: &str) -> Vec<Bus> {
    let parse_bus = |s| match s {
        "x" => OutOfService,
        _ => InService(s.parse().unwrap())
    };

    s.split(',').map(parse_bus).collect()
}

#[aoc(day13, part1)]
pub fn part1(input: &Input) -> u64 {
    let to_catch = input.buses.iter()
        .filter(|b| matches!(b, InService(_)))
        .min_by_key(|b| wait_for_next_departure(b, input.arrival_at_seaport_time).unwrap())
        .unwrap();

    match to_catch {
        InService(id) => id * wait_for_next_departure(to_catch, input.arrival_at_seaport_time).unwrap(),
        _ => panic!("Bad bus")
    }
}

fn wait_for_next_departure(bus: &Bus, now: u64) -> Option<u64> {
    match bus {
        OutOfService => None,
        InService(id) => {
            let m = now % id;
            let to_wait = if m == 0 {0} else {id - m};
            Some(to_wait)
        }
    }
}

#[aoc(day13, part2)]
pub fn part2(input: &Input) -> i64 {
    first_departure_with_pattern(&input.buses)
}

fn first_departure_with_pattern(buses: &[Bus]) -> i64 {
    let congruences = buses.iter().enumerate()
        .filter(|(_, b)| matches!(b, InService(_)))
        .map(|(i, b)|
            Congruence {
                residue: b.get_id() as i64 - (i as i64),
                modulus: b.get_id() as i64
            }
        )
        .collect_vec();

    solve_congruences_crt(&congruences).unwrap()
}

#[derive(Debug)]
struct Congruence {
    modulus: i64,
    residue: i64
}

fn solve_congruences_crt(congruences: &[Congruence]) -> Option<i64> {
    let prod: i64 = congruences.iter()
        .map(|c| c.modulus)
        .product();

    let mut sum = 0;
    for congruence in congruences {
        let p = prod / congruence.modulus;
        sum += congruence.residue * modinverse(p, congruence.modulus)? * p;
    }
    Some(sum % prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let buses = parse_buses("17,x,13,19");
        assert_eq!(3417, first_departure_with_pattern(&buses));
    }

    #[test]
    fn test_2() {
        let buses = parse_buses("67,7,59,61");
        assert_eq!(754018, first_departure_with_pattern(&buses));
    }

    #[test]
    fn test_3() {
        let buses = parse_buses("67,x,7,59,61");
        assert_eq!(779210, first_departure_with_pattern(&buses));
    }

    #[test]
    fn test_4() {
        let buses = parse_buses("67,7,x,59,61");
        assert_eq!(1261476, first_departure_with_pattern(&buses));
    }

    #[test]
    fn test_5() {
        let buses = parse_buses("1789,37,47,1889");
        assert_eq!(1202161486, first_departure_with_pattern(&buses));
    }

    #[test]
    fn test_6() {
        let buses = parse_buses("7,13,x,x,59,x,31,19");
        assert_eq!(1068781, first_departure_with_pattern(&buses));
    }
}
