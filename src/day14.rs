use std::collections::{hash_map::Entry, HashMap, VecDeque};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Reaction {
    inputs: Vec<(String, u64)>,
    output: (String, u64),
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Reaction> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(" => ").collect::<Vec<_>>();
            Reaction {
                inputs: parts[0].split(", ").map(parse_chem_amount).collect(),
                output: parse_chem_amount(parts[1]),
            }
        })
        .collect()
}

fn parse_chem_amount(input: &str) -> (String, u64) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+) ([A-Z]+)").unwrap();
    }
    let caps = RE.captures(input).unwrap();
    (caps[2].to_owned(), caps[1].parse().unwrap())
}

#[aoc(day14, part1)]
pub fn solve_part1(reactions: &[Reaction]) -> u64 {
    let mut factory = Factory::from_reactions(reactions);
    factory.produce_fuel(1)
}

#[aoc(day14, part2)]
pub fn solve_part2(reactions: &[Reaction]) -> u64 {
    let mut factory = Factory::from_reactions(reactions);
    let mut remaining_ore = 1_000_000_000_000;
    let max_needed_ore = factory.produce_fuel(1);
    let mut produced_fuel = 1;
    remaining_ore -= max_needed_ore;

    while remaining_ore > 0 {
        let bulk_count = (remaining_ore / max_needed_ore).max(1);
        let needed_ore = factory.produce_fuel(bulk_count);
        if needed_ore <= remaining_ore {
            produced_fuel += bulk_count;
        }
        remaining_ore = remaining_ore.saturating_sub(needed_ore);
    }
    produced_fuel
}

struct Factory {
    reaction_map: HashMap<String, Reaction>,
    distance_to_ore: HashMap<String, u64>,
    leftovers: HashMap<String, u64>,
}

impl Factory {
    fn from_reactions(reactions: &[Reaction]) -> Self {
        Factory {
            reaction_map: reactions
                .iter()
                .map(|r| (r.output.0.clone(), r.clone()))
                .collect(),
            distance_to_ore: compute_distance_to_ore(reactions),
            leftovers: HashMap::new(),
        }
    }

    // Returns the amount of ore needed to produce 1 fuel, keeping track of leftovers
    fn produce_fuel(&mut self, fuel_count: u64) -> u64 {
        let mut cur_chemicals = HashMap::new();
        cur_chemicals.insert("FUEL".to_owned(), fuel_count);
        while !(cur_chemicals.len() == 1 && cur_chemicals.contains_key("ORE")) {
            let cur = cur_chemicals
                .keys()
                .max_by_key(|&chem| self.distance_to_ore[chem])
                .unwrap()
                .clone();

            let mut desired = cur_chemicals.remove(&cur).unwrap();
            let from_leftover = desired.min(*self.leftovers.get(&cur).unwrap_or(&0));
            desired -= from_leftover;
            *self.leftovers.entry(cur.to_owned()).or_default() -= from_leftover;

            let reaction = &self.reaction_map[&cur];
            let amount = desired / reaction.output.1 + (desired % reaction.output.1).min(1);
            let leftover = amount * reaction.output.1 - desired;
            *self.leftovers.entry(cur.to_owned()).or_default() += leftover;
            for input in &reaction.inputs {
                *cur_chemicals.entry(input.0.clone()).or_default() += amount * input.1;
            }
        }
        self.leftovers.retain(|_, count| *count > 0);
        cur_chemicals["ORE"]
    }
}

fn compute_distance_to_ore(reactions: &[Reaction]) -> HashMap<String, u64> {
    let mut adj_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for reaction in reactions {
        for input in &reaction.inputs {
            adj_map
                .entry(input.0.as_ref())
                .or_default()
                .push(reaction.output.0.as_ref());
        }
    }

    let mut distances = HashMap::new();
    let mut open = VecDeque::new();
    distances.insert("ORE".to_owned(), 0);
    open.push_front("ORE");
    while let Some(cur) = open.pop_back() {
        if adj_map.contains_key(cur) {
            for &adj in &adj_map[cur] {
                let mut updated = false;
                let new_distance = distances[cur] + 1;
                match distances.entry(adj.to_owned()) {
                    Entry::Vacant(e) => {
                        updated = true;
                        e.insert(new_distance);
                    }
                    Entry::Occupied(mut o) => {
                        if new_distance > *o.get() {
                            updated = true;
                            o.insert(new_distance);
                        }
                    }
                }
                if updated {
                    open.push_back(adj);
                }
            }
        }
    }
    distances
}
