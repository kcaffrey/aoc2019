use std::num::ParseIntError;

use itertools::Itertools;

use crate::intcode::{Intcode, IoBus};

#[aoc_generator(day7)]
pub fn generate_input(input: &str) -> Result<Intcode, ParseIntError> {
    input.parse()
}

#[aoc(day7, part1)]
pub fn solve_part1(program: &Intcode) -> i64 {
    (0..5)
        .permutations(5)
        .map(|permutation| run_amplifier_circuit(program, &permutation, 0))
        .max()
        .unwrap()
}

fn run_amplifier_circuit(program: &Intcode, phase_settings: &[i64], input: i64) -> i64 {
    let mut io: Vec<IoBus> = vec![Default::default(); phase_settings.len() + 1];
    for i in 0..phase_settings.len() {
        io[i].write(phase_settings[i]);
    }
    io[0].write(input);
    for i in 0..phase_settings.len() {
        let mut output = 0;
        program
            .clone()
            .execute_with_io(&mut io[i], |val| output = val);
        io[i + 1].write(output);
    }
    io[phase_settings.len()].read().unwrap()
}

#[aoc(day7, part2)]
pub fn solve_part2(program: &Intcode) -> i64 {
    (5..10)
        .permutations(5)
        .map(|permutation| run_feedback_amplifier_circuit(program, &permutation, 0))
        .max()
        .unwrap()
}

fn run_feedback_amplifier_circuit(program: &Intcode, phase_settings: &[i64], input: i64) -> i64 {
    let mut io: Vec<IoBus> = vec![Default::default(); phase_settings.len()];
    let mut amplifiers = (0..phase_settings.len())
        .map(|_| program.clone())
        .collect::<Vec<_>>();
    for i in 0..phase_settings.len() {
        io[i].write(phase_settings[i]);
    }
    io[0].write(input);
    while !amplifiers[phase_settings.len() - 1].is_halted() {
        for i in 0..phase_settings.len() {
            let mut temp = IoBus::default();
            amplifiers[i].execute_with_io(&mut io[i], &mut temp);
            while let Some(output) = temp.read() {
                io[(i + 1) % phase_settings.len()].write(output);
            }
        }
    }
    io[0].read().unwrap()
}
