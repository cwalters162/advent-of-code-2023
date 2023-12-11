use std::{env, fs};
use std::thread::current;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let removed_returns = lines.iter().map(|line| {
        line.replace("\r", "")
    }).collect::<Vec<_>>();
    let mut remove_empty_lines = removed_returns.iter().filter_map(|line| {
        if line.len() == 0 {
            None
        } else {
            Some(line)
        }
    }).collect::<Vec<_>>();

    let instructions = remove_empty_lines.remove(0);

    let nodes = remove_empty_lines.iter().map(|line| {
        let mut id = String::new();
        &line[0..3].clone_into(&mut id);
        let mut left = String::new();
        &line[7..10].clone_into(&mut left);
        let mut right = String::new();
        &line[12..15].clone_into(&mut right);
        Node {
            id,
            left,
            right
        }

    }).collect::<Vec<Node>>();

    // let part1result = part1(instructions, &nodes);
    let part2result = part2(instructions, &nodes);

    // println!("Part 1: {}", part1result);
    println!("Part 2: {}", part2result);
}

fn part1(instructions: &String, nodes: &Vec<Node>) -> i32 {

    let mut current_instructions = instructions.clone().chars().rev().collect::<Vec<char>>();
    let mut current_node = nodes.iter().find(|node| node.id == "AAA").expect("Unable to find node 'AAA'");
    let mut steps = 0;
    loop {
        if current_node.id == "ZZZ" {
            return steps;
        }
        steps += 1;
        if current_instructions.len() == 0 {
            current_instructions = instructions.clone().chars().rev().collect::<Vec<char>>();
        }
        let ins = current_instructions.pop().expect("Should have an instruction");

        match ins {
            'L' => current_node = nodes.iter().find(|node| node.id == current_node.left).expect(&*format!("Unable to find node: {}", current_node.left)),
            'R' => current_node = nodes.iter().find(|node| node.id == current_node.right).expect(&*format!("Unable to find node: {}", current_node.right)),
            _ => {panic!("Unknown instruction: {}", ins)}
        }
    }
}

fn part2(instructions: &String, nodes: &Vec<Node>) -> i32 {
    let mut steps = 0;
    let nodes_ending_in_a = nodes.iter().filter_map(|node| if node.id.clone().into_boxed_str().ends_with('A') { Some(node) } else { None }).collect::<Vec<_>>();
    let mut looking = true;

    for (i, &node) in nodes_ending_in_a.iter().enumerate() {
        println!("Nodes completed: {}/{}", &i, nodes_ending_in_a.len());
        let mut current_node = node;
        let mut current_instructions = instructions.clone().chars().rev().collect::<Vec<char>>();

        while looking == true {
            println!("Step: {}", steps);
            if current_node.id.clone().into_boxed_str().ends_with('Z') {
                looking = false;
            }
            steps += 1;
            if current_instructions.len() == 0 {
                current_instructions = instructions.clone().chars().rev().collect::<Vec<char>>();
            }
            let ins = current_instructions.pop().expect("Should have an instruction");

            match ins {
                'L' => current_node = nodes.iter().find(|node| node.id == current_node.left).expect(&*format!("Unable to find node: {}", current_node.left)),
                'R' => current_node = nodes.iter().find(|node| node.id == current_node.right).expect(&*format!("Unable to find node: {}", current_node.right)),
                _ => {panic!("Unknown instruction: {}", ins)}
            }
        }
    }
    steps
}

#[derive(Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}