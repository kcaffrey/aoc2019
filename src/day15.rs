use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{self, Display, Formatter};
use std::num::ParseIntError;

use crate::geometry::Point2D;
use crate::intcode::{Intcode, IoBus};

#[aoc_generator(day15)]
pub fn generate_input(input: &str) -> Result<Intcode, ParseIntError> {
    input.parse()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Unknown,
    Wall,
    Open,
    Oxygen,
}

#[derive(Default)]
struct Map {
    tiles: HashMap<Point2D<i32>, Tile>,
}

#[aoc(day15, part1)]
pub fn solve_part1(program: &Intcode) -> usize {
    let map = create_complete_map(program);
    let (_, distance) = map
        .find_tile_path(Point2D::origin(), |_, t| t == Tile::Oxygen)
        .expect("there should be a path to oxygen in a complete map");
    distance
}

#[aoc(day15, part2)]
pub fn solve_part2(program: &Intcode) -> usize {
    let mut map = create_complete_map(program);
    let oxygen_source = *map
        .tiles
        .iter()
        .find(|&(_, t)| *t == Tile::Oxygen)
        .expect("there should be an oxygen tile")
        .0;
    let mut queue = VecDeque::new();
    let mut max_minutes = 0;
    queue.push_back((oxygen_source, 0));
    while let Some((cur, minutes)) = queue.pop_front() {
        max_minutes = max_minutes.max(minutes);
        for direction in 1..=4 {
            let next = do_movement(cur, direction);
            if map.tile_at(next) == Tile::Open {
                map.tiles.insert(next, Tile::Oxygen);
                queue.push_back((next, minutes + 1));
            }
        }
    }
    max_minutes
}

fn create_complete_map(program: &Intcode) -> Map {
    let mut robot = program.clone();
    let mut input = IoBus::default();
    let mut output = IoBus::default();
    let mut map = Map::new();
    let mut position = Point2D::origin();
    map.tiles.insert(position, Tile::Open);
    while let Some((direction, _)) = map.find_tile_path(position, |_, t| t == Tile::Unknown) {
        let target_position = do_movement(position, direction);

        input.write(direction);
        robot.execute_with_io(&mut input, &mut output);

        let status = output.read().unwrap();
        if status == 0 {
            map.tiles.insert(target_position, Tile::Wall);
        } else if status == 1 {
            position = target_position;
            map.tiles.insert(position, Tile::Open);
        } else if status == 2 {
            position = target_position;
            map.tiles.insert(position, Tile::Oxygen);
        } else {
            panic!("uhh crap status is {}", status)
        }
    }
    map
}

fn do_movement(point: Point2D<i32>, direction: i64) -> Point2D<i32> {
    match direction {
        1 => point - Point2D::y_basis(),
        2 => point + Point2D::y_basis(),
        3 => point - Point2D::x_basis(),
        4 => point + Point2D::x_basis(),
        _ => panic!("whats this direction garbage {}", direction),
    }
}

impl Map {
    fn new() -> Self {
        Map {
            ..Default::default()
        }
    }

    fn tile_at(&self, point: Point2D<i32>) -> Tile {
        *self.tiles.get(&point).unwrap_or(&Tile::Unknown)
    }

    // Finds a path to a particular tile (defined by a predicate), returning
    // both the first step on the path and the path length
    fn find_tile_path<F>(&self, source: Point2D<i32>, f: F) -> Option<(i64, usize)>
    where
        F: Fn(Point2D<i32>, Tile) -> bool,
    {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        seen.insert(source);
        for direction in 1..=4 {
            let point = do_movement(source, direction);
            let tile = self.tile_at(point);
            if tile != Tile::Wall {
                queue.push_back((point, direction, 1));
                seen.insert(point);
            }
        }
        while let Some((cur_point, first_direction, distance)) = queue.pop_front() {
            if f(cur_point, self.tile_at(cur_point)) {
                return Some((first_direction, distance));
            }
            for direction in 1..=4 {
                let point = do_movement(cur_point, direction);
                let tile = self.tile_at(point);
                if tile != Tile::Wall && seen.insert(point) {
                    queue.push_back((point, first_direction, distance + 1));
                }
            }
        }
        None
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let minx = self.tiles.keys().map(|p| p.x).min().unwrap();
        let maxx = self.tiles.keys().map(|p| p.x).max().unwrap();
        let miny = self.tiles.keys().map(|p| p.y).min().unwrap();
        let maxy = self.tiles.keys().map(|p| p.y).max().unwrap();
        for y in miny..=maxy {
            writeln!(f)?;
            for x in minx..=maxx {
                let ch = if x == 0 && y == 0 {
                    'O'
                } else {
                    match self.tile_at(point2D!(x, y)) {
                        Tile::Unknown => ' ',
                        Tile::Open => '.',
                        Tile::Wall => '#',
                        Tile::Oxygen => 'D',
                    }
                };
                write!(f, "{}", ch)?;
            }
        }
        writeln!(f)
    }
}
