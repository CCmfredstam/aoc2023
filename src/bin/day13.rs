use std::fs::read_to_string;

#[derive(Debug)]
struct Group {
    ground: Vec<String>,
}

impl Group {
    fn new(group: Vec<String>) -> Self {
        Self { ground: group }
    }

    fn summarize_columns(&self) -> i64 {
        0
    }

    fn summarize_rows(&self) -> i64 {
        0 * 100
    }

    fn summarize_pattern(&self) -> i64 {
        self.summarize_rows() + self.summarize_columns()
    }
}

fn parse_puzzle_input(lines: &Vec<String>) -> Vec<Group> {
    let mut groups: Vec<Group> = vec![];
    let mut current_group: Vec<String> = vec![];

    for line in lines {
        if !line.is_empty() {
            current_group.push(line.clone());
        } else {
            if !current_group.is_empty() {
                groups.push(Group::new(current_group.clone()));
            }
            current_group.clear();
        }
    }

    // Push last group
    if !current_group.is_empty() {
        groups.push(Group::new(current_group.clone()));
    }

    groups
}

fn main_part1() {
    // Read todays input
    let _data = read_to_string("input/day13.txt").unwrap();
    let _lines: Vec<String> = _data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
    let data = read_to_string("input/test_input.txt").unwrap();
    let lines: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();

    let puzzle_groups: Vec<Group> = parse_puzzle_input(&lines);

    let mut total_sum = 0;
    for group in puzzle_groups {
        println!("{group:#?}");
        total_sum = group.summarize_pattern();
    }

    println!("Part1: {}", total_sum);

}

fn main_part2() {
    // Read todays input
    let data = read_to_string("input/day13.txt").unwrap();
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
