mod part_1;
mod part_2;
use std::{env, fs};


fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let part_1_result = part_1::process(&contents);
    let part_2_result = part_2::process(&contents);

    println!("Sum of Engine Part Numbers: {}", part_1_result);
    println!("Sum of Gear Numbers: {}", part_2_result);
}