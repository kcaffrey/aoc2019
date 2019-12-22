use std::mem;

use lazy_static::lazy_static;
use mod_exp::mod_exp;
use regex::Regex;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ShuffleTechnique {
    DealIntoNewStack,
    Cut(i128),
    DealWithIncrement(usize),
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<ShuffleTechnique> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(deal into new stack)|(?:cut (-?\d+))|(?:deal with increment (-?\d+))")
                .unwrap();
    }
    RE.captures_iter(input)
        .map(|cap| {
            if let Some(i) = cap.get(2) {
                ShuffleTechnique::Cut(i.as_str().parse().unwrap())
            } else if let Some(i) = cap.get(3) {
                ShuffleTechnique::DealWithIncrement(i.as_str().parse().unwrap())
            } else {
                ShuffleTechnique::DealIntoNewStack
            }
        })
        .collect()
}

#[aoc(day22, part1)]
pub fn solve_part1(techniques: &[ShuffleTechnique]) -> usize {
    shuffle(10007, techniques)
        .into_iter()
        .position(|c| c == 2019)
        .unwrap()
}

#[aoc(day22, part2)]
pub fn solve_part2(techniques: &[ShuffleTechnique]) -> i128 {
    let desired_index = 2020;
    let deck_len: i128 = 119_315_717_514_047;
    let desired_shuffles: i128 = 101_741_582_076_661;

    // Turn the shuffle techniques into modular arithmetic in the form
    // of ax + b, where x is the index of a card, and the result is index
    // of the same card prior to the shuffle
    // Apply the techniques in reverse to get the original card value.
    // Start with a = 1 and b = 0, which is the identity.
    let (mut a, mut b) = (1i128, 0i128);
    for t in techniques.iter().rev() {
        match t {
            ShuffleTechnique::DealIntoNewStack => {
                a = -a;
                b = -b - 1;
            }
            ShuffleTechnique::Cut(i) => {
                b += i;
            }
            ShuffleTechnique::DealWithIncrement(i) => {
                let n = mod_exp(*i as i128, deck_len - 2, deck_len);
                a *= n;
                b *= n;
            }
        }
        a = (a + deck_len as i128) % deck_len as i128;
        b = (b + deck_len as i128) % deck_len as i128;
    }

    // Apply the shuffle n times, which results in the following modular equation:
    // a * (a * (ax + b) + b) + b ....
    // a^n * x + (b + a^1 * b + a^2 * b + .... + a ^ (n - 1) * b)
    // a^n * x + b * (a^0 + a^1 + .... a^(n - 1))  <---- geometric series, had to look up the closed form for modular arithmetic
    // a^n * x + b * (a^n - 1) / (a - 1)
    // a^n * x + b * (a^n - 1) * (a - 1)^-1
    let a_term = desired_index * mod_exp(a, desired_shuffles, deck_len) % deck_len;
    let b_mult = (mod_exp(a, desired_shuffles, deck_len) - 1)
        * mod_exp(a - 1, deck_len - 2, deck_len) // note deck_len-2 = -1 mod deck_len, 
        % deck_len;
    let b_term = b * b_mult % deck_len;
    (a_term + b_term) % deck_len
}

fn shuffle(num_cards: usize, techniques: &[ShuffleTechnique]) -> Vec<usize> {
    let mut cur_deck = (0..num_cards).collect::<Vec<usize>>();
    let mut new_deck = cur_deck.clone();
    for t in techniques {
        match t {
            ShuffleTechnique::DealIntoNewStack => {
                new_deck.clear();
                new_deck.extend(cur_deck.iter().rev())
            }
            ShuffleTechnique::Cut(i) => {
                new_deck.clear();
                let index = (i + num_cards as i128) as usize % cur_deck.len();
                new_deck.extend(&cur_deck[index..]);
                new_deck.extend(&cur_deck[..index]);
            }
            ShuffleTechnique::DealWithIncrement(i) => {
                let mut cur_index = 0;
                for &val in &cur_deck {
                    new_deck[cur_index] = val;
                    cur_index = (cur_index + i) % cur_deck.len();
                }
            }
        }
        mem::swap(&mut cur_deck, &mut new_deck);
    }
    cur_deck
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_shuffle() {
        use ShuffleTechnique::*;
        assert_eq!(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            shuffle(10, &[DealIntoNewStack])
        );
        assert_eq!(vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2], shuffle(10, &[Cut(3)]));
        assert_eq!(vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5], shuffle(10, &[Cut(-4)]));
        assert_eq!(
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
            shuffle(10, &[DealWithIncrement(3)])
        );
    }
}
