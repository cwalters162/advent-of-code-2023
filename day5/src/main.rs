use std::{env, fs};
use std::collections::HashMap;

fn main() {
    let paths = [
        "src/test-input/1seeds.txt",
        "src/test-input/2seed-to-soil.txt",
        "src/test-input/3soil-to-fertilizer.txt",
        "src/test-input/4fertilizer-to-water.txt",
        "src/test-input/5water-to-light.txt",
        "src/test-input/6light-to-temperature.txt",
        "src/test-input/7humidity-to-location.txt",
    ];
    let seeds = process_seeds(paths[0]);
    let seed_to_soil = map_source_to_destination(paths[1], seeds);
    let soil_to_fertilizer = map_source_to_destination(paths[2], seed_to_soil);
    //let fertilizer_to_water = map_source_to_destination(paths[1], seeds);
    //let water-to-light = map_source_to_destination(paths[1], seeds);
    //let light-to-temperature = map_source_to_destination(paths[1], seeds);
    //let temperature-to-humidity = map_source_to_destination(paths[1], seeds);
    //let humidity-to-location = map_source_to_destination(paths[1], seeds);
    //process all maps

    dbg!(soil_to_fertilizer);
    // let result = process(&contents);
    // println!("Lowest Location: {}", result.0);
    // println!("TBD: {}", result.1);
}

fn process_seeds(file_path: &str) -> Vec<i32> {
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

fn map_source_to_destination(path_to_mapping_file: &str, input_for_mapping: Vec<i32>) -> Vec<i32> {
    let contents = fs::read_to_string(path_to_mapping_file)
        .expect("Cannot find 1seeds.txt. Please ensure it is in the directory provided");
    let removed_returns = contents.replace("\r", "");
    let mut lines = removed_returns.split("\n").collect::<Vec<_>>();
    lines.remove(0);
    let removed_whitespace = lines.iter().map(|line| line.split_whitespace().collect::<Vec<_>>()).collect::<Vec<_>>();
    let seed_soil_mapping = removed_whitespace.iter().map(|line| {
        let destination_start = line[0].parse::<i32>().unwrap();
        let source_start = line[1].parse::<i32>().unwrap();
        let range_length = line[2].parse::<i32>().unwrap();

        SeedSoilMapItem::new(destination_start,source_start, range_length)
    }).collect::<Vec<_>>();

    let mut seed_to_soil_results = Vec::<i32>::new();

    let mut seed_soil_map = HashMap::<i32, i32>::new();

    //Why did this take me over an hour to figure out???
    for mapping in seed_soil_mapping {
        let d_start = mapping.destination_start;
        let s_start = mapping.source_start;
        let range_length = mapping.range_length;

        for i in 0..range_length {
            seed_soil_map.insert(s_start+i, d_start+i);
        }
    }


    for source_seed in input_for_mapping.iter() {
        let destination = seed_soil_map.get(source_seed);
        match destination {
            Some(_) => seed_to_soil_results.push(destination.unwrap().clone()),
            None => seed_to_soil_results.push(source_seed.clone())
        }
    }

    seed_to_soil_results
}




// fn process_seed_to_soil(file_path: &String) -> Vec<i32> {
//     let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
//     let lines = contents.split("\n").collect::<Vec<_>>();
// }

#[derive(Debug)]
struct SeedSoilMapItem {
    destination_start: i32,
    source_start: i32,
    range_length: i32
}

impl SeedSoilMapItem {
    fn new(destination_start: i32, source_start: i32, range_length: i32) -> SeedSoilMapItem {
        SeedSoilMapItem {
            destination_start,
            source_start,
            range_length,
        }
    }
}