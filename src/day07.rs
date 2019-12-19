use std::collections::VecDeque;
use std::num::ParseIntError;

use itertools::Itertools;

use crate::intcode::{Input, Intcode, Output};

#[derive(Clone, Debug)]
struct InputOutputPipe {
    values: VecDeque<i32>,
}

impl Default for InputOutputPipe {
    fn default() -> Self {
        Self {
            values: Default::default(),
        }
    }
}

impl Input for &mut InputOutputPipe {
    fn get_input(&mut self) -> Option<i32> {
        self.values.pop_front()
    }
}

impl Output for &mut InputOutputPipe {
    fn receive_output(&mut self, output: i32) {
        self.values.push_back(output);
    }
}

#[aoc_generator(day7)]
pub fn generate_input(input: &str) -> Result<Intcode, ParseIntError> {
    input.parse()
}

#[aoc(day7, part1)]
pub fn solve_part1(program: &Intcode) -> i32 {
    (0..5)
        .permutations(5)
        .map(|permutation| run_amplifier_circuit(program, &permutation, 0))
        .max()
        .unwrap()
}

fn run_amplifier_circuit(program: &Intcode, phase_settings: &[i32], input: i32) -> i32 {
    let mut io: Vec<InputOutputPipe> = vec![Default::default(); phase_settings.len() + 1];
    for i in 0..phase_settings.len() {
        (&mut io[i]).receive_output(phase_settings[i]);
    }
    (&mut io[0]).receive_output(input);
    for i in 0..phase_settings.len() {
        let mut output = 0;
        program
            .clone()
            .execute_with_io(&mut io[i], |val| output = val);
        (&mut io[i + 1]).receive_output(output);
    }
    (&mut io[phase_settings.len()]).get_input().unwrap()
}

#[aoc(day7, part2)]
pub fn solve_part2(program: &Intcode) -> i32 {
    (5..10)
        .permutations(5)
        .map(|permutation| run_feedback_amplifier_circuit(program, &permutation, 0))
        .max()
        .unwrap()
}

fn run_feedback_amplifier_circuit(program: &Intcode, phase_settings: &[i32], input: i32) -> i32 {
    let mut io: Vec<InputOutputPipe> = vec![Default::default(); phase_settings.len()];
    let mut amplifiers = (0..phase_settings.len())
        .map(|_| program.clone())
        .collect::<Vec<_>>();
    for i in 0..phase_settings.len() {
        (&mut io[i]).receive_output(phase_settings[i]);
    }
    (&mut io[0]).receive_output(input);
    while !amplifiers[phase_settings.len() - 1].is_halted() {
        for i in 0..phase_settings.len() {
            let mut temp = InputOutputPipe::default();
            amplifiers[i].execute_with_io(&mut io[i], &mut temp);
            while let Some(output) = (&mut temp).get_input() {
                io.get_mut((i + 1) % phase_settings.len())
                    .unwrap()
                    .receive_output(output);
            }
        }
    }
    (&mut io[0]).get_input().unwrap()
}
