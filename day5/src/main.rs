use std::num::ParseIntError;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];

    let seeds = process_seeds(&file_path);
    //let seed_to_soil = process_seeds();
    //let soil_to_fertilizer = process_seeds();
    //let fertilizer_to_water = process_seeds();
    //let water-to-light = process_seeds();
    //let light-to-temperature = process_seeds();
    //let temperature-to-humidity = process_seeds();
    //let humidity-to-location = process_seeds();
    //process all maps

    // let result = process(&contents);
    // println!("Lowest Location: {}", result.0);
    // println!("TBD: {}", result.1);
}

fn process_seeds(file_path_prefix: &String) -> Vec<i32> {
    let mut file_path = file_path_prefix.clone();
    file_path.push_str("/1seeds.txt");
    dbg!(&file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Cannot find 1seeds.txt. Please ensure it is in the directory provided");
    let split_on_whitespace = contents.split_whitespace().collect::<Vec<_>>();
    let parse_to_numbers = split_on_whitespace
        .iter()
        .filter_map(|possible_number| {
            let maybe_number = possible_number.parse::<i32>();
            match maybe_number {
                Ok(_) => Some(maybe_number.unwrap()),
                Err(_) => None,
            }
        })
        .collect::<Vec<_>>();
    parse_to_numbers
}

// fn process_seed_to_soil(file_path: &String) -> Vec<i32> {
//     let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
//     let lines = contents.split("\n").collect::<Vec<_>>();
// }
