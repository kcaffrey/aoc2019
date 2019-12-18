use std::collections::{hash_map::Entry, HashMap, HashSet};

use num::Integer;

use crate::geometry::Point2D;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Point2D<i32>> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut asteroids = vec![];
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if map[y][x] == '#' {
                asteroids.push(point2D!(x as i32, y as i32));
            }
        }
    }
    asteroids
}

#[aoc(day10, part1)]
pub fn solve_part1(asteroids: &[Point2D<i32>]) -> usize {
    asteroids
        .iter()
        .map(|&asteroid| visible_asteroids(asteroids, asteroid).len())
        .max()
        .unwrap()
}

#[aoc(day10, part2)]
pub fn solve_part2(asteroids: &[Point2D<i32>]) -> i32 {
    let station = asteroids
        .iter()
        .copied()
        .max_by_key(|&asteroid| visible_asteroids(asteroids, asteroid).len())
        .unwrap();
    let mut zapped = HashSet::new();
    zapped.insert(station);
    let mut zapcount = 0;
    while zapcount < 200 {
        let remaining: Vec<_> = asteroids
            .iter()
            .copied()
            .filter(|a| !zapped.contains(a))
            .collect();
        let current_visible = visible_asteroids(&remaining, station);
        zapped.extend(&current_visible);
        if zapcount + current_visible.len() >= 200 {
            let index = 200 - zapcount - 1;
            return current_visible[index].x * 100 + current_visible[index].y;
        }
        zapcount += current_visible.len();
    }
    0
}

fn visible_asteroids(asteroids: &[Point2D<i32>], asteroid: Point2D<i32>) -> Vec<Point2D<i32>> {
    let mut visibility_map = HashMap::new();
    for &a in asteroids {
        if a != asteroid {
            let (angle, distance) = to_polar(asteroid, a);
            match visibility_map.entry(angle) {
                Entry::Vacant(v) => {
                    v.insert((distance, a));
                }
                Entry::Occupied(mut o) => {
                    if o.get().0 > distance {
                        o.insert((distance, a));
                    }
                }
            }
        }
    }
    let mut visible: Vec<_> = visibility_map.into_iter().collect();
    visible.sort_by(|a, b| to_degrees(a.0).partial_cmp(&to_degrees(b.0)).unwrap());
    visible.into_iter().map(|e| (e.1).1).collect()
}

fn to_polar(a: Point2D<i32>, b: Point2D<i32>) -> ((i32, i32), i32) {
    let mut x_distance = b.x - a.x;
    let mut y_distance = b.y - a.y;
    if y_distance == 0 {
        x_distance = x_distance.signum();
    }
    if x_distance == 0 {
        y_distance = y_distance.signum();
    }
    if x_distance != 0 && y_distance != 0 {
        let g = x_distance.gcd(&y_distance);
        x_distance /= g;
        y_distance /= g;
    }
    (
        (x_distance, y_distance),
        x_distance.abs() + y_distance.abs(),
    )
}

fn to_degrees((x_distance, y_distance): (i32, i32)) -> f32 {
    let (x, y) = (x_distance as f32, y_distance as f32);
    let mut degrees = (y / (x * x + y * y).sqrt()).asin().to_degrees();
    if x >= 0.0 {
        degrees += 90.0;
    } else {
        degrees = 270.0 - degrees;
    }
    degrees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_visible_asteroids() {
        let asteroids = vec![
            point2D!(1, 0),
            point2D!(4, 0),
            point2D!(0, 2),
            point2D!(1, 2),
            point2D!(2, 2),
            point2D!(3, 2),
            point2D!(4, 2),
            point2D!(4, 3),
            point2D!(3, 4),
            point2D!(4, 4),
        ];
        let distances: Vec<_> = asteroids
            .iter()
            .copied()
            .map(|a| visible_asteroids(&asteroids, a).len())
            .collect();
        assert_eq!(vec![7, 7, 6, 7, 7, 7, 5, 7, 8, 7], distances);
    }

    #[test]
    pub fn test_part1() {
        let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        assert_eq!(33, solve_part1(&input_generator(&input)));

        let input = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
        assert_eq!(35, solve_part1(&input_generator(&input)));

        let input = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
        assert_eq!(41, solve_part1(&input_generator(&input)));

        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        assert_eq!(210, solve_part1(&input_generator(&input)));
    }
}
