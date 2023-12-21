use std::{fs::read_to_string, collections::{HashMap, HashSet}};

fn parse_puzzle_input(lines: &Vec<String>) -> (HashMap<(i64, i64), char>, (usize, usize)) {
    let mut input: HashMap<(i64, i64), char> = Default::default();
    let mut start_position: (usize, usize) = (0, 0);

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start_position = (row, col);
            }
            input.insert((row as i64, col as i64), ch);
        }
    }

    (input, start_position)
}

fn read_input_data(test_data: bool) -> Vec<String> {
    if test_data {
        let data = read_to_string("input/test_input.txt").unwrap();
        data.split('\n').map(|s| s.to_string()).collect()
    } else {
        let data = read_to_string("input/day21.txt").unwrap();
        data.split('\n').map(|s| s.to_string()).collect()        
    }
}

fn main_part1() {
    // Read todays input
    let lines = read_input_data(false);

    let (garden, start_position) = parse_puzzle_input(&lines);
    let remaining_steps: usize = 64;

    let garden_plots = calc_garden_plots(garden, start_position, remaining_steps);

    println!("Part1: {}", garden_plots);

}

fn main_part2() {
    // Read todays input
    let lines = read_input_data(true);

    println!("Part2: {}", 0);

}

fn main() {
    main_part1();
    main_part2();
}

fn calc_garden_plots(garden: HashMap<(i64, i64), char>, start_position: (usize, usize), max_steps: usize) -> usize {
    let mut current_positions: HashSet<(usize, usize)> = HashSet::new();
    current_positions.insert(start_position);

    let mut current_step = 0;
    while current_step < max_steps {
        let mut valid_neighbors: Vec<(usize, usize)> = Vec::new();

        // Find all valid neighbors for each current position
        for &position in &current_positions.clone() {
            valid_neighbors.extend(get_valid_neighbors(&position, &garden));
        }

        // Remove plots we've just stepped out of
        current_positions.clear();

        // Add neighboring plots
        current_positions.extend(valid_neighbors.iter());

        current_step += 1;
    }

    current_positions.len()
}

fn get_valid_neighbors(position: &(usize, usize), garden: &HashMap<(i64, i64), char>) -> Vec<(usize, usize)>{
    let row: i64 = position.0 as i64;
    let col: i64 = position.1 as i64;
    let mut valid_neighbors: Vec<(usize, usize)> = Vec::new();

    // North
    if let Some(pos) = neighbor(row-1, col, garden) {
        valid_neighbors.push(pos);
    }

    // South
    if let Some(pos) = neighbor(row+1, col, garden) {
        valid_neighbors.push(pos);
    }

    // West
    if let Some(pos) = neighbor(row, col-1, garden) {
        valid_neighbors.push(pos);
    }

    // East
    if let Some(pos) = neighbor(row, col+1, garden) {
        valid_neighbors.push(pos);
    }
    
    valid_neighbors
}

fn neighbor(row: i64, col: i64, garden: &HashMap<(i64, i64), char>) -> Option<(usize, usize)> {
    // Check that we're inbounds of garden
    if garden.contains_key(&(row, col)) {
        // Check that the position does not contain a rock
        if garden.get(&(row, col)).unwrap() != &'#' {
            return Some((row as usize, col as usize));
        }
    }

    // Outside bounds of garden or rock in the way
    None
}
