use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let part1result = part1(&contents);
    let part2result = part2(&contents);

    println!("Part 1: {}", part1result);
    println!("Part 2: {}", part2result);
}

fn part1(contents: &String) -> i32 {
    todo!()
}

fn part2(contents: &String) -> i32 {
    todo!()
}