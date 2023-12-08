pub fn process(content: &String) -> usize {
    let lines = content.split("\n").collect::<Vec<&str>>();
    let cmp_lines = lines.clone();

    let lines2d = lines
        .iter()
        .map(|lines| lines.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let strings_of_numbers_near_symbols = lines2d
        .iter()
        .enumerate()
        .filter_map(|(r, line)| {
            let mut numbers_on_this_line = Vec::<String>::new();
            let mut string_of_number_chars = "".to_string();
            let mut near_symbol = false;
            for (c, char) in line.iter().enumerate() {
                if char.is_numeric() {
                    if !near_symbol && check_neighbors(r, c, &cmp_lines) {
                        near_symbol = true
                    }
                    string_of_number_chars.push(char.clone());
                    if c == line.len() - 1 && near_symbol{
                        numbers_on_this_line.push(string_of_number_chars.clone());
                        string_of_number_chars = "".to_string();
                        near_symbol = false;
                    }
                } else {
                    if string_of_number_chars.len() == 0 || !near_symbol {
                        string_of_number_chars = "".to_string();
                    } else {
                        numbers_on_this_line.push(string_of_number_chars.clone());
                        string_of_number_chars = "".to_string();
                        near_symbol = false;
                    }
                }
            }
            if numbers_on_this_line.len() == 0 {
                None
            } else {
                Some(numbers_on_this_line)
            }
        }).flatten().collect::<Vec<_>>();



    let numbers_near_symbols = strings_of_numbers_near_symbols.iter()
        .map(|string| string.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    numbers_near_symbols.iter().sum()
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
        check_middle_left(r, c, lines)
            || check_middle_right(r, c, lines)
            || check_bottom_left(r, c, lines)
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

#[test]
fn test_part_1() {
    let content = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n.......755\n....$.*...\n..664.598.".to_string();
    let result = process(&content);
    dbg!(&result);
    assert_eq!(result, 4361);
}