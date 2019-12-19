use lazy_static::lazy_static;
use num::Integer;
use regex::Regex;

use crate::geometry::Point3D;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Moon {
    position: Point3D<i32>,
    velocity: Point3D<i32>,
}

impl Moon {
    pub fn potential_energy(self) -> i32 {
        self.position.manhattan_distance(Point3D::origin())
    }

    pub fn kinetic_energy(self) -> i32 {
        self.velocity.manhattan_distance(Point3D::origin())
    }

    pub fn total_energy(self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Moon> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    }
    RE.captures_iter(input)
        .map(|cap| {
            point3D!(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap()
            )
        })
        .map(|p| Moon {
            position: p,
            velocity: Point3D::origin(),
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(moons: &[Moon]) -> i32 {
    let mut moons = moons.to_owned();
    for _ in 0..1000 {
        do_step(&mut moons);
    }
    moons.into_iter().map(Moon::total_energy).sum()
}

#[aoc(day12, part2)]
pub fn solve_part2(moons: &[Moon]) -> usize {
    let periodx = find_period(&moons.iter().map(|m| m.position.x).collect::<Vec<_>>());
    let periody = find_period(&moons.iter().map(|m| m.position.y).collect::<Vec<_>>());
    let periodz = find_period(&moons.iter().map(|m| m.position.z).collect::<Vec<_>>());

    periodx.lcm(&periody.lcm(&periodz))
}

fn do_step(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in i + 1..moons.len() {
            let (a, b) = (moons[i], moons[j]);
            let (x, y, z) = (
                b.position.x - a.position.x,
                b.position.y - a.position.y,
                b.position.z - a.position.z,
            );
            let deltav = point3D!(x.signum(), y.signum(), z.signum());
            moons.get_mut(i).unwrap().velocity += deltav;
            moons.get_mut(j).unwrap().velocity += -deltav;
        }
    }
    for moon in moons {
        moon.position += moon.velocity;
    }
}

fn find_period(values: &[i32]) -> usize {
    let mut positions = values.to_owned();
    let mut velocities = vec![0; positions.len()];
    let mut count = 0;
    loop {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let deltav = (positions[j] - positions[i]).signum();
                velocities[i] += deltav;
                velocities[j] -= deltav;
            }
        }
        for i in 0..positions.len() {
            positions[i] += velocities[i];
        }
        count += 1;
        if &positions[..] == values && velocities == vec![0; positions.len()] {
            return count;
        }
    }
}
