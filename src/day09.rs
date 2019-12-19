use std::num::ParseIntError;

use crate::intcode::{Intcode, IoBus};

#[aoc_generator(day9)]
pub fn generate_input(input: &str) -> Result<Intcode, ParseIntError> {
    input.parse()
}

#[aoc(day9, part1)]
pub fn solve_part1(program: &Intcode) -> i64 {
    let mut output = IoBus::default();
    program.clone().execute_with_io(|| 1, &mut output);
    output.read().unwrap()
}

#[aoc(day9, part2)]
pub fn solve_part2(program: &Intcode) -> i64 {
    let mut output = IoBus::default();
    program.clone().execute_with_io(|| 2, &mut output);
    output.read().unwrap()
}
