use std::collections::HashMap;
use std::num::ParseIntError;

use crate::geometry::Point2D;
use crate::intcode::{Intcode, IoBus};

#[aoc_generator(day13)]
pub fn generate_input(input: &str) -> Result<Intcode, ParseIntError> {
    input.parse()
}

#[aoc(day13, part1)]
pub fn solve_part1(program: &Intcode) -> usize {
    let mut output = IoBus::default();
    program.clone().execute_with_io(|| 0, &mut output);
    let mut output_vec = Vec::new();
    while let Some(v) = output.read() {
        output_vec.push(v);
    }

    let mut tiles = HashMap::new();
    for tile in output_vec.chunks_exact(3) {
        tiles.insert(point2D!(tile[0], tile[1]), tile[2]);
    }
    tiles.values().filter(|&&v| v == 2).count()
}

#[aoc(day13, part2)]
pub fn solve_part2(program: &Intcode) -> i64 {
    let mut game = program.clone();
    let mut input = IoBus::default();
    let mut output = IoBus::default();

    // Play for free!
    *game.mem_access(0) = 2;

    let mut tiles = HashMap::new();
    let mut score = 0;
    while !game.is_halted() {
        // Compute input
        input.write(compute_joystick(&tiles));

        // Execute game step until next input request
        game.execute_with_io(&mut input, &mut output);

        // Write all output to "screen"
        let mut output_vec = Vec::new();
        while let Some(v) = output.read() {
            output_vec.push(v);
        }
        for chunk in output_vec.chunks_exact(3) {
            if chunk[0] == -1 && chunk[1] == 0 {
                score = chunk[2];
            } else {
                tiles.insert(point2D!(chunk[0], chunk[1]), chunk[2]);
            }
        }
    }
    score
}

fn compute_joystick(tiles: &HashMap<Point2D<i64>, i64>) -> i64 {
    let ball = find_tile(tiles, 4);
    let paddle = find_tile(tiles, 3);
    if ball.x < paddle.x {
        -1
    } else if ball.x > paddle.x {
        1
    } else {
        0
    }
}

fn find_tile(tiles: &HashMap<Point2D<i64>, i64>, tile: i64) -> Point2D<i64> {
    tiles
        .iter()
        .find(|&e| *e.1 == tile)
        .map(|e| e.0)
        .copied()
        .unwrap_or_else(Point2D::origin)
}
