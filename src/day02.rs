use std::num::ParseIntError;

use crate::intcode::Intcode;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Intcode, ParseIntError> {
    input.parse()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Intcode) -> i64 {
    let mut input = input.clone();
    *input.mem_access(1) = 12;
    *input.mem_access(2) = 2;
    input.execute();
    *input.mem_access(0)
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Intcode) -> i64 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut test = input.clone();
            *test.mem_access(1) = noun;
            *test.mem_access(2) = verb;
            test.execute();
            if *test.mem_access(0) == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("Couldn't find answer!");
}
