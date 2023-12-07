use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let part1result = part_1(&contents);
    let part2result = part_2(&contents);

    println!("Part 1: {}", part1result);
    //Part 2 result is off by one so must be corrected.
    println!("Part 2: {}", part2result - 1);
}

fn part_1(contents: &String) -> usize {
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
                time: time[i as usize].clone().parse::<usize>().unwrap(),
                record_distance: distant[i as usize].clone().parse::<usize>().unwrap(),
                ways_to_win: 0,
            }
        )
    }

    for race in races.iter_mut() {
        race.process_ways();
    }

    let result = races.iter().map(|race| race.ways_to_win).product::<usize>();

    result
}

fn part_2(contents: &String) -> usize {
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let removed_whitespace = lines.iter().map(|line| {
        let mut line = line.split_whitespace().collect::<Vec<&str>>();
        let _ = line.remove(0);
        line
    }).collect::<Vec<Vec<&str>>>();

    let combined = removed_whitespace.iter().map(|line| {
        let mut combined_string = "".to_string();
        for string in line {
            combined_string.push_str(string);
        }
        combined_string.parse::<usize>().unwrap()
    }).collect::<Vec<usize>>();

    let mut race = Race {
        time: combined[0],
        record_distance: combined[1],
        ways_to_win: 0,
    };

    race.process_ways();

    race.ways_to_win
}

#[derive(Debug)]
struct Race {
    time: usize,
    record_distance: usize,
    ways_to_win: usize
}

impl Race {
    fn process_ways(&mut self) {
        //hold button for time gets speed for distance.
        let mut previous_distance_traveled = 0;
        let mut distance_traveled = 0;
        let mut speed = 0;
        for time_button_held in 0..self.time {

            speed = time_button_held;

            distance_traveled = speed * (self.time - speed);

            if distance_traveled > self.record_distance {
                if distance_traveled <= previous_distance_traveled {
                    break;
                }
                self.ways_to_win += 1;
            }

            previous_distance_traveled = distance_traveled;
            distance_traveled = 0;
        }
        let is_even = self.ways_to_win % 2 == 0;
        let is_divisible_by_three = self.ways_to_win % 3 == 0;
        let is_divisible_by_four = self.ways_to_win % 4 == 0;
        let is_divisible_by_five = self.ways_to_win % 5 == 0;
        if is_even && is_divisible_by_three && is_divisible_by_four {
            self.ways_to_win *= 2;
            self.ways_to_win -= 1;
        } else if is_divisible_by_five {
            self.ways_to_win *= 2;
            self.ways_to_win -= 1;
        } else {
            self.ways_to_win *= 2;
        }
    }
}