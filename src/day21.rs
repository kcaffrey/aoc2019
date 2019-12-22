use std::num::ParseIntError;

use crate::intcode::{Intcode, IoBus};

#[aoc_generator(day21)]
pub fn generate_input(input: &str) -> Result<Intcode, ParseIntError> {
    input.parse()
}

#[aoc(day21, part1)]
pub fn solve_part1(program: &Intcode) -> i64 {
    let mut input = IoBus::default();
    let mut output = IoBus::default();
    let mut springdroid = program.clone();

    let program = "NOT B J
NOT C T
OR T J
NOT A T
OR T J
AND D J
WALK\n";
    input.write_str(program);

    springdroid.execute_with_io(&mut input, &mut output);
    println!("{}", output.read_str());
    output.read().unwrap_or(-1)
}

#[aoc(day21, part2)]
pub fn solve_part2(program: &Intcode) -> i64 {
    let mut input = IoBus::default();
    let mut output = IoBus::default();
    let mut springdroid = program.clone();

    let program = "NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
NOT E T
NOT T T
OR H T
AND T J
RUN\n";
    input.write_str(program);

    springdroid.execute_with_io(&mut input, &mut output);
    println!("{}", output.read_str());
    output.read().unwrap_or(-1)
}
