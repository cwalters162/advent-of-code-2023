use std::ops::Add;
use std::{env, fs};

//test should be 353

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    day1part1(&contents);
    day1part2(&contents);
}

fn day1part1(contents: &String) -> u32 {
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut results: Vec<u32> = Vec::new();

    for &line in lines.iter() {
        let numbers: Vec<_> = line
            .chars()
            .into_iter()
            .filter_map(|char| if char.is_numeric() { Some(char) } else { None })
            .collect();

        if numbers.len() == 1 {
            let a = numbers[0];
            let b = numbers[0];
            let ab = a.to_string().add(&*b.to_string()).parse().unwrap();
            results.push(ab);
        } else if numbers.len() >= 2 {
            let a = numbers[0];
            let b = numbers[numbers.len() - 1];
            let ab: u32 = a.to_string().add(&*b.to_string()).parse().unwrap();
            results.push(ab);
        }
    }
    let result = results.iter().sum::<u32>();
    dbg!(&results[1]);
    println!("{:?}", &result);
    result
}

fn day1part2(contents: &String) {
    let mut results: Vec<u32> = Vec::new();
    let lines: Vec<&str> = contents.split("\n").collect();

    for &line in lines.iter() {
        let numbers: Vec<_> = line
            .chars()
            .into_iter()
            .filter_map(|char| if char.is_numeric() { Some(char) } else { None })
            .collect();

        if numbers.len() == 1 {
            let a = numbers[0];
            let b = numbers[0];
            let ab = a.to_string().add(&*b.to_string()).parse().unwrap();
            results.push(ab);
        } else if numbers.len() >= 2 {
            let a = numbers[0];
            let b = numbers[numbers.len() - 1];
            let ab: u32 = a.to_string().add(&*b.to_string()).parse().unwrap();
            results.push(ab);
        }
    }
    let result = results.iter().sum::<u32>();
    dbg!(&results[1]);
    println!("{:?}", &result);
    result
}

#[test]
fn testday1part1() {
    let content = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet".to_string();

    let result = day1part1(&content);
    assert_eq!(result, 142);
}

#[test]
fn testday1part2() {
    let content = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen".to_string();
    let result = day1part2(&content);

    assert_eq!(result, 281)
}