use std::{fs::read_to_string, collections::{HashMap, HashSet}};

fn parse_puzzle_input(lines: &Vec<String>) -> HashMap<(i64, i64), char> {
    let mut input: HashMap<(i64, i64), char> = HashMap::default();

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            input.insert((row as i64, col as i64), ch);
        }
    }

    input
}

fn main_part1() {
    // Read todays input
    let _data = read_to_string("input/day16.txt").unwrap();
    let _lines: Vec<String> = _data.split('\n').map(|s| s.to_string()).collect();
    let data = read_to_string("input/test_input.txt").unwrap();
    let lines: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();

    let contraption: HashMap<(i64, i64), char> = parse_puzzle_input(&lines);
    let mut light: Light = Light::new(0, -1, DirectionType::Right);

    let mut energized_tiles: HashSet<(i64, i64)> = Default::default();
    light.beam(&contraption, lines.len() as i64, lines[0].len() as i64, &mut energized_tiles);
    
    println!("Part1: {}", energized_tiles.len());

}

fn main_part2() {
    // Read todays input
    let data = read_to_string("input/day16.txt").unwrap();
    let lines: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();
    let _data = read_to_string("input/test_input.txt").unwrap();
    let _lines: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();

    let contraption: HashMap<(i64, i64), char> = parse_puzzle_input(&lines);
    let mut light: Light = Light::new(-1, 3, DirectionType::Down);

    let mut energized_tiles: HashSet<(i64, i64)> = Default::default();
    light.beam(&contraption, lines.len() as i64, lines[0].len() as i64, &mut energized_tiles);

    // Create all starting points
    let mut lights: Vec<Light> = vec![];
    for i in 0..lines.len() {
        lights.push(Light::new(i as i64, -1, DirectionType::Right));
        lights.push(Light::new(i as i64, lines[0].len() as i64, DirectionType::Left));
    }
    for i in 0..lines[0].len() {
        lights.push(Light::new(-1, i as i64, DirectionType::Down));
        lights.push(Light::new(lines.len() as i64, i as i64, DirectionType::Up));
    }

    // Find highest energized tiles
    let mut best_energized: Vec<i64> = vec![];
    for mut light in lights {
        let mut energized_tiles: HashSet<(i64, i64)> = Default::default();
        light.beam(&contraption, lines.len() as i64, lines[0].len() as i64, &mut energized_tiles);
        best_energized.push(energized_tiles.len() as i64);
    }

    best_energized.sort();
    println!("Part2: {}", best_energized.last().unwrap());

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

struct Light {
    direction: DirectionType,
    current_row: i64,
    current_column: i64,
}

impl Light {
    fn new(row: i64, col: i64, dir: DirectionType) -> Self {
        Self {
            direction: dir,
            current_row: row,
            current_column: col,
        }
    }

    fn direction_move(&self) -> (i64, i64) {
        match self.direction {
            DirectionType::Down => (1, 0),
            DirectionType::Up => (-1, 0),
            DirectionType::Right => (0, 1),
            DirectionType::Left => (0, -1),
        }
    }

    /// Moves the light forward in its direction
    fn beam(&mut self, contraption: &HashMap<(i64, i64), char>, max_row: i64, max_col: i64, visited: &mut HashSet<(i64, i64)>) {

        'beam_loop: loop {
            let moving = self.direction_move();
    
            // Move in the direction (inside borders)
            self.current_row += moving.0;
            self.current_column += moving.1;
    
            // Check within bound of two dimentional array
            if self.current_row < 0 || self.current_row >= max_row || self.current_column < 0 || self.current_column >= max_col {
                break 'beam_loop;
            }

            // Unwarp OK since we made sure we're inside bounds above
            match contraption.get(&(self.current_row, self.current_column)).unwrap() {
                '/' => {
                    match self.direction {
                        DirectionType::Up => self.direction = DirectionType::Right,
                        DirectionType::Down => self.direction = DirectionType::Left,
                        DirectionType::Right => self.direction = DirectionType::Up,
                        DirectionType::Left => self.direction = DirectionType::Down,
                    }
                }
                '\\' => {
                    match self.direction {
                        DirectionType::Up => self.direction = DirectionType::Left,
                        DirectionType::Down => self.direction = DirectionType::Right,
                        DirectionType::Right => self.direction = DirectionType::Down,
                        DirectionType::Left => self.direction = DirectionType::Up,
                    }
                }
                '-' => {
                    match self.direction {
                        DirectionType::Up | DirectionType::Down => { // Split beam
                            if visited.contains(&(self.current_row, self.current_column)) {
                                break 'beam_loop;
                            }
                            // Save the new position to visited
                            visited.insert((self.current_row, self.current_column));
                            let mut light_left = Light::new(self.current_row, self.current_column, DirectionType::Left);
                            let mut light_right = Light::new(self.current_row, self.current_column, DirectionType::Right);
                            light_left.beam(contraption, max_row, max_col, visited);
                            light_right.beam(contraption, max_row, max_col, visited);
                            break 'beam_loop;  // Self light beam is no longer valid, stop going to next position
                        },
                        DirectionType::Left | DirectionType::Right => {},  // Do nothing, keep moving through
                    }
                }
                '|' => {
                    match self.direction {
                        DirectionType::Left | DirectionType::Right => { // Split beam
                            if visited.contains(&(self.current_row, self.current_column)) {
                                break 'beam_loop;
                            }
                            // Save the new position to visited
                            visited.insert((self.current_row, self.current_column));
                            let mut light_up = Light::new(self.current_row, self.current_column, DirectionType::Up);
                            let mut light_down = Light::new(self.current_row, self.current_column, DirectionType::Down);
                            light_up.beam(contraption, max_row, max_col, visited);
                            light_down.beam(contraption, max_row, max_col, visited);
                            break 'beam_loop;  // Self light beam is no longer valid, stop going to next position
                        },
                        DirectionType::Up | DirectionType::Down => {},  // Do nothing, keep moving through
                    }
                }
                '.' => {},  // Empty space, do nothing
                _ => panic!("Unexpected character"),
            }

            // Save the new position to visited
            visited.insert((self.current_row, self.current_column));
            
        } // beam_loop
    }
}