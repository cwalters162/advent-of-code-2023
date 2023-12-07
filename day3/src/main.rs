use std::{env, fs};
//BROKEN!!!!!!!
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let result = process(&contents);
    println!("Sum of Engine Part Numbers: {}", result.0);
    println!("Sum of Gear Numbers: {}", result.1);
}

fn process(content: &String) -> (u32, u32) {
    let lines = content.split("\n").collect::<Vec<&str>>();
    let cmp_lines = lines.clone();

    let rows_of_dirty_not_validated_numbers: Vec<_> = lines
        .iter()
        .enumerate()
        .map(|(r, line)| {
            let mut chars_with_metadata: Vec<Vec<CharMetadata>> = Vec::new();
            let mut current_number_vec: Vec<CharMetadata> = Vec::new();

            for (c, char) in line.chars().enumerate() {
                if char.is_numeric() {
                    current_number_vec.push(CharMetadata {
                        c: char,
                        near_symbol: check_neighbors(r, c, &cmp_lines),
                    });
                    continue;
                } else {
                    if c == 0 {
                        continue;
                    }
                    let middle_left = line.chars().map(|c| c).collect::<Vec<char>>()[c - 1];

                    if middle_left.is_numeric() {
                        chars_with_metadata.push(current_number_vec.clone());
                        current_number_vec = Vec::new();
                    } else {
                        continue;
                    }
                }
            }
            chars_with_metadata
        })
        .collect();
    let rows_of_not_validated_numbers: Vec<_> = rows_of_dirty_not_validated_numbers
        .iter()
        .filter(|row| row.len() >= 1)
        .collect();
    let validated_chars: Vec<_> = rows_of_not_validated_numbers
        .iter()
        .filter_map(|row| {
            let result: Vec<_> = row
                .into_iter()
                .filter_map(|chars| {
                    let result = chars
                        .iter()
                        .filter(|char| char.near_symbol)
                        .collect::<Vec<&CharMetadata>>();
                    if result.len() >= 1 {
                        let mut number_string = String::new();
                        let _ = chars.iter().map(|c| number_string.push(c.c)).collect::<Vec<_>>();
                        let number: u32 = number_string.parse().unwrap();
                        Some(number)
                    } else {
                        None
                    }
                })
                .collect();
            Some(result)
        })
        .collect();

    let flat = validated_chars.iter().flatten().collect::<Vec<&u32>>();
    let result_sum: u32 = flat.iter().copied().clone().sum();

    (result_sum, 0u32)
}

fn check_neighbors(r: usize, c: usize, lines: &Vec<&str>) -> bool {
    if r == 0 && c == 0 {
        //is_top_left?
        check_middle_right(r, c, lines)
            || check_bottom_middle(r, c, lines)
            || check_bottom_right(r, c, lines)
    } else if r == 0 && c == lines[r].len() - 1 {
        //is_top_right?
        check_middle_left(r, c, lines)
            || check_bottom_left(r, c, lines)
            || check_bottom_middle(r, c, lines)
    } else if r == lines.len() - 1 && c == 0 {
        //is_bottom_left?
        check_top_middle(r, c, lines)
            || check_top_right(r, c, lines)
            || check_middle_right(r, c, lines)
    } else if r == lines.len() - 1 && c == lines[r].len() - 1 {
        //is_bottom_right?
        check_top_left(r, c, lines)
            || check_top_middle(r, c, lines)
            || check_middle_left(r, c, lines)
    } else if r == lines.len() - 1 && c != 0 && c != lines[r].len() - 1 {
        //is_bottom_middle?
        check_middle_left(r, c, lines)
            || check_middle_right(r, c, lines)
            || check_top_left(r, c, lines)
            || check_top_middle(r, c, lines)
            || check_top_right(r, c, lines)
    } else if r == 0 && r != lines.len() - 1 && c != 0 && c != lines[r].len() - 1 {
        // Top middle
            check_middle_left(r,c, lines)
            || check_middle_right(r, c, lines) ||
                check_bottom_left(r, c, lines)
            || check_bottom_middle(r, c, lines)
            || check_bottom_right(r, c, lines)
    } else if r != 0 && r != lines.len() - 1 && c != 0 && c != lines[r].len() - 1 {
        //Middle of the 2d array
        check_top_left(r, c, lines)
            || check_top_middle(r, c, lines)
            || check_top_right(r, c, lines)
            || check_middle_left(r, c, lines)
            || check_middle_right(r, c, lines)
            || check_bottom_left(r, c, lines)
            || check_bottom_middle(r, c, lines)
            || check_bottom_right(r, c, lines)
    } else {
        false
    }
}

fn check_top_left(r: usize, c: usize, lines: &Vec<&str>) -> bool {
    let location: char = lines[r - 1].chars().collect::<Vec<char>>()[c - 1];
    is_ascii_graphic_excluding_period(location)
}

fn check_top_middle(r: usize, c: usize, lines: &Vec<&str>) -> bool {
    let location: char = lines[r - 1].chars().collect::<Vec<char>>()[c];
    is_ascii_graphic_excluding_period(location)
}

fn check_top_right(r: usize, c: usize, lines: &Vec<&str>) -> bool {
    let location: char = lines[r - 1].chars().collect::<Vec<char>>()[c + 1];
    is_ascii_graphic_excluding_period(location)
}

fn check_middle_left(r: usize, c: usize, lines: &Vec<&str>) -> bool {
    let location: char = lines[r].chars().collect::<Vec<char>>()[c - 1];
    is_ascii_graphic_excluding_period(location)
}

fn check_middle_right(r: usize, c: usize, lines: &Vec<&str>) -> bool {
    let location: char = lines[r].chars().collect::<Vec<char>>()[c + 1];
    is_ascii_graphic_excluding_period(location)
}

fn check_bottom_left(r: usize, c: usize, lines: &Vec<&str>) -> bool {
    let location: char = lines[r + 1].chars().collect::<Vec<char>>()[c - 1];
    is_ascii_graphic_excluding_period(location)
}

fn check_bottom_middle(r: usize, c: usize, lines: &Vec<&str>) -> bool {
    let location: char = lines[r + 1].chars().collect::<Vec<char>>()[c];
    is_ascii_graphic_excluding_period(location)
}

fn check_bottom_right(r: usize, c: usize, lines: &Vec<&str>) -> bool {
    let location: char = lines[r + 1].chars().collect::<Vec<char>>()[c + 1];
    is_ascii_graphic_excluding_period(location)
}
fn is_ascii_graphic_excluding_period(char: char) -> bool {
    matches!(char, '!'..='-' | '/' | ':'..='@' | '['..='`' | '{'..='~')
}

#[derive(Clone, Debug)]
struct CharMetadata {
    c: char,
    near_symbol: bool,
}

#[test]
fn test_part_1() {
    let content = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..".to_string();
    let (result, _) = process(&content);
    assert_eq!(result, 4361);
}
#[test]
fn test_part_2() {
    let content = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..".to_string();
    let (_, result) = process(&content);
    assert_eq!(result, 4361);
}
