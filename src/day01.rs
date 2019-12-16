#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    input.iter().map(|m| (m / 3).saturating_sub(2)).sum()
}

fn fuel_for_mass(mass: u32) -> u32 {
    let mut fuel = 0;
    let mut cur_mass = mass;
    while cur_mass > 0 {
        let added_fuel = (cur_mass / 3).saturating_sub(2);
        fuel += added_fuel;
        cur_mass = added_fuel
    }
    fuel
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    input.iter().cloned().map(fuel_for_mass).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_fuel_for_mass() {
        assert_eq!(2, fuel_for_mass(14));
        assert_eq!(966, fuel_for_mass(1969));
        assert_eq!(50346, fuel_for_mass(100756));
    }
}