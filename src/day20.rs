use std::collections::{HashMap, HashSet, VecDeque};

use crate::geometry::{Point2D, Point3D};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Open,
    Wall,
}

#[derive(Clone, Default)]
pub struct Map {
    tiles: HashMap<Point2D<i32>, Tile>,
    portals: HashMap<Point2D<i32>, Point2D<i32>>,
    outer_portals: HashSet<Point2D<i32>>,
    start: Point2D<i32>,
    goal: Point2D<i32>,
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Map {
    let mut map = Map::default();
    let g = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let (w, h) = (g[0].len(), g.len());
    let mut labels: HashMap<String, Vec<Point2D<i32>>> = HashMap::new();
    let mut outer_portals = HashSet::new();

    for x in 0..w {
        for y in 0..h {
            match g[y][x] {
                '#' => {
                    map.tiles.insert(point2D!(x as i32, y as i32), Tile::Wall);
                }
                '.' => {
                    map.tiles.insert(point2D!(x as i32, y as i32), Tile::Open);
                }
                'A'..='Z' => {
                    if x + 2 < w && g[y][x + 1].is_alphabetic() && g[y][x + 2] == '.' {
                        labels
                            .entry(label(g[y][x], g[y][x + 1]))
                            .or_default()
                            .push(point2D!(x as i32 + 2, y as i32));
                        if x == 0 {
                            outer_portals.insert(point2D!(x as i32 + 2, y as i32));
                        }
                    }
                    if x + 1 < w && x > 0 && g[y][x - 1] == '.' && g[y][x + 1].is_alphabetic() {
                        labels
                            .entry(label(g[y][x], g[y][x + 1]))
                            .or_default()
                            .push(point2D!(x as i32 - 1, y as i32));
                        if x + 2 == w {
                            outer_portals.insert(point2D!(x as i32 - 1, y as i32));
                        }
                    }
                    if y + 2 < h && g[y + 1][x].is_alphabetic() && g[y + 2][x] == '.' {
                        labels
                            .entry(label(g[y][x], g[y + 1][x]))
                            .or_default()
                            .push(point2D!(x as i32, y as i32 + 2));
                        if y == 0 {
                            outer_portals.insert(point2D!(x as i32, y as i32 + 2));
                        }
                    }
                    if y + 1 < h && y > 0 && g[y - 1][x] == '.' && g[y + 1][x].is_alphabetic() {
                        labels
                            .entry(label(g[y][x], g[y + 1][x]))
                            .or_default()
                            .push(point2D!(x as i32, y as i32 - 1));
                        if y + 2 == h {
                            outer_portals.insert(point2D!(x as i32, y as i32 - 1));
                        }
                    }
                }
                _ => {}
            };
        }
    }
    map.start = labels.remove("AA").unwrap()[0];
    map.goal = labels.remove("ZZ").unwrap()[0];
    map.outer_portals = outer_portals;
    map.outer_portals.remove(&map.start);
    map.outer_portals.remove(&map.goal);
    for (label, points) in labels {
        map.portals.insert(points[0], points[1]);
        map.portals.insert(points[1], points[0]);
    }
    map
}

fn label(ch1: char, ch2: char) -> String {
    let mut ret = String::new();
    ret.push(ch1);
    ret.push(ch2);
    ret
}

#[aoc(day20, part1)]
pub fn solve_part1(map: &Map) -> u32 {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    visited.insert(map.start);
    queue.push_back((map.start, 0));
    while let Some((cur, distance)) = queue.pop_front() {
        if cur == map.goal {
            return distance;
        }
        for neighbor in map.neighbors(cur) {
            if visited.insert(neighbor) {
                queue.push_back((neighbor, distance + 1));
            }
        }
    }
    0
}

#[aoc(day20, part2)]
pub fn solve_part2(map: &Map) -> u32 {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let start = point3D!(map.start.x, map.start.y, 0);
    visited.insert(start);
    queue.push_back((start, 0));
    while let Some((cur, distance)) = queue.pop_front() {
        if cur == point3D!(map.goal.x, map.goal.y, 0) {
            return distance;
        }
        for neighbor in map.recursive_neighbors(cur) {
            if visited.insert(neighbor) {
                queue.push_back((neighbor, distance + 1));
            }
        }
    }
    0
}

impl Map {
    pub fn neighbors(&self, point: Point2D<i32>) -> Vec<Point2D<i32>> {
        let mut neighbors = vec![];
        let offsets = [
            Point2D::x_basis(),
            -Point2D::x_basis(),
            Point2D::y_basis(),
            -Point2D::y_basis(),
        ];
        for &offset in &offsets {
            let neighbor = point + offset;
            if self.tiles.get(&neighbor).unwrap_or(&Tile::Wall) == &Tile::Open {
                neighbors.push(neighbor);
            }
        }
        if let Some(&neighbor) = self.portals.get(&point) {
            neighbors.push(neighbor);
        }
        neighbors
    }

    pub fn recursive_neighbors(&self, point: Point3D<i32>) -> Vec<Point3D<i32>> {
        let mut neighbors = vec![];
        let offsets = [
            Point3D::x_basis(),
            -Point3D::x_basis(),
            Point3D::y_basis(),
            -Point3D::y_basis(),
        ];
        for &offset in &offsets {
            let neighbor = point + offset;
            if self
                .tiles
                .get(&point2D!(neighbor.x, neighbor.y))
                .unwrap_or(&Tile::Wall)
                == &Tile::Open
            {
                neighbors.push(neighbor);
            }
        }
        if let Some(&neighbor) = self.portals.get(&point2D!(point.x, point.y)) {
            if !self.outer_portals.contains(&point2D!(point.x, point.y)) {
                neighbors.push(point3D!(neighbor.x, neighbor.y, point.z + 1));
            } else if point.z > 0 {
                neighbors.push(point3D!(neighbor.x, neighbor.y, point.z - 1))
            }
        }
        neighbors
    }
}
