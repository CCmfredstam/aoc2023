use std::{fs::read_to_string, collections::HashSet};

use regex::Regex;

const TEST_DATA: bool = false;

fn parse_puzzle_input(_lines: &[String]) -> Vec<DigInstruction> {
    let mut input: Vec<DigInstruction> = Vec::new();

    
    let mut input_as_string = include_str!("../../input/day18.txt");  
    if TEST_DATA {
        input_as_string = include_str!("../../input/test_input.txt");
    }
    
    let dig_instr_pattern = r"(?P<dir>[UDLR]) (?P<met>\d+) \(\#(?P<rgb>[0-9a-fA-F]+)\)";
    let re_dig_instr = Regex::new(dig_instr_pattern).expect("dig_instr_pattern: Invalid regex pattern...");

    for (_, [dir, met, rgb]) in re_dig_instr.captures_iter(input_as_string).map(|c| c.extract()) {
        
        let dig_dir = match dir {
            "U" => DirectionType::Up,
            "D" => DirectionType::Down,
            "L" => DirectionType::Left,
            "R" => DirectionType::Right,
            _ => todo!(),
        };

        let rgb_code = u32::from_str_radix(rgb, 16).unwrap();

        input.push(DigInstruction {
            direction: dig_dir,
            meters_to_dig: met.parse().unwrap(),
            _rgb_code: RGBCode {
                _code: rgb_code,
            },
        });
    }

    input
}

fn read_input_data(test_data: bool) -> Vec<String> {
    if test_data {
        let data = read_to_string("input/test_input.txt").unwrap();
        data.split('\n').map(|s| s.to_string()).collect()
    } else {
        let data = read_to_string("input/day18.txt").unwrap();
        data.split('\n').map(|s| s.to_string()).collect()        
    }
}

fn main_part1() {
    // Read todays input
    let lines = read_input_data(TEST_DATA);
    let dig_instructions = parse_puzzle_input(&lines);

    let total_dig = dig(&dig_instructions);

    println!("Part1: {}", total_dig);

}

fn main_part2() {
    // Read todays input
    let lines = read_input_data(TEST_DATA);
    let _x = parse_puzzle_input(&lines);

    println!("Part2: {}", 0);

}

fn main() {
    main_part1();
    main_part2();
}

#[derive(Debug)]
enum DirectionType {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct RGBCode {
    _code: u32,
}

#[derive(Debug)]
struct DigInstruction {
    direction: DirectionType,
    meters_to_dig: i64,
    _rgb_code: RGBCode,
}

fn dig(instructions: &Vec<DigInstruction>) -> usize {

    let digged_positions = dig_edges_according_to_instruction(instructions);

    println!("Before extra digging:");
    print_dig_site(&digged_positions);

    let digged_positions = dig_between_edges(&digged_positions);

    println!();
    println!("After extra digging:");
    print_dig_site(&digged_positions);

    digged_positions.len()
}

fn dig_between_edges(dig_site: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    
    let mut digging_site: HashSet<(i64, i64)> = dig_site.clone();
    let (minx, _maxx, _miny, maxy) = find_bounds(dig_site);

    // Flood fill
    let start_fill = find_next_dig(&digging_site, minx, 0, maxy);
    let Some(start_fill) = start_fill else {panic!("no start place on row 0")};

    let mut fill_stack = vec![(minx+1, start_fill+1)];

    while let Some(pos) = fill_stack.pop() {
        if !digging_site.insert(pos) {
            continue;
        }
        fill_stack.push((pos.0-1, pos.1));
        fill_stack.push((pos.0+1, pos.1));
        fill_stack.push((pos.0, pos.1-1));
        fill_stack.push((pos.0, pos.1+1));
    } 

    digging_site
}

fn find_next_dig(dig_site: &HashSet<(i64, i64)>, row: i64, col: i64, max_col: i64) -> Option<i64> {
    (col..=max_col).find(|&c| dig_site.contains(&(row, c)))
}


fn dig_edges_according_to_instruction(instructions: &Vec<DigInstruction>) -> HashSet<(i64, i64)> {
    let mut digged_positions: HashSet<(i64, i64)> = HashSet::default();

    let mut current_position = (0, 0);
    digged_positions.insert(current_position);


    // Dig according to instructions. This will dig the edges.
    for instruction in instructions {
        match instruction.direction {
            DirectionType::Up => {
                for _ in 0..instruction.meters_to_dig {
                    current_position.0 -= 1;
                    digged_positions.insert(current_position);
                }
            },
            DirectionType::Down => {
                for _ in 0..instruction.meters_to_dig {
                    current_position.0 += 1;
                    digged_positions.insert(current_position);
                }
            },
            DirectionType::Left => {
                for _ in 0..instruction.meters_to_dig {
                    current_position.1 -= 1;
                    digged_positions.insert(current_position);
                }
            },
            DirectionType::Right => {
                for _ in 0..instruction.meters_to_dig {
                    current_position.1 += 1;
                    digged_positions.insert(current_position);
                }
            },
        }
    }

    digged_positions
}


fn print_dig_site(dig_site: &HashSet<(i64, i64)>) {
    let (minx, maxx, miny, maxy) = find_bounds(dig_site);

    for i in minx..=maxx {
        for j in miny..=maxy {
            if dig_site.contains(&(i,j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

// Helper function to find the minimum and maximum x and y values in the hash set
fn find_bounds(hash_set: &HashSet<(i64, i64)>) -> (i64, i64, i64, i64) {
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    for &(x, y) in hash_set.iter() {
        if x < min_x { min_x = x; }
        if x > max_x { max_x = x; }
        if y < min_y { min_y = y; }
        if y > max_y { max_y = y; }
    }

    (min_x, max_x, min_y, max_y)
}