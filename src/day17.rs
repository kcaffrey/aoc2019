use std::iter;
use std::num::ParseIntError;

use crate::geometry::Point2D;
use crate::intcode::{Intcode, IoBus};

#[aoc_generator(day17)]
pub fn generate_input(input: &str) -> Result<Intcode, ParseIntError> {
    input.parse()
}

#[aoc(day17, part1)]
pub fn solve_part1(program: &Intcode) -> usize {
    let mut display = print_display(program);
    let mut alignment = 0;
    for y in 1..display.len() - 1 {
        for x in 1..display[y].len() - 1 {
            // println!("{}, {}", x, y);
            if display[y][x] != '.'
                && display[y - 1][x] != '.'
                && display[y + 1][x] != '.'
                && display[y][x - 1] != '.'
                && display[y][x + 1] != '.'
            {
                alignment += x * y;
                display[y][x] = 'O';
            }
        }
    }
    let s = display
        .iter()
        .flat_map(|line| line.iter().copied().chain(iter::once('\n')))
        .collect::<String>();
    println!("\n{}", s);
    alignment
}

fn print_display(program: &Intcode) -> Vec<Vec<char>> {
    let mut program = program.clone();
    let mut output = IoBus::default();
    program.execute_with_io(|| 0, &mut output);

    let mut screen = Vec::new();
    screen.push(Vec::new());
    let mut cur_line = screen.last_mut().unwrap();
    while let Some(o) = output.read() {
        if o == 10 {
            screen.push(Vec::new());
            cur_line = screen.last_mut().unwrap();
        } else {
            cur_line.push(o as u8 as char);
        }
    }
    screen.into_iter().filter(|line| !line.is_empty()).collect()
}
