pub fn process(p0: &String) -> usize {
    0
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
    location.is_numeric()
}

fn check_top_middle(r: usize, c: usize, lines: &Vec<&str>) -> bool {
    let location: char = lines[r - 1].chars().collect::<Vec<char>>()[c];
    location.is_numeric()
}

fn check_top_right(r: usize, c: usize, lines: &Vec<&str>) -> bool {
    let location: char = lines[r - 1].chars().collect::<Vec<char>>()[c + 1];
    location.is_numeric()
}

fn check_middle_left(r: usize, c: usize, lines: &Vec<&str>) -> bool {
    let location: char = lines[r].chars().collect::<Vec<char>>()[c - 1];
    location.is_numeric()
}

fn check_middle_right(r: usize, c: usize, lines: &Vec<&str>) -> bool {
    let location: char = lines[r].chars().collect::<Vec<char>>()[c + 1];
    location.is_numeric()
}

fn check_bottom_left(r: usize, c: usize, lines: &Vec<&str>) -> bool {
    let location: char = lines[r + 1].chars().collect::<Vec<char>>()[c - 1];
    location.is_numeric()
}

fn check_bottom_middle(r: usize, c: usize, lines: &Vec<&str>) -> bool {
    let location: char = lines[r + 1].chars().collect::<Vec<char>>()[c];
    location.is_numeric()
}

fn check_bottom_right(r: usize, c: usize, lines: &Vec<&str>) -> bool {
    let location: char = lines[r + 1].chars().collect::<Vec<char>>()[c + 1];
    location.is_numeric()
}


#[test]
fn test_part_2() {
    let content = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..".to_string();
    let result = process(&content);
    assert_eq!(result, 4361);
}
