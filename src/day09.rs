use std::num::ParseIntError;

use crate::intcode::{Input, Intcode, IoBus, Output};

#[aoc_generator(day9)]
pub fn generate_input(input: &str) -> Result<Intcode, ParseIntError> {
    input.parse()
}

#[aoc(day9, part1)]
pub fn solve_part1(program: &Intcode) -> i64 {
    let mut output = IoBus::default();
    program.clone().execute_with_io(|| 1, &mut output);
    output.get_input().unwrap()
}

#[aoc(day9, part2)]
pub fn solve_part2(program: &Intcode) -> i64 {
    let mut output = IoBus::default();
    program.clone().execute_with_io(|| 2, &mut output);
    output.get_input().unwrap()
}
