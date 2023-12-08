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

    let part1result = process(&contents, false);
    let part2result = process(&contents, true);

    println!("Part 1: {}", part1result);
    println!("Part 2: {}", part2result);
}

fn process(contents: &String, with_joker: bool) -> i32 {
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
        hand.discover_hand_type(with_joker);
    }

    hands.sort_by(|left, right| left.hand_type.partial_cmp(&right.hand_type).unwrap());
    hands.sort_by(|left, right| {
        if left.hand_type != right.hand_type {
            return Ordering::Equal;
        }
        let left_cards = left.cards.chars().collect::<Vec<char>>();
        let right_cards = right.cards.chars().collect::<Vec<char>>();
        for i in 0..left.cards.len() {
            let left_card = get_rank_from_char(left_cards[i], with_joker);
            let right_card = get_rank_from_char(right_cards[i], with_joker);
            let result = left_card.partial_cmp(&right_card).unwrap();
            if result == Ordering::Equal {
                continue;
            } else {
                return result;
            }
        }
        Ordering::Equal
    });

    dbg![&hands];


    hands.reverse();

    let bets = hands.iter().map(|hand| hand.bet);
    let mut bet_result = 0;
    for i in 0..bets.len() {
        let hand_bet = hands.get(i).unwrap().bet;
        let multiplier = 1 + i as i32;
        bet_result += hand_bet * multiplier;
    }
    bet_result
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    cards: String,
    bet: i32,
}

impl Hand {
    fn discover_hand_type(&mut self, with_joker: bool) {
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
        let mut four_kind = 0;
        let mut three_kind = false;
        let mut joker_count = 0;

        for (card, count) in found_cards {
            if card == 'J' && with_joker {
                joker_count += count;
            }
             match count {
                2 => {
                    pairs += 1;
                }
                3 => {
                    three_kind = true;
                }
                4 => {
                    four_kind += 1;
                }
                _ => {}
            }
        }

        match (pairs, three_kind, four_kind) {

            //2222J // four kind upgrade to five kind
            //JJJJ1 // four kind upgrade to ive kind
            //FOUR KIND
            (0, false, 1) => {
                if joker_count == 1 || joker_count == 4 {
                    self.hand_type = FIVEKIND;
                } else {
                    self.hand_type = FOURKIND;
                }
            },
            //33322 // full house
            //333JJ // full house upgrade to five kind
            //JJJ22 // full house upgrades five kind
            //FULL HOUSE
            (1, true, 0) => {
                if joker_count >= 2 {
                    self.hand_type = FIVEKIND;
                } else {
                    self.hand_type = FULLHOUSE;
                }
            }

            //22234 //three kind
            //222J3 //three kind upgrade to a four kind
            //JJJ24 //THREE KIND upgrade to a four kind
            // THREE KIND
            (0, true, 0) => {
                if joker_count == 1 || joker_count == 3 {
                    self.hand_type = FOURKIND;
                } else {
                    self.hand_type = THREEKIND;
                }
            }

            //J2233 // two pair upgrade to a full house
            //3JJ22 // two pair upgrade to four kind
            //22334 // two pair
            // TWO PAIR
            (2, false, 0) => {
                if joker_count == 1 {
                    self.hand_type = FULLHOUSE;
                } else if joker_count == 2 {
                    self.hand_type = FOURKIND;
                } else {
                    self.hand_type = TWOPAIR;
                }
            }

            //JJ345 // one pair upraded to threekind
            //22J45 // one pair upgraded to threekind
            //22345 // one pair
            // ONE PAIR
            (1, false, 0) => {
                if joker_count >= 1 {
                    self.hand_type = THREEKIND;
                } else {
                    self.hand_type = ONEPAIR;
                }
            }
            //23J56 // high card upgraded to one pair
            //23456 // high Card
            // HIGH CARD
            (_, _, _) => {
                if joker_count == 1{
                    self.hand_type = ONEPAIR;
                } else {
                    self.hand_type = HIGHCARD;
                }
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
    JOKER,
}

fn get_rank_from_char(char: char, with_joker: bool) -> Rank {
    match char {
        'A' => Rank::ACE,
        'K' => Rank::KING,
        'Q' => Rank::QUEEN,
        'J' => {
            if with_joker {
                Rank::JOKER
            } else {
                Rank::JACK
            }
        },
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

