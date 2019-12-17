use std::num::ParseIntError;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Result<(u32, u32), ParseIntError> {
    let parts = input
        .split('-')
        .map(str::parse)
        .collect::<Result<Vec<u32>, ParseIntError>>()?;
    Ok((parts[0], parts[1]))
}

#[aoc(day4, part1)]
pub fn solve_part1(&(low, high): &(u32, u32)) -> usize {
    (low..=high)
        .map(|num| {
            num.to_string()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|candidate| matches_rules(candidate))
        .count()
}

fn matches_rules(digits: &[u32]) -> bool {
    let has_double = digits
        .iter()
        .zip(digits.iter().skip(1))
        .any(|(a, b)| a == b);
    let is_decreasing = digits
        .iter()
        .zip(digits.iter().skip(1))
        .all(|(a, b)| a <= b);
    has_double && is_decreasing
}

#[aoc(day4, part2)]
pub fn solve_part2(&(low, high): &(u32, u32)) -> usize {
    (low..=high)
        .map(|num| {
            num.to_string()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|candidate| matches_rules2(candidate))
        .count()
}

fn matches_rules2(digits: &[u32]) -> bool {
    let mut last = 0;
    let mut group_size = 1;
    let mut has_group_of_two = false;
    for &digit in digits {
        if digit == last {
            group_size += 1;
        } else {
            has_group_of_two = has_group_of_two || group_size == 2;
            group_size = 1;
        }
        last = digit;
    }
    has_group_of_two = has_group_of_two || group_size == 2;
    let is_decreasing = digits
        .iter()
        .zip(digits.iter().skip(1))
        .all(|(a, b)| a <= b);
    has_group_of_two && is_decreasing
}
