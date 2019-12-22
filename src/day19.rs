use std::collections::{hash_map::Entry, HashMap};
use std::num::ParseIntError;

use crate::geometry::Point2D;
use crate::intcode::{Intcode, IoBus};

#[aoc_generator(day19)]
pub fn generate_input(input: &str) -> Result<Intcode, ParseIntError> {
    input.parse()
}

#[aoc(day19, part1)]
pub fn solve_part1(program: &Intcode) -> usize {
    let mut beam_tracker = BeamTracker::with_program(program);
    let mut pulled_points = 0;
    for y in 0..50 {
        for x in 0..50 {
            if beam_tracker.is_pulled(x, y) {
                pulled_points += 1;
            }
        }
    }
    pulled_points
}

#[aoc(day19, part2)]
pub fn solve_part2(program: &Intcode) -> i64 {
    let mut beam_tracker = BeamTracker::with_program(program);
    let mut first_x = 0;
    for y in 0.. {
        // Find the last x that is pulled.
        let mut x = first_x;
        while !beam_tracker.is_pulled(x, y) {
            x += 1;
        }
        first_x = x;

        // Check if a spaceship would fit touching the bottom edge.
        if y >= 100
            && x >= 100
            && beam_tracker.is_pulled(x, y - 99)
            && beam_tracker.is_pulled(x + 99, y)
            && beam_tracker.is_pulled(x + 99, y - 99)
        {
            let mut y = y - 99;

            // Make sure we can't move the ship up any (maybe not necessary?)
            while y > 0 && beam_tracker.fits_spaceship_at(x, y - 1, 100) {
                y -= 1;
            }
            return 10_000 * x + y;
        }
    }
    0
}

struct BeamTracker {
    program: Intcode,
    point_statuses: HashMap<Point2D<i64>, i64>,
}

impl BeamTracker {
    pub fn with_program(program: &Intcode) -> Self {
        Self {
            program: program.clone(),
            point_statuses: HashMap::new(),
        }
    }

    pub fn is_pulled(&mut self, x: i64, y: i64) -> bool {
        match self.point_statuses.entry(point2D!(x, y)) {
            Entry::Occupied(o) => *o.get() == 1,
            Entry::Vacant(v) => {
                let mut drone = self.program.clone();
                let mut input = IoBus::default();
                let mut output = IoBus::default();
                input.write(x);
                input.write(y);
                drone.execute_with_io(&mut input, &mut output);
                let pulled = output.read().unwrap();
                v.insert(pulled);
                pulled == 1
            }
        }
    }

    pub fn fits_spaceship_at(&mut self, x: i64, y: i64, size: i64) -> bool {
        if !self.is_pulled(x + size - 1, y) {
            return false;
        }
        if !self.is_pulled(x, y + size - 1) {
            return false;
        }
        true
    }
}
