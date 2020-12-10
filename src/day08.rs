use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use crate::day08::ExecutionResult::{InfiniteLoop, Success, Error};
use crate::day08::Instruction::ACC;

#[derive(Copy, Clone)]
pub enum Instruction {
    JMP(i64),
    ACC(i64),
    NOP(i64)
}

#[derive(Debug)]
enum ExecutionResult {
    InfiniteLoop,
    Success,
    Error
}

struct State {
    program: Vec<Instruction>,
    program_counter: i64,
    accumulator: i64,
}

impl State {
    fn initialise(program: Vec<Instruction>) -> State {
        State {
            program,
            program_counter: 0,
            accumulator: 0,
        }
    }

    fn step(&mut self) {
        match self.program[self.program_counter as usize] {
            Instruction::JMP(offset) => {
                self.program_counter += offset
            },
            Instruction::ACC(x) => {
                self.program_counter += 1;
                self.accumulator += x;
            },
            Instruction::NOP(_) => {
                self.program_counter += 1
            }
        }
    }

    fn run(&mut self) -> ExecutionResult {
        let mut seen_pcs = HashSet::new();
        loop {
            if seen_pcs.contains(&self.program_counter) {
                return InfiniteLoop;
            }
            seen_pcs.insert(self.program_counter);

            if self.program_counter == self.program.len() as i64 {
                return Success;
            }

            if self.program_counter < 0 || self.program_counter > self.program.len() as i64 {
                return Error;
            }
            self.step();
        }
    }
}

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> Vec<Instruction> {
    input.split("\n")
        .map(|s| parse_instruction(s))
        .collect()
}

fn parse_instruction(input: &str) -> Instruction {
    lazy_static! {
        static ref REGEX : Regex = Regex::new(
                r"([a-z]*) (.\d+)"
            ).unwrap();
    }
    let caps = REGEX.captures(input).unwrap();
    let val = caps[2].parse().unwrap();
    match &caps[1] {
        "jmp" => Instruction::JMP(val),
        "acc" => Instruction::ACC(val),
        "nop" => Instruction::NOP(val),
        _ => panic!("Unknown instruction")
    }
}

#[aoc(day8, part1)]
pub fn part1(program: &[Instruction]) -> i64 {
    let mut state = State::initialise(program.to_vec());
    state.run();
    state.accumulator
}

#[aoc(day8, part2)]
pub fn part2(program: &[Instruction]) -> i64 {
    for candidate in get_program_candidates(program) {
        let mut state = State::initialise(candidate);
        let result = state.run();
        if matches!(result, ExecutionResult::Success) {
            return state.accumulator;
        }
    }
    panic!("No Solution");
}

fn get_program_candidates(program: &[Instruction]) -> impl Iterator<Item=Vec<Instruction>> + '_ {
    (0..program.len())
        .map(move |i| get_program_candidate(program, i))
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
}

fn get_program_candidate(program: &[Instruction], i: usize) -> Option<Vec<Instruction>> {
    match program[i] {
        Instruction::ACC(_) => None,
        Instruction::NOP(x) => {
            let mut p = program.to_vec();
            p[i] = Instruction::JMP(x);
            Some(p)
        },
        Instruction::JMP(x) => {
            let mut p = program.to_vec();
            p[i] = Instruction::NOP(x);
            Some(p)
        }
    }
}
