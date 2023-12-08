use std::collections::VecDeque;

pub fn process(content: &String) -> usize {
    let lines = content.split("\n").collect::<Vec<&str>>();
    let lines2d = lines
        .iter()
        .map(|lines| lines.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let cmp_lines = lines2d.clone();

    let thing = lines2d.iter().enumerate().map(|(r, line)| {
        for (c, char) in line.iter().enumerate() {
            if *char == '*' {
                let mut numbers = Vec::<usize>::new();
                match get_left(r, c, &lines2d) {
                    Some(number) => numbers.push(number),
                    None => {}
                }
                match get_right(r, c, &lines2d) {
                    Some(number) => numbers.push(number),
                    None => {}
                }
                match get_top(r, c, &lines2d) {
                    Some(number) => numbers.push(number),
                    None => {}
                }

                dbg![&numbers];
            }
        }
        0
    }).collect::<Vec<_>>();

    //find asterisk
    //Check neighbors for any numeric.
    // if numeric found
    //  push current char to current_number_string
    //  replace with period
    // then check left if number,
    // push number to back of current_number_string
    // if perid is found break
    // then check right
    // if number push to front of current_number_string
    // if period found break.
    //if number is found take it and replace it with a period.
    //continue to check left and right until end of number is found.
    //

    dbg!(thing);
    0
}

//
//WORKING ON CHECKING LEFT && RIGHT FUNCTIONS
//THEN WORK ON TOP AND BOTTOM FUNCTION UTILIZING THE CHECK LEFT AND RIGHT FUNCTIONS
//
fn get_left(row: usize, start_col: usize, grid: &Vec<Vec<char>>) -> Option<usize> {
    let mut found_symbol = false;
    let mut found_numbers = VecDeque::<char>::new();
    let mut step = 1;
    while !found_symbol && step < 10000 {
        let position = start_col as i32 - step;
        if position < 0 {
            break;
        }

        let new_char = grid[row][position as usize];

        if new_char.is_numeric() {
            found_numbers.push_back(new_char);
        } else {
            found_symbol = true;
        }
        step += 1;
    }
    let mut result_string = String::new();
    for char in found_numbers.into_iter().rev() {
        result_string.push(char);
    }
    match result_string.parse::<usize>() {
        Ok(number) => Some(number),
        Error => None,
    }
}

fn get_right(row: usize, start_col: usize, grid: &Vec<Vec<char>>) -> Option<usize> {
    let mut found_symbol = false;
    let mut found_numbers = VecDeque::<char>::new();
    let mut step = 1;
    while !found_symbol && step < 10000 {
        let position = start_col + step;
        if position > grid[row].len() {
            break;
        }

        let new_char = grid[row][position as usize];

        if new_char.is_numeric() {
            found_numbers.push_back(new_char);
        } else {
            found_symbol = true;
        }
        step += 1;
    }
    let mut result_string = String::new();
    for char in found_numbers {
        result_string.push(char);
    }
    match result_string.parse::<usize>() {
        Ok(number) => Some(number),
        Error => None,
    }
}


#[test]
fn test_part_2() {
    let content = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..".to_string();
    let result = process(&content);
    assert_eq!(result, 4361);
}
