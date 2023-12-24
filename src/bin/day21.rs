use std::{fs::read_to_string, collections::{HashMap, HashSet}};

fn parse_puzzle_input(lines: &[String]) -> (HashMap<(i64, i64), char>, (usize, usize)) {
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

    let garden_plots = calc_garden_plots_1(garden, start_position, remaining_steps);

    println!("Part1: {}", garden_plots);

}

fn main_part2() {
    // Read todays input
    let test_data = true;
    let lines = read_input_data(test_data);

    let (garden, start_position) = parse_puzzle_input(&lines);
    let mut remaining_steps: usize = 26_501_365;
    let garden_size = lines.len();

    if test_data {
        remaining_steps = 50;
    }

    dbg!(remaining_steps);

    let garden_plots = calc_garden_plots_2(garden, start_position, remaining_steps, garden_size);

    println!("Part2: {}", garden_plots);

}

fn main() {
    main_part1();
    main_part2();
}

fn calc_garden_plots_1(garden: HashMap<(i64, i64), char>, start_position: (usize, usize), max_steps: usize) -> usize {
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

fn calc_garden_plots_2(garden: HashMap<(i64, i64), char>,
                       start_position: (usize, usize),
                       max_steps: usize,
                       garden_size: usize) -> usize {
    let mut current_positions: HashSet<(i64, i64)> = HashSet::new();
    current_positions.insert((start_position.0 as i64, start_position.1 as i64));

    // Cache for pre-calculated valid neighbors
    let mut valid_neighbor_cache: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::default();

    let mut current_step = 0;
    while current_step < max_steps {
        // Print every now and then when using large numbers
        if current_step % 100_000 == 0 {
            dbg!(current_step);
        }

        let mut valid_neighbors: Vec<(i64, i64)> = Vec::new();

        // Find all valid neighbors for each current position
        for &position in &current_positions.clone() {
            if let Some(neighbors) = valid_neighbor_cache.get(&position) {
                // dbg!("Found cached neighbors");
                valid_neighbors.extend(neighbors);
            } else {
                let neighb = get_valid_neighbors_2(&position, &garden, garden_size);
                valid_neighbor_cache.insert(position, neighb.clone());
                valid_neighbors.extend(neighb);
            }
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

fn get_valid_neighbors_2(position: &(i64, i64), garden: &HashMap<(i64, i64), char>, garden_size: usize) -> Vec<(i64, i64)>{
    let row: i64 = position.0;
    let col: i64 = position.1;
    let mut valid_neighbors: Vec<(i64, i64)> = Vec::new();

    // North
    if let Some(pos) = neighbor_2(row-1, col, garden, garden_size) {
        valid_neighbors.push(pos);
    }

    // South
    if let Some(pos) = neighbor_2(row+1, col, garden, garden_size) {
        valid_neighbors.push(pos);
    }

    // West
    if let Some(pos) = neighbor_2(row, col-1, garden, garden_size) {
        valid_neighbors.push(pos);
    }

    // East
    if let Some(pos) = neighbor_2(row, col+1, garden, garden_size) {
        valid_neighbors.push(pos);
    }
    
    valid_neighbors
}

fn neighbor(row: i64, col: i64, garden: &HashMap<(i64, i64), char>) -> Option<(usize, usize)> {
    // Calculate wrapping in the garden.
    // Then check in bounds of garden and if rock in the way.



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

fn neighbor_2(row: i64, col: i64, garden: &HashMap<(i64, i64), char>, garden_size: usize) -> Option<(i64, i64)> {
    // Calculate wrapping in the garden.
    // Then check in bounds of garden and if rock in the way.
    let gsize = garden_size as i64;
    let mut r = row;
    let mut c = col;

    // dbg!(r, c);


    if r < 0 {
        r = gsize - (r.abs() % gsize);
    } else if r >= gsize {
        r %= gsize;
    }

    if c < 0 {
        c = gsize - (c.abs() % gsize);
    } else if c >= gsize {
        c %= gsize;
    }


    // -3  -2  -1  0  1  2  3  4  5 6 7 8 9 10

    // if r == -1 {
    //     r = gsize-1;
    // } else if r == gsize {
    //     r = 0;
    // }
    
    // if c == -1 {
    //     c = gsize-1;
    // } else if c == gsize {
    //     c = 0;
    // }

    //dbg!(row, col, r, c);
    if garden.contains_key(&(r, c)) {
        //dbg!("valid range");
        // Check that the position does not contain a rock
        if garden.get(&(r, c)).unwrap() != &'#' {
            //dbg!("not a rock");
            return Some((row, col));
        }
    }


    // Outside bounds of garden or rock in the way
    None
}
