use std::ops::Add;
use std::{env, fs};

//test should be 353

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut results: Vec<u32> = Vec::new();

    for &line in lines.iter() {
        let numbers: Vec<_> = line
            .chars()
            .into_iter()
            .filter_map(|char| if char.is_numeric() { Some(char) } else { None })
            .collect();

        if numbers.len() <= 1 {
            let a = numbers[0];
            let b = numbers[0];
            let ab = a.to_string().add(&*b.to_string()).parse().unwrap();
            results.push(ab);
        } else {
            let a = numbers[0];
            let b = numbers[numbers.len() - 1];
            let ab: u32 = a.to_string().add(&*b.to_string()).parse().unwrap();
            results.push(ab);
        }
    }

    dbg!(&results[1]);
    println!("{:?}", results.iter().sum::<u32>())
}
