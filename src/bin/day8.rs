use std::{fs::read_to_string, collections::HashMap};

use regex::Regex;

#[derive(Debug)]
struct LeftRightDirection {
    left: String,
    right: String,
}

impl LeftRightDirection {
    fn fetch_direction(&self, instruction: char) -> String {
        if instruction == 'L' {
            self.left.clone()
        } else if instruction == 'R' {
            self.right.clone()
        } else {
            "?".to_string()
        }
    }
}

fn parse_network_nodes(lines: Vec<String>) -> HashMap<String, LeftRightDirection> {
    let mut network: HashMap<String, LeftRightDirection> = HashMap::new();

    let re_network = Regex::new(r"(?<pos>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();
    for line in lines {
        if let Some(cap) = re_network.captures(&line) {
            let pos = cap.name("pos").map_or("", |s|s.as_str()).to_string();
            let left = cap.name("left").map_or("", |s|s.as_str()).to_string();
            let right = cap.name("right").map_or("", |s|s.as_str()).to_string();
            network.insert(pos, LeftRightDirection { left, right });
        }
    }

    network
}

fn main_part1() {
    // Read todays input
    let data = read_to_string("input/day8.txt").unwrap();
    let lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    let _lines: Vec<String> = vec!["RL".to_string(),
                                  "AAA = (BBB, CCC)".to_string(),
                                  "BBB = (DDD, EEE)".to_string(),
                                  "CCC = (ZZZ, GGG)".to_string(),
                                  "DDD = (DDD, DDD)".to_string(),
                                  "EEE = (EEE, EEE)".to_string(),
                                  "GGG = (GGG, GGG)".to_string(),
                                  "ZZZ = (ZZZ, ZZZ)".to_string(),
                                ];

    let _lines: Vec<String> = vec!["LLR".to_string(),
                                  "AAA = (BBB, BBB)".to_string(),
                                  "BBB = (AAA, ZZZ)".to_string(),
                                  "ZZZ = (ZZZ, ZZZ)".to_string(),
                                ];

    let (left_right_instruction, network_nodes) = lines.split_at(1);
    let network: HashMap<String, LeftRightDirection> = parse_network_nodes(network_nodes.to_vec());

    let mut looking: bool = true;
    let mut current_position = "AAA".to_string();
    let mut total_steps = 0;
    while looking {
        for instruction in left_right_instruction.concat().chars() {
            if current_position == "ZZZ" {
                looking = false;
                break;
            }
            if let Some(pos) = network.get(&current_position) {
                current_position = pos.fetch_direction(instruction);
            }
            total_steps += 1;
        }
    }

    println!("Part1: {}", total_steps);

}

fn main_part2() {
    // Read todays input
    let data = read_to_string("input/day8.txt").unwrap();
    let _lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    println!("Part2: {}", 0);

}

fn main() {
    main_part1();
    main_part2();
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_placeholder() {
    }
}