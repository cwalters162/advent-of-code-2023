pub fn process(content: &String) -> usize {
    let lines = content.split("\n").collect::<Vec<&str>>();
    let lines2d = lines
        .iter()
        .map(|lines| lines.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let cmp_lines = lines2d.clone();

    let thing = lines2d.iter().enumerate().map(|(r, line)| {
        for (c, char) in line.iter().enumerate() {
            if char == '*' {
                if get_left(r, c, cmp_lines) == "" {

                }
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
fn get_left(start_row: usize, start_col: usize, grid: Vec<Vec<char>>) -> &'static str {
    let mut current_char = '0';
    let mut found_numbers = "".to_string();
    let mut step = 0;
    while current_char.is_numeric() {

    }
    ""
}


#[test]
fn test_part_2() {
    let content = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..".to_string();
    let result = process(&content);
    assert_eq!(result, 4361);
}
