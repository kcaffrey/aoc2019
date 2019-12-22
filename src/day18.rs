use std::collections::{HashMap, HashSet, VecDeque};

use lazy_static::lazy_static;

use crate::geometry::Point2D;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Open,
    Key(char),
    Door(char),
}

#[derive(Debug, Clone, Default)]
pub struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    entrance: Point2D<i32>,
    doors: HashMap<char, Point2D<i32>>,
    closed_doors: HashSet<Point2D<i32>>,
    reachable: HashSet<Point2D<i32>>,
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Map {
    let mut map = Map::default();
    let mut x = 0;
    let mut y = 0;
    for ch in input.chars() {
        let cur_position = point2D!(x, y);
        x += 1;
        match ch {
            '\n' => {
                if y == 0 {
                    map.width = map.tiles.len();
                }
                x = 0;
                y += 1;
            }
            '#' => map.tiles.push(Tile::Wall),
            '.' => map.tiles.push(Tile::Open),
            '@' => {
                map.tiles.push(Tile::Open);
                map.entrance = cur_position;
            }
            'a'..='z' => map.tiles.push(Tile::Key(ch)),
            'A'..='Z' => {
                map.tiles.push(Tile::Door(ch.to_ascii_lowercase()));
                map.doors.insert(ch.to_ascii_lowercase(), cur_position);
            }
            _ => panic!("unexpected character: {}", ch),
        }
    }
    map.height = y as usize + 1;
    map.closed_doors.extend(map.doors.values());
    map.fill_reachable_from(map.entrance);
    map
}

#[aoc(day18, part1)]
pub fn solve_part1(map: &Map) -> u32 {
    // todo: too slow
    // shortest_path(map.clone(), map.entrance, 0)
    0
}

fn shortest_path(mut map: Map, position: Point2D<i32>, distance_so_far: u32) -> u32 {
    let key_paths = map.find_shortest_path_to_keys(position);
    if key_paths.is_empty() {
        if map.doors.is_empty() {
            return distance_so_far;
        } else {
            // Just in case we somehow manage to lock ourselves out of all doors (should be impossible?)
            return u32::max_value();
        }
    }

    let mut min = u32::max_value();
    for (key, (distance, destination)) in key_paths {
        let mut new_map = map.clone();
        new_map.closed_doors.remove(&new_map.doors[&key]);
        *new_map.get_tile_mut(new_map.doors[&key]).unwrap() = Tile::Open;
        new_map.doors.remove(&key);
        *new_map.get_tile_mut(destination).unwrap() = Tile::Open;

        let candidate_distance = shortest_path(new_map, destination, distance_so_far + distance);
        if candidate_distance < min {
            min = candidate_distance;
        }
    }
    min
}

impl Map {
    pub fn fill_reachable_from(&mut self, start: Point2D<i32>) {
        self.find_shortest_path_to_keys(start);
    }

    pub fn find_shortest_path_to_keys(
        &mut self,
        from: Point2D<i32>,
    ) -> HashMap<char, (u32, Point2D<i32>)> {
        let mut visited = HashSet::new();
        visited.insert(from);
        let mut queue = VecDeque::new();
        queue.push_back((from, 0));
        let mut key_distances = HashMap::new();
        while let Some((cur, distance_so_far)) = queue.pop_front() {
            if let Some(Tile::Key(k)) = self.get_tile(cur) {
                key_distances.insert(k, (distance_so_far, cur));
            }
            for neighbor in self.neighbors(cur) {
                if self.get_tile(neighbor) != Some(Tile::Wall)
                    && visited.insert(neighbor)
                    && !self.closed_doors.contains(&neighbor)
                {
                    queue.push_back((neighbor, distance_so_far + 1));
                }
            }
        }
        self.reachable.extend(visited);
        key_distances
    }

    pub fn get_tile(&self, coordinate: Point2D<i32>) -> Option<Tile> {
        self.tiles.get(self.to_offset(coordinate)?).map(|c| *c)
    }

    pub fn get_tile_mut(&mut self, coordinate: Point2D<i32>) -> Option<&mut Tile> {
        let offset = self.to_offset(coordinate)?;
        self.tiles.get_mut(offset)
    }

    fn to_offset(&self, coordinate: Point2D<i32>) -> Option<usize> {
        if coordinate.y as usize >= self.height || coordinate.x as usize >= self.width {
            return None;
        }
        let offset = coordinate.y as usize * self.width + coordinate.x as usize;
        if offset >= self.tiles.len() {
            return None;
        }
        Some(offset)
    }

    fn neighbors(&self, coordinate: Point2D<i32>) -> Vec<Point2D<i32>> {
        lazy_static! {
            static ref OFFSETS: [Point2D<i32>; 4] = [
                Point2D::x_basis(),
                -Point2D::x_basis(),
                Point2D::y_basis(),
                -Point2D::y_basis(),
            ];
        }
        OFFSETS
            .iter()
            .copied()
            .map(move |offset| coordinate + offset)
            .filter(move |&c| self.to_offset(c).is_some())
            .collect()
    }
}
