use core::panic;
use std::{fs::read_to_string, collections::HashMap};

fn parse_puzzle_input(lines: &Vec<String>) -> Map {
    let mut map = Map { rows: lines.len() as i64, columns: lines[0].len() as i64, ..Default::default() };

    for (row, line) in lines.iter().enumerate() {
        for (col, sym) in line.chars().enumerate() {
            match sym {
                'O' => {
                    map.rocks.insert((row as i64, col as i64), RockType::Round);
                }
                '#' => {
                    map.rocks.insert((row as i64, col as i64), RockType::Square);
                }
                '.' => {},
                _ => panic!("Unexpected char"),
            }
        }
    }

    map
}

fn tilt_north(map: &Map) -> Map {
    let mut north_tilted_map = Map { rows: map.rows, columns: map.columns, ..Default::default() };

    for row in 0..map.rows {
        for col in 0..map.columns {
            match map.rocks.get(&(row, col)) {
                Some(RockType::Square) => {
                    north_tilted_map.rocks.insert((row, col), RockType::Square);
                }
                Some(RockType::Round) => {
                    let mut current_row = row;
                    while current_row > 0 && !north_tilted_map.rocks.contains_key(&(current_row - 1, col)) {
                        current_row -= 1;
                    }
                    north_tilted_map.rocks.insert((current_row, col), RockType::Round);
                }
                None => {}
            }
        }
    }

    north_tilted_map
}

fn main_part1() {
    // Read todays input
    let data = read_to_string("input/day14.txt").unwrap();
    let lines: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();
    let _data = read_to_string("input/test_input.txt").unwrap();
    let _lines: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();

    let map = parse_puzzle_input(&lines);

    let tilted_map = tilt_north(&map);

    println!("Part1: {}", tilted_map.calc_load());

}

fn main_part2() {
    // Read todays input
    let _data = read_to_string("input/day14.txt").unwrap();
    let _lines: Vec<String> = _data.split('\n').map(|s| s.to_string()).collect();
    let data = read_to_string("input/test_input.txt").unwrap();
    let lines: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();

    

    let total_sum = 0;
    for _line in lines {
    }

    println!("Part2: {}", total_sum);

}

fn main() {
    main_part1();
    main_part2();
}

#[derive(Debug, PartialEq)]
enum RockType {
    Round,
    Square
}

#[derive(Debug)]
#[derive(Default)]
struct Map {
    rocks: HashMap<(i64, i64), RockType>,
    rows: i64,
    columns: i64
}

impl Map {
    fn _print(&self) {
        for row in 0..self.rows {
            for col in 0..self.columns {
                print!("{}",
                    match self.rocks.get(&(row, col)) {
                        None => '.',
                        Some(RockType::Round) => 'O',
                        Some(RockType::Square) => '#',
                    }
                );
            }
            println!();
        }
    }

    fn calc_load(&self) -> i64 {
        self.rocks
            .iter()
            .filter_map(|(k, v)| {
                if *v == RockType::Round {
                    Some(self.rows - k.0)
                } else {
                    None
                }
            })
            .sum::<i64>()
    }
}