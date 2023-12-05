use std::{env, fs};

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
    valid_game: bool,
    minimum_red: u32,
    minimum_green: u32,
    minimum_blue: u32,
}

#[derive(Debug, Clone)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let result = process(&contents);
    println!("Sum of Game IDs: {}", result.0);
    println!("Sum of Cube Powers: {}", result.1);
}

fn process(contents: &String) -> (u32, u32) {
    let red_cube_limit = 12;
    let green_cube_limit = 13;
    let blue_cube_limit = 14;

    let lines: Vec<&str> = contents.split("\n").collect();
    let games: Vec<_> = into_games(lines);

    let validated_games = games
        .into_iter()
        .map(|mut game| {
            let highest_red = game.rounds.iter().map(|round| round.red).max().unwrap();
            let highest_blue = game.rounds.iter().map(|round| round.blue).max().unwrap();
            let highest_green = game.rounds.iter().map(|round| round.green).max().unwrap();

            game.minimum_blue = highest_blue;
            game.minimum_green = highest_green;
            game.minimum_red = highest_red;

            if highest_blue > blue_cube_limit {
                game
            } else if highest_red > red_cube_limit {
                game
            } else if highest_green > green_cube_limit {
                game
            } else {
                game.valid_game = true;
                game
            }
        })
        .collect::<Vec<_>>();

    let game_id_sum_result: u32 = validated_games
        .clone()
        .iter()
        .filter(|game| game.valid_game)
        .map(|game| game.id)
        .sum();

    let minimum_cubes_per_game_power_sum: u32 = validated_games
        .iter()
        .map(|game| {
            let product = game.minimum_red * game.minimum_green * game.minimum_blue;
            product
        })
        .sum();

    (game_id_sum_result, minimum_cubes_per_game_power_sum)
}

fn into_games(lines: Vec<&str>) -> Vec<Game> {
    lines
        .into_iter()
        .map(|line| {
            // Getting game id
            let split_on_colon: Vec<&str> = line.split(":").collect();
            let split_game_on_whitespace: Vec<_> = split_on_colon[0].split_whitespace().collect();
            let game_id: u32 = split_game_on_whitespace[split_game_on_whitespace.len() - 1]
                .parse()
                .unwrap();

            // Getting Rounds
            let split_on_semi_colon = split_on_colon[1].split(";").collect::<Vec<_>>();
            let split_on_comma = split_on_semi_colon
                .into_iter()
                .map(|round| round.split(",").collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let split_on_white_space: Vec<_> = split_on_comma
                .into_iter()
                .map(|game| {
                    game.into_iter()
                        .map(|round| round.trim_start().split_whitespace().collect::<Vec<_>>())
                        .collect::<Vec<_>>()
                })
                .collect();
            let rounds: Vec<_> = split_on_white_space
                .into_iter()
                .map(|round| -> Round {
                    let mut red = 0;
                    let mut blue = 0;
                    let mut green = 0;

                    for hand in round {
                        let amount = hand[0].parse::<u32>().unwrap();
                        let color = hand[1];

                        match color {
                            "red" => {
                                red += amount;
                            }
                            "green" => {
                                green += amount;
                            }
                            "blue" => {
                                blue += amount;
                            }
                            _ => {}
                        };
                    }
                    Round { red, blue, green }
                })
                .collect();
            Game {
                id: game_id,
                rounds,
                valid_game: false,
                minimum_red: 0,
                minimum_green: 0,
                minimum_blue: 0,
            }
        })
        .collect()
}

#[test]
fn test_part_1() {
    let content = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();

    let (sum, _) = process(&content);
    assert_eq!(sum, 8)
}

#[test]
fn test_part_2() {
    let content = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();

    let (_, product_sum) = process(&content);
    assert_eq!(product_sum, 2286)
}
