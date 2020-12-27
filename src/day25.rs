use itertools::Itertools;

pub struct PublicKeys {
    card: u64,
    door: u64
}

#[aoc_generator(day25)]
pub fn parse_input(input: &str) -> PublicKeys {
    let split = input.split("\n")
        .map(|x| x.parse().unwrap())
        .collect_vec();

    PublicKeys {
        card: split[0],
        door: split[1]
    }
}

fn iterate(input: u64, subject_number: u64) -> u64 {
    (input * subject_number) % 20201227
}

fn transform_subject(subject_number: u64, loop_size: usize) -> u64 {
    let mut result = 1;
    for _ in 0..loop_size {
        result = iterate(result, subject_number);
    }
    result
}

fn determine_private_key(public_key: u64) -> usize {
    let mut transformed = 1;
    for loop_size in 1.. {
        transformed = iterate(transformed, 7);
        if transformed == public_key {
            return loop_size;
        }
    }
    unreachable!();
}

fn generate_key(public_key_1: u64, private_key_2: usize) -> u64 {
    transform_subject(public_key_1, private_key_2)
}

#[aoc(day25, part1)]
pub fn part1(keys: &PublicKeys) -> u64 {
    let door_private_key = determine_private_key(keys.door);
    let card_private_key = determine_private_key(keys.card);

    let k1 = generate_key(keys.door, card_private_key);
    let k2 = generate_key(keys.card, door_private_key);
    assert_eq!(k1, k2);

    k1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_public_key() {
        assert_eq!(8, determine_private_key(5764801));
        assert_eq!(5764801, transform_subject(7, 8));

        assert_eq!(11, determine_private_key(17807724));
        assert_eq!(17807724, transform_subject(7, 11));
    }
}
