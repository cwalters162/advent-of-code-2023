use std::{fs};
use std::collections::VecDeque;

fn main() {
    let paths = [
        "src/test-input/1seeds.txt",
        "src/test-input/2seed-to-soil.txt",
        "src/test-input/3soil-to-fertilizer.txt",
        "src/test-input/4fertilizer-to-water.txt",
        "src/test-input/5water-to-light.txt",
        "src/test-input/6light-to-temperature.txt",
        "src/test-input/7temperature-to-humidity.txt",
        "src/test-input/8humidity-to-location.txt",
    ];
    let part1result = part1(paths);
    let part2result = part2(paths);
    println!("Part 1 Lowest Location: {}", part1result);
    println!("Part 2 Lowest Location: {}", part2result);
}

fn part1(paths: [&str; 8]) -> usize {
    println!("Processing seeds.");
    let seeds = process_seeds_part_1(paths[0]);
    println!("Processing seeds to soil.");
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
    humidity_to_location.iter().min().unwrap().clone()
}

fn process_seeds_part_1(file_path: &str) -> Vec<usize> {
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

fn part2(paths: [&str; 8]) -> usize {
    println!("Processing seeds.");
    let seeds = process_part_2(paths[0]);
    // println!("Processing seeds to soil.");
    // let seed_to_soil = map_source_to_destination(paths[1], seeds);
    // println!("Processing soil to fertilizer.");
    // let soil_to_fertilizer = map_source_to_destination(paths[2], seed_to_soil);
    // println!("Processing fertilizer to water.");
    // let fertilizer_to_water = map_source_to_destination(paths[3], soil_to_fertilizer);
    // println!("Processing water to light.");
    // let water_to_light = map_source_to_destination(paths[4], fertilizer_to_water);
    // println!("Processing light to temperature.");
    // let light_to_temperature = map_source_to_destination(paths[5], water_to_light);
    // println!("Processing temperature to humidity.");
    // let temperature_to_humidity = map_source_to_destination(paths[6], light_to_temperature);
    // println!("Processing humidity to location.");
    // let humidity_to_location = map_source_to_destination(paths[7], temperature_to_humidity);
    // humidity_to_location.iter().min().unwrap().clone()
    0
}


fn process_part_2(file_path: &str) -> Vec<SeedRange> {
    let contents = fs::read_to_string(file_path)
        .expect("Cannot find file. Please ensure it is in the directory provided");
    let mut split_on_whitespace = contents.split_whitespace().collect::<VecDeque<_>>();
    split_on_whitespace.pop_front();
    let mut seed_ranges = Vec::<SeedRange>::new();
    let mut i = 0;
    while i < split_on_whitespace.len() - 1 {
        let start = split_on_whitespace[i].parse::<usize>().unwrap();
        let length = split_on_whitespace[i + 1].parse::<usize>().unwrap();
        let new_seed_range = SeedRange::new(start, length);
        seed_ranges.push(new_seed_range);
        i += 1;
    }

    println!("Finished processing");
    dbg!(&seed_ranges);
    seed_ranges
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

    println!("Processing through the source to build output values.");
    for &source in input_for_mapping.iter() {
        let mut was_inserted = false;
        for item in source_destination_items.iter() {
            let is_source_between = source >= item.source_start && source <= (item.source_start + item.range_length);
            if is_source_between {
                let number = source - item.source_start;
                let value_to_push = number + item.destination_start;
                was_inserted = true;
                source_to_destination_results.push(value_to_push)
            }
        }
        if was_inserted {
            continue
        } else {
            source_to_destination_results.push(source);
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

#[derive(Debug)]
struct SeedRange {
    start: usize,
    length: usize
}

impl SeedRange {
    fn new(start: usize, length: usize) -> SeedRange {
        SeedRange {
            start,
            length,
        }
    }
}