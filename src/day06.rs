use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> HashMap<String, String> {
    input
        .lines()
        .map(|line| line.split(')').collect::<Vec<_>>())
        .map(|orbit| (orbit[1].to_owned(), orbit[0].to_owned()))
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(map: &HashMap<String, String>) -> u32 {
    map.keys().map(|body| distance_to_com(map, body)).sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(map: &HashMap<String, String>) -> u32 {
    let mut you_orbited = &map["YOU"];
    let mut san_orbited = &map["SAN"];
    let mut you_distance = distance_to_com(map, you_orbited);
    let mut san_distance = distance_to_com(map, san_orbited);
    let mut transfers = 0;
    while you_orbited != san_orbited {
        transfers += 1;
        if you_distance >= san_distance {
            you_orbited = &map[you_orbited];
            you_distance -= 1;
        } else {
            san_orbited = &map[san_orbited];
            san_distance -= 1;
        }
    }
    transfers
}

fn distance_to_com(map: &HashMap<String, String>, body: &str) -> u32 {
    let mut distance_to_com = 0;
    let mut cur_body = body;
    while let Some(orbited) = map.get(cur_body) {
        distance_to_com += 1;
        cur_body = orbited;
    }
    distance_to_com
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT1: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

    const SAMPLE_INPUT2: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

    #[test]
    pub fn test_part1() {
        let map = input_generator(SAMPLE_INPUT1);
        assert_eq!(42, solve_part1(&map));
    }

    #[test]
    pub fn test_part2() {
        let map = input_generator(SAMPLE_INPUT2);
        assert_eq!(4, solve_part2(&map));
    }
}
