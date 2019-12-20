use std::{iter, mem};

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as i32)
        .collect()
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &[i32]) -> String {
    let result = &to_input_format(&do_fft(input, 100))[..8];
    result.to_owned()
}

fn do_fft(input: &[i32], phases: usize) -> Vec<i32> {
    let mut current = input.to_owned();
    let mut next = current.clone();
    for _ in 0..phases {
        for i in 0..input.len() {
            let pattern = [0, 1, 0, -1]
                .iter()
                .flat_map(|el| iter::repeat(el).take(i + 1))
                .cycle()
                .skip(1);
            let element_sum: i32 = current.iter().zip(pattern).map(|(a, b)| a * b).sum();
            next[i] = element_sum.abs() % 10;
        }
        mem::swap(&mut current, &mut next);
    }
    current
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &[i32]) -> String {
    // Huge assumption here: the offset is far enough along that only the
    // "1" part of the pattern is relevant.
    // We can ignore everything before the offset, since those will always
    // be multiplied by "0" at the offset positions.
    let real_input: Vec<i32> = iter::repeat(input)
        .take(10_000)
        .flat_map(|i| i.iter().copied())
        .skip(to_input_format(&input[..7]).parse::<usize>().unwrap())
        .collect();

    let mut cur = real_input.clone();
    let mut next = real_input.clone();
    for _ in 0..100 {
        // Hacky solution: sum all the elements once, then subtract out elements one by one
        // to compute each successive new element.
        let mut sum: i32 = cur.iter().sum();
        for i in 0..cur.len() {
            next[i] = sum % 10;
            sum -= cur[i];
        }
        mem::swap(&mut cur, &mut next);
    }
    to_input_format(&cur)[..8].to_owned()
}

fn to_input_format(digits: &[i32]) -> String {
    digits
        .iter()
        .map(|&d| std::char::from_digit(d as u32, 10).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_do_fft_phase() {
        assert_eq!(
            "48226158",
            to_input_format(&do_fft(&input_generator("12345678"), 1))
        );
        assert_eq!(
            "34040438",
            to_input_format(&do_fft(&input_generator("48226158"), 1))
        );
        assert_eq!(
            "03415518",
            to_input_format(&do_fft(&input_generator("34040438"), 1))
        );
        assert_eq!(
            "01029498",
            to_input_format(&do_fft(&input_generator("03415518"), 1))
        );
    }

    #[test]
    pub fn test_part1() {
        assert_eq!(
            "24176176",
            solve_part1(&input_generator("80871224585914546619083218645595"))
        );
        assert_eq!(
            "73745418",
            solve_part1(&input_generator("19617804207202209144916044189917"))
        );
        assert_eq!(
            "52432133",
            solve_part1(&input_generator("69317163492948606335995924319873"))
        );
    }
}
