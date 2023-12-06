use std::{env, fs};
use std::collections::HashMap;

fn main() {
    let paths = [
        "src/input/1seeds.txt",
        "src/input/2seed-to-soil.txt",
        "src/input/3soil-to-fertilizer.txt",
        "src/input/4fertilizer-to-water.txt",
        "src/input/5water-to-light.txt",
        "src/input/6light-to-temperature.txt",
        "src/input/7temperature-to-humidity.txt",
        "src/input/8humidity-to-location.txt",
    ];
    println!("Processing seeds.");
    let seeds = process_seeds(paths[0]);
    println!("Processing seds to soil.");
    let seed_to_soil = map_source_to_destination(paths[1], seeds);
    println!("Processing soil to fertilizer.");
    let soil_to_fertilizer = map_source_to_destination(paths[2], seed_to_soil);
    println!("Processing fertilizer to water.");
    let fertilizer_to_water = map_source_to_destination(paths[3], soil_to_fertilizer);
    println!("Processing water to light.");
    let water_to_light = map_source_to_destination(paths[4], fertilizer_to_water);
    println!("Processing light to temperature.");
    let light_to_temperature = map_source_to_destination(paths[5], water_to_light);
    println!("Processing temperature to humidity.");
    let temperature_to_humidity = map_source_to_destination(paths[6], light_to_temperature);
    println!("Processing humidity to location.");
    let humidity_to_location = map_source_to_destination(paths[7], temperature_to_humidity);

    println!("Lowest Location: {}", humidity_to_location.iter().min().unwrap());
    // println!("TBD: {}", result.1);
}

fn process_seeds(file_path: &str) -> Vec<usize> {
    let contents = fs::read_to_string(file_path)
        .expect("Cannot find 1seeds.txt. Please ensure it is in the directory provided");
    let split_on_whitespace = contents.split_whitespace().collect::<Vec<_>>();
    let parse_to_numbers = split_on_whitespace
        .iter()
        .filter_map(|possible_number| {
            let maybe_number = possible_number.parse::<usize>();
            match maybe_number {
                Ok(_) => Some(maybe_number.unwrap()),
                Err(_) => None,
            }
        })
        .collect::<Vec<_>>();
    println!("Finished processing seeds");
    parse_to_numbers
}

fn map_source_to_destination(path_to_mapping_file: &str, input_for_mapping: Vec<usize>) -> Vec<usize> {
    let contents = fs::read_to_string(path_to_mapping_file)
        .expect(&*format!("Cannot find file: {path_to_mapping_file}."));
    let removed_returns = contents.replace("\r", "");
    let mut lines = removed_returns.split("\n").collect::<Vec<_>>();
    lines.remove(0);
    let removed_whitespace = lines.iter().map(|line| line.split_whitespace().collect::<Vec<_>>()).collect::<Vec<_>>();
    println!("Making SD items.");
    let source_destination_items = removed_whitespace.iter().map(|line| {
        let destination_start = line[0].parse::<usize>().unwrap();
        let source_start = line[1].parse::<usize>().unwrap();
        let range_length = line[2].parse::<usize>().unwrap();

        SourceDestinationItem::new(destination_start, source_start, range_length)
    }).collect::<Vec<_>>();

    let mut source_to_destination_results = Vec::<usize>::new();

    let mut source_destination_hashmap = HashMap::<usize, usize>::new();

    println!("Mapping over to build hashmaps.");
    //Why did this take me over an hour to figure out???
    //How do I make this faster?!!
    for mapping in source_destination_items.iter() {
        let d_start = mapping.destination_start;
        let s_start = mapping.source_start;
        let range_length = mapping.range_length;



        for i in 0..range_length {
            let percent_complete = (i as f64 / range_length as f64) * 100.0;
            println!("Percent complete: {:.1$}%", percent_complete, 2);

            source_destination_hashmap.insert(s_start+i, d_start+i);
        }
    }

    println!("Processing through the source to build output values.");
    for source in input_for_mapping.iter() {
        let destination = source_destination_hashmap.get(source);
        match destination {
            Some(_) => source_to_destination_results.push(destination.unwrap().clone()),
            None => source_to_destination_results.push(source.clone())
        }
    }

    source_to_destination_results
}

#[derive(Debug)]
struct SourceDestinationItem {
    destination_start: usize,
    source_start: usize,
    range_length: usize
}

impl SourceDestinationItem {
    fn new(destination_start: usize, source_start: usize, range_length: usize) -> SourceDestinationItem {
        SourceDestinationItem {
            destination_start,
            source_start,
            range_length,
        }
    }
}