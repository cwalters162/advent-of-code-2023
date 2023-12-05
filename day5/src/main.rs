use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let result = process(&contents);
    println!("Sum of Game IDs: {}", result.0);
    println!("Sum of Cube Powers: {}", result.1);
}

fn process(p0: &String) -> _ {
    todo!()
}
