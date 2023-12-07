use std::ops::Add;
use std::{env, fs};
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let part1result = day1part1(&contents);
    let part2result = day1part2(&contents);

    println!("Part 1: {}", part1result);
    println!("Part 2: {}", part2result);
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
    result
}

fn day1part2(contents: &String) -> i32 {
    let mut results: Vec<i32> = Vec::new();
    let valid_words = HashMap::from([("one", "1"),("two", "2"),("three", "3"),("four", "4"),("five", "5"),("six", "6"),("seven", "7"),("eight", "8"),("nine", "9")]);

    let lines: Vec<&str> = contents.split("\n").collect();


    for &line in lines.iter() {
        let mut word = "".to_string();
        let mut numbers = "".to_string();

        for char in line.chars().into_iter() {
            if char.is_numeric() {
                 numbers.push(char);
            } else {
                word.push(char);
                if word.len() > 2 {
                    for key in valid_words.keys() {
                        if word.contains(key) {
                            numbers.push_str(valid_words.get(key).unwrap());
                            word = "".to_string();
                            word.push(char);
                            break;
                        }
                    };
                }
            }
        }
        let first = numbers.chars().collect::<Vec<_>>()[0];
        let last = numbers.chars().collect::<Vec<_>>()[numbers.len() - 1];

        let mut pair = "".to_string();
        pair.push(first);
        pair.push(last);

        results.push(pair.parse::<i32>().unwrap())
    }

    results.into_iter().sum::<i32>()
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