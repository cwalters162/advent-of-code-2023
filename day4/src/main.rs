use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let result = process(contents);
    println!("Sum of Winning Scratchcards: {}", result.0);
    println!("Sum of Cards: {}.", result.1);
}


#[test]
fn test_part_1() {
    let content = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string();
    let (part1, _) = process(content);
    assert_eq!(part1, 13)
}

#[test]
fn test_part_1_again() {
    let content = "Card   1:  8 86 59 90 68 52 55 24 37 69 | 10 55  8 86  6 62 69 68 59 37 91 90 24 22 78 61 58 89 52 96 95 94 13 36 81
Card   2:  6 42 98  5 17 31 13 36 63 61 | 99 88 14 20 63  5 56 33  6 21 92 13 17  7 31 93 30 74 98 15 11 36 61 42 47
Card   3: 16 58 72 77  1 67 33 82 68  7 | 16 37 15 75 78  1 49 82 22 45 83 58 77 79 50 88 98 67 33 72 42 29 35  7 68".to_string();
    let (part1, _) = process(content);
    assert_eq!(part1, 13)
}
fn process(content: String) -> (i32, i32) {
    let lines = content.split("\n").collect::<Vec<_>>();

    let split_on_colon = lines.iter().map(|card| {
        card.split(":").collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut cards = parse_into_cards(&split_on_colon);

    let part_1_results = cards.iter().map(|card| {
        if card.number_matching_values > 2 {
            double_n_times(card.number_matching_values)
        } else {
            card.number_matching_values
        }
    }).collect::<Vec<i32>>().iter().sum::<i32>();

    let part_2_results = part_2(cards);

    (part_1_results, part_2_results)
}

fn part_2(global_cards: Vec<Card>) -> i32 {
    fn process_cards(acc: i32, mut global_cards: &mut Vec<Card>, reference_cards: Vec<Card>) -> i32 {
        println!("Processed {} cards. Remaining: {:?}", &acc, &global_cards.len());

        if global_cards.len() == 0 {
            return acc
        }

        let current_card = global_cards.remove(0);
        println!("Processing Card: {}.", &current_card.id);

        for i in 0..current_card.number_matching_values {
            let next_card_index = current_card.id + i;
            if i < global_cards.len() as i32 {
                let next_card: Card = reference_cards[next_card_index as usize].clone();
                global_cards.push(next_card);
            }
        }

        process_cards(acc + 1, global_cards, reference_cards)
    }
    return process_cards(0, &mut global_cards.clone(), global_cards)
}

fn parse_card_id(left_of_colon: &Vec<Vec<&str>>) -> Vec<i32> {
    let card_id_split = left_of_colon.iter().map(|card| {
        card[0].split_whitespace().collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let card_ids = card_id_split.into_iter().map(|card_id| {
        card_id[1].parse::<i32>().unwrap()
    }).collect::<Vec<_>>();
    card_ids
}

fn parse_into_cards(split_on_colon: &Vec<Vec<&str>>) -> Vec<Card> {
    let mut cards = Vec::<Card>::new();

    let card_ids = parse_card_id(&split_on_colon);

    let no_card_ids = split_on_colon.iter().map(|card| {
        card[1]
    }).collect::<Vec<_>>();

    let winning_values = no_card_ids.iter().map(|card| {
        let split_values = card.split("|").collect::<Vec<_>>();
        let str_values = split_values[0].split_whitespace().collect::<Vec<_>>();

        str_values.iter().map(|value| {
            value.parse::<i32>().unwrap()
        }).collect::<Vec<_>>()

    }).collect::<Vec<_>>();

    let card_values = no_card_ids.iter().map(|card| {
        let split_values = card.split("|").collect::<Vec<_>>();
        let str_values = split_values[1].split_whitespace().collect::<Vec<_>>();

        str_values.iter().map(|value| {
            value.parse::<i32>().unwrap()
        }).collect::<Vec<_>>()

    }).collect::<Vec<_>>();


    for (i, card_id) in card_ids.iter().enumerate() {
        let mut scores_matching = 0;
        for wv in winning_values[i].iter() {
            if card_values[i].contains(&wv) {
                scores_matching += 1
            }
        }
        cards.push(Card::new(card_id.clone(), winning_values[i].clone(), card_values[i].clone(), scores_matching))
    }

    cards
}

fn double_n_times(amount: i32) -> i32 {
    let mut value = 1;
    for _ in 1..amount {
        value = value * 2;
    }
    value
}

#[derive(Clone, Debug)]
struct Card {
    pub id: i32,
    pub winning_values: Vec<i32>,
    pub card_values: Vec<i32>,
    pub number_matching_values: i32,
    pub count: i32,
}

impl Card {
    fn new(
        id: i32,
        winning_values: Vec<i32>,
        card_values: Vec<i32>,
        number_matching_values: i32
    ) -> Card {
        Card {
            id,
            winning_values,
            card_values,
            number_matching_values,
            count: 1,
        }
    }
}

impl Default for Card {
    fn default() -> Self {
        Card {
            id: 0,
            winning_values: Vec::<i32>::new(),
            card_values: Vec::<i32>::new(),
            number_matching_values: 0,
            count: 0,
        }
    }
}