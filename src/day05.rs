use std::num::ParseIntError;

use crate::intcode::Intcode;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Result<Intcode, ParseIntError> {
    input.parse()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Intcode) -> i64 {
    let mut input = input.clone();
    let mut diagnostic_code = 0;
    input.execute_with_io(|| 1, |v| diagnostic_code = v);
    diagnostic_code
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Intcode) -> i64 {
    let mut input = input.clone();
    let mut diagnostic_code = 0;
    input.execute_with_io(|| 5, |v| diagnostic_code = v);
    diagnostic_code
}
