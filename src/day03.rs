use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

use crate::geometry::Point2D;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PathSegment {
    direction: Direction,
    distance: u32,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Path(Vec<PathSegment>);

impl FromStr for PathSegment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PathSegment {
            direction: match &s[0..1] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Unknown direction"),
            },
            distance: s[1..].parse()?,
        })
    }
}

impl FromStr for Path {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Path(
            s.split(',')
                .map(str::parse)
                .collect::<Result<Vec<PathSegment>, ParseIntError>>()?,
        ))
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Result<Vec<Path>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: (&[Path])) -> i32 {
    let mut points: HashSet<Point2D<i32>> = input[0].to_point_map().keys().copied().collect();
    for path in &input[1..] {
        points = &points & &path.to_point_map().keys().copied().collect();
    }
    points
        .into_iter()
        .map(|p| p.manhattan_distance(Point2D::origin()))
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Path]) -> u32 {
    let points1 = input[0].to_point_map();
    let points2 = input[1].to_point_map();
    let mut best = u32::max_value();
    for (point, d1) in points1 {
        if let Some(d2) = points2.get(&point) {
            if d1 + d2 < best {
                best = d1 + d2;
            }
        }
    }
    best
}

impl Path {
    fn to_point_map(&self) -> HashMap<Point2D<i32>, u32> {
        let mut result = HashMap::new();
        let mut cur_point = Point2D::origin();
        let mut count = 0;
        for segment in &self.0 {
            let dir = match segment.direction {
                Direction::Up => Point2D::y_basis(),
                Direction::Down => -Point2D::y_basis(),
                Direction::Left => -Point2D::x_basis(),
                Direction::Right => Point2D::x_basis(),
            };
            for _ in 0..segment.distance {
                count += 1;
                cur_point += dir;
                result.entry(cur_point).or_insert(count);
            }
        }
        result
    }
}
