use std::{env, fs};
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let result = part1(contents);
    println!("Sum of Winning Scratchcards: {}", result.0);
    println!("TDB: {}.", result.1);
}


#[test]
fn test_part_1() {
    let content = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string();
    let (part1, _) = part1(content);
    assert_eq!(part1, 13)
}

#[test]
fn test_part_1_again() {
    let content = "Card   1:  8 86 59 90 68 52 55 24 37 69 | 10 55  8 86  6 62 69 68 59 37 91 90 24 22 78 61 58 89 52 96 95 94 13 36 81
Card   2:  6 42 98  5 17 31 13 36 63 61 | 99 88 14 20 63  5 56 33  6 21 92 13 17  7 31 93 30 74 98 15 11 36 61 42 47
Card   3: 16 58 72 77  1 67 33 82 68  7 | 16 37 15 75 78  1 49 82 22 45 83 58 77 79 50 88 98 67 33 72 42 29 35  7 68".to_string();
    let (part1, _) = part1(content);
    assert_eq!(part1, 13)
}
fn part1(content: String) -> (i32, i32) {
    let lines = content.split("\n").collect::<Vec<_>>();

    let split_on_colon = lines.iter().map(|game| {
        game.split(":").collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let game_ids = parse_game_id(&split_on_colon);

    let game_results = parse_game_results(&split_on_colon);
    let game_results_sum = game_results.iter().sum();
    dbg!(game_results);
    (game_results_sum, 0)
}

fn parse_game_id(left_of_colon: &Vec<Vec<&str>>) -> Vec<usize> {
    let game_id_split = left_of_colon.iter().map(|game| {
        game[0].split_whitespace().collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let game_ids = game_id_split.into_iter().map(|game_id| {
        game_id[1].parse::<usize>().unwrap()
    }).collect::<Vec<_>>();
    game_ids
}

fn parse_game_results(split_on_colon: &Vec<Vec<&str>>) -> Vec<i32> {
    let no_game_ids = split_on_colon.iter().map(|game| {
        game[1]
    }).collect::<Vec<_>>();

    let winning_values = no_game_ids.iter().map(|game| {
        let split_values = game.split("|").collect::<Vec<_>>();
        split_values[0].split_whitespace().collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let card_values = no_game_ids.iter().map(|game| {
        let split_values = game.split("|").collect::<Vec<_>>();
        split_values[1].split_whitespace().collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut card_scores = Vec::new();

    for (i, game) in winning_values.iter().enumerate() {
        let mut scores_matching = 0;
        for wv in game {
            for cv in card_values[i].iter() {
                if wv == cv {
                    scores_matching += 1;
                } else {
                    continue
                }
            }
        }
        match scores_matching {
            0 => card_scores.push(0),
            1 => card_scores.push(1),
            2 => card_scores.push(2),
            _ => card_scores.push(double_n_times(scores_matching)),
        }
    }

    card_scores
}

fn double_n_times(amount: i32) -> i32 {
    let mut value = 1;
    for _ in 1..=amount {
        value = amount * 2;
    }
    value
}