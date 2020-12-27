use lazy_static::lazy_static;
use regex::Regex;
use crate::day14::Instruction::{SetMask, SetMemory};
use itertools::Itertools;
use std::iter;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct Mask {
    zeroes: u64,
    ones: u64,
    floating: u64
}

impl Mask {
    pub fn new() -> Mask {
        Mask {zeroes: 0, ones: 0, floating: 0}
    }

    pub fn from_string(s: &str) -> Mask {
        let mut zeroes = 0;
        let mut ones = 0;
        let mut floating = 0;
        for (i, c) in s.chars().rev().enumerate() {
            let bit = 1 << i;
            match c {
                '0' => zeroes |= bit,
                '1' => ones |= bit,
                'X' => floating |= bit,
                _ => panic!("Bad mask value")
            }
        }
        Mask { zeroes, ones, floating }
    }

    pub fn apply(&self, mut value: u64) -> u64 {
        value |= self.ones;
        value &= !self.zeroes;
        value
    }

    pub fn get_addresses(&self, value: usize) -> Vec<usize> {
        let floating_bits = (0..64)
            .filter(|i| self.floating & (1 << i) > 0)
            .collect_vec();

        let mut result = Vec::new();

        for mut x in 0..(1 << floating_bits.len()) {
            let mut mask = Mask::new();
            mask.ones = self.ones;
            for (i, bit_index) in floating_bits.iter().enumerate() {
                if x & (1 << i) > 0 {
                    mask.ones |= (1 << bit_index);
                } else {
                    mask.zeroes |= (1 << bit_index);
                }
            }
            result.push(mask.apply(value as u64) as usize);
        }

        result
    }
}

pub enum Instruction {
    SetMask(Mask),
    SetMemory((usize, u64))
}

#[aoc_generator(day14)]
pub fn parse_input(input: &str) -> Vec<Instruction> {
    input.split('\n')
        .map(|s| parse_instruction(s).unwrap())
        .collect_vec()
}

fn parse_instruction(s: &str) -> Option<Instruction> {
    parse_set_memory(s)
        .or_else(|| parse_set_mask(s))
}

fn parse_set_mask(s: &str) -> Option<Instruction> {
    lazy_static! {
        static ref REGEX : Regex = Regex::new(
                r"mask = ([01X]+)"
            ).unwrap();
    }
    let matches = REGEX.captures(s)?;
    Some(
        SetMask(Mask::from_string(&matches[1]))
    )
}

fn parse_set_memory(s: &str) -> Option<Instruction> {
    lazy_static! {
        static ref REGEX : Regex = Regex::new(
                r"mem\[(\d+)\] = (\d+)"
            ).unwrap();
    }
    let matches = REGEX.captures(s)?;
    Some(
        SetMemory((matches[1].parse().unwrap(), matches[2].parse().unwrap()))
    )
}

#[aoc(day14, part1)]
pub fn part1(instructions: &[Instruction]) -> u64 {
    let mut memory: HashMap<usize, u64> = HashMap::new();
    let mut mask = Mask::new();

    for i in instructions {
        match i {
            SetMask(m) => mask = *m,
            SetMemory((index, value)) => {
                memory.insert(*index, mask.apply(*value));
            }
        }
    }
    memory.values().sum()
}


#[aoc(day14, part2)]
pub fn part2(instructions: &[Instruction]) -> u64 {
    let mut memory: HashMap<usize, u64> = HashMap::new();
    let mut mask = Mask::new();

    for i in instructions {
        match i {
            SetMask(m) => mask = *m,
            SetMemory((index, value)) => {
                for addr in mask.get_addresses(*index) {
                    memory.insert(addr, *value);
                }
            }
        }
    }
    memory.values().sum()
}
