use std::num::ParseIntError;

use crate::intcode::Intcode;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Intcode, ParseIntError> {
    input.parse()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Intcode) -> u32 {
    let mut input = input.clone();
    input.memory[1] = 12;
    input.memory[2] = 2;
    input.execute();
    input.memory[0]
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Intcode) -> u32 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut test = input.clone();
            test.memory[1] = noun;
            test.memory[2] = verb;
            test.execute();
            if test.memory[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("Couldn't find answer!");
}