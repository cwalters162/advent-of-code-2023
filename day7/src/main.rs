use crate::HandType::{
    FIVEKIND, FOURKIND, FULLHOUSE, HIGHCARD, ONEPAIR, THREEKIND, TWOPAIR, UNKNOWN,
};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let part1result = part1(&contents);
    // let part2result = part2(&contents);

    println!("Part 1: {}", part1result);
    // println!("Part 2: {}", part2result);
}

fn part1(contents: &String) -> i32 {
    let lines = contents.split("\n").collect::<Vec<_>>();
    let no_whitespace = lines
        .iter()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let mut hands = no_whitespace
        .iter()
        .map(|line| Hand {
            hand_type: UNKNOWN,
            cards: line[0 as usize].parse().unwrap(),
            bet: line[1 as usize].parse::<i32>().unwrap(),
        })
        .collect::<Vec<Hand>>();

    for mut hand in hands.iter_mut() {
        hand.discover_hand_type();
    }

    hands.sort_by(|left, right| left.hand_type.partial_cmp(&right.hand_type).unwrap());
    hands.sort_by(|left, right| {
        if left.hand_type != right.hand_type {
            return Ordering::Equal;
        }
        let left_cards = left.cards.chars().collect::<Vec<char>>();
        let right_cards = right.cards.chars().collect::<Vec<char>>();
        for i in 0..left.cards.len() {
            let left_card = get_rank_from_char(left_cards[i]);
            let right_card = get_rank_from_char(right_cards[i]);
            let result = left_card.partial_cmp(&right_card).unwrap();
            if result == Ordering::Equal {
                continue;
            } else {
                return result;
            }
        }
        Ordering::Equal
    });
    hands.reverse();

    dbg![&hands];

    let bets = hands.iter().map(|hand| hand.bet);
    let mut bet_result = 0;
    for i in 0..bets.len() {
        let hand_bet = hands.get(i).unwrap().bet;
        let multiplier = 1 + i as i32;
        bet_result += hand_bet * multiplier;
    }
    bet_result
}

fn part2(contents: &String) -> i32 {
    todo!()
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    cards: String,
    bet: i32,
}

impl Hand {
    fn discover_hand_type(&mut self) {
        let mut found_cards = HashMap::new();
        for card in self.cards.chars() {
            if !found_cards.contains_key(&card) {
                found_cards.insert(card, 1);
            } else {
                *found_cards.get_mut(&card).expect("Not in found cards") += 1;
            }
        }
        if found_cards.len() == 1 {
            self.hand_type = FIVEKIND;
            return;
        }

        let mut pairs = 0;
        let mut three_kind = false;

        for card_count in found_cards.values() {
            match card_count {
                2 => {
                    pairs += 1;
                }
                3 => {
                    three_kind = true;
                }
                4 => {
                    self.hand_type = FOURKIND;
                    return;
                }
                _ => {}
            }
        }

        match (pairs, three_kind) {
            (1, true) => {
                self.hand_type = FULLHOUSE;
            }
            (0, true) => {
                self.hand_type = THREEKIND;
            }
            (2, false) => {
                self.hand_type = TWOPAIR;
            }
            (1, false) => {
                self.hand_type = ONEPAIR;
            }
            (_, _) => {
                self.hand_type = HIGHCARD;
            }
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    FIVEKIND,
    FOURKIND,
    FULLHOUSE,
    THREEKIND,
    TWOPAIR,
    ONEPAIR,
    HIGHCARD,
    UNKNOWN,
}

#[derive(Debug, PartialOrd, PartialEq)]
enum Rank {
    ACE,
    KING,
    QUEEN,
    JACK,
    TEN,
    NINE,
    EIGHT,
    SEVEN,
    SIX,
    FIVE,
    FOUR,
    THREE,
    TWO,
}

fn get_rank_from_char(char: char) -> Rank {
    match char {
        'A' => Rank::ACE,
        'K' => Rank::KING,
        'Q' => Rank::QUEEN,
        'J' => Rank::JACK,
        'T' => Rank::TEN,
        '9' => Rank::NINE,
        '8' => Rank::EIGHT,
        '7' => Rank::SEVEN,
        '6' => Rank::SIX,
        '5' => Rank::FIVE,
        '4' => Rank::FOUR,
        '3' => Rank::THREE,
        '2' => Rank::TWO,
        _ => {
            panic!("Should not have any not cards! {}", char)
        }
    }
}
