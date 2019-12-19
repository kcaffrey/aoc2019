use std::collections::HashMap;
use std::num::ParseIntError;

use crate::geometry::Point2D;
use crate::intcode::{Intcode, IoBus};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[aoc_generator(day11)]
pub fn generate_input(input: &str) -> Result<Intcode, ParseIntError> {
    input.parse()
}

#[aoc(day11, part1)]
pub fn solve_part1(program: &Intcode) -> usize {
    paint_panels(program, 0).len()
}

#[aoc(day11, part2)]
pub fn solve_part2(program: &Intcode) -> String {
    let panel_colors = paint_panels(program, 1);

    let minx = panel_colors.keys().map(|k| k.x).min().unwrap();
    let maxx = panel_colors.keys().map(|k| k.x).max().unwrap();
    let miny = panel_colors.keys().map(|k| k.y).min().unwrap();
    let maxy = panel_colors.keys().map(|k| k.y).max().unwrap();
    let mut output = String::new();
    output.push('\n');
    for y in miny..=maxy {
        for x in minx..=maxx {
            let color = *panel_colors.get(&point2D!(x, y)).unwrap_or(&0);
            if color == 0 {
                output.push(' ');
            } else {
                output.push('▓');
            }
        }
        output.push('\n');
    }
    output
}

fn paint_panels(program: &Intcode, initial_panel: i64) -> HashMap<Point2D<i32>, i64> {
    let mut robot = program.clone();
    let mut input = IoBus::default();
    let mut output = IoBus::default();
    let mut position = Point2D::origin();
    let mut direction = Direction::Up;
    let mut panel_colors = HashMap::new();

    // Set initial panel color
    panel_colors.insert(position, initial_panel);

    while !robot.is_halted() {
        // Handle input
        let cur_color = *panel_colors.get(&position).unwrap_or(&0);
        input.write(cur_color);

        // Execute robot
        robot.execute_with_io(&mut input, &mut output);

        // Handle output
        let (new_color, rotation) = (output.read().unwrap(), output.read().unwrap());
        panel_colors.insert(position, new_color);
        direction = direction.rotate(rotation);
        position = offset_point(position, direction);
    }

    panel_colors
}

fn offset_point(point: Point2D<i32>, direction: Direction) -> Point2D<i32> {
    let offset = match direction {
        Direction::Up => -Point2D::y_basis(),
        Direction::Down => Point2D::y_basis(),
        Direction::Right => Point2D::x_basis(),
        Direction::Left => -Point2D::x_basis(),
    };
    point + offset
}

impl Direction {
    fn rotate(self, direction: i64) -> Self {
        use self::Direction::*;
        if direction == 0 {
            match self {
                Up => Left,
                Right => Up,
                Down => Right,
                Left => Down,
            }
        } else {
            match self {
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up,
            }
        }
    }
}
