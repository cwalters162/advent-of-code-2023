use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let part1result = part_1(&contents);
    let part2result = part_2(&contents);

    println!("Part 1: {}", part1result);
    println!("Part 2: {}", part2result);
}

fn part_1(contents: &String) -> i32 {
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let removed_whitespace = lines.iter().map(|line| {
        line.split_whitespace().collect::<Vec<&str>>()
    }).collect::<Vec<Vec<&str>>>();
    let mut races = Vec::<Race>::new();
    let time = removed_whitespace.get(0).unwrap();
    let distant = removed_whitespace.get(1).unwrap();
    for i in 1..time.len() {
        races.push(
            Race {
                time: time[i as usize].clone().parse::<i32>().unwrap(),
                record_distance: distant[i as usize].clone().parse::<i32>().unwrap(),
                ways_to_win: 0,
            }
        )
    }

    for race in races.iter_mut() {
        race.process_ways();
    }

    let result = races.iter().map(|race| race.ways_to_win).product::<i32>();

    dbg![&result];
    result
}

fn part_2(contents: &String) -> i32 {
    0
}

#[derive(Debug)]
struct Race {
    time: i32,
    record_distance: i32,
    ways_to_win: i32
}

impl Race {
    fn process_ways(&mut self) {
        //hold button for time gets speed for distance.
        let mut distance_traveled = 0;
        let mut speed = 0;
        for time_button_held in 0..self.time {
            speed = time_button_held;
            for _current_distance_traveled in time_button_held..self.time {
                distance_traveled += speed;
            }
            if distance_traveled > self.record_distance {
                self.ways_to_win += 1;
            }
            distance_traveled = 0;
        }
    }
}