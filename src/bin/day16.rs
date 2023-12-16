use std::{fs::read_to_string, collections::{HashMap, HashSet}};

fn parse_puzzle_input(lines: Vec<String>) -> HashMap<(i64, i64), char> {
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

    let contraption: HashMap<(i64, i64), char> = parse_puzzle_input(lines);
    let mut light: Light = Light::new(0, -1, DirectionType::Right);
    
    let energized_tiles = light.beam(&contraption, lines.len() as i64, lines[0].len() as i64);

    println!("Part1: {}", energized_tiles.len());

}

fn main_part2() {
    // Read todays input
    let _data = read_to_string("input/day16.txt").unwrap();
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
    visited: HashSet<(i64, i64)>,
}

impl Light {
    fn new(row: i64, col: i64, dir: DirectionType) -> Self {
        Self {
            direction: dir,
            current_row: row,
            current_column: col,
            visited: Default::default()
        }
    }

    fn calc_energized(&self) -> i64 {
        self.visited.iter().count().try_into().unwrap()
    }

    /// Moves the light forward in its direction
    fn beam(&mut self, contraption: &HashMap<(i64, i64), char>, max_row: i64, max_col: i64) -> HashSet<(i64, i64)> {

        'beam_loop: loop {
            let moving = match self.direction {
                DirectionType::Down => (1, 0),
                DirectionType::Up => (-1, 0),
                DirectionType::Right => (0, 1),
                DirectionType::Left => (0, -1),
            };
    
            // Move in the direction (inside borders)
            self.current_row += moving.0;
            self.current_column += moving.1;
    
            if self.current_row < 0 || self.current_row > max_row ||self.current_column < 0 || self.current_column > max_col {
                break 'beam_loop;
            }
    
            // Save the new position to visited
            self.visited.insert((self.current_row, self.current_column));
    
            // Match current char at position:
            //      Check if direction should swap
            //      Check if splitting light. If yes, spawn new Light at this position with correct direction
            //      If going to straight line, do nothing.
    
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
                        DirectionType::Up | DirectionType::Down => {
                            let light_left = Light::new(self.current_row, self.current_column, DirectionType::Left);
                            let light_right = Light::new(self.current_row, self.current_column, DirectionType::Right);
                            let energized_left = light_left.beam(contraption, max_row, max_col);
                            let energized_right = light_right.beam(contraption, max_row, max_col);
                            self.visited = self.visited.union(&energized_left);
                            self.visited = self.visited.union(&energized_right);
                            break 'beam_loop;  // Self light beam is no longer valid, stop going to next position
                        },
                        DirectionType::Left | DirectionType::Right => {},  // Do nothing, keep moving through
                    }
                }
                '|' => {
                    match self.direction {
                        //DirectionType::Left | DirectionType::Right => todo!("Split into two beams, at current position moving up and down"),
                        DirectionType::Left | DirectionType::Right => {
                            let light_up = Light::new(self.current_row, self.current_column, DirectionType::Up);
                            let light_down = Light::new(self.current_row, self.current_column, DirectionType::Down);
                            let energized_left = light_up.beam(contraption, max_row, max_col);
                            let energized_right = light_down.beam(contraption, max_row, max_col);
                            self.visited = self.visited.union(&energized_up);
                            self.visited = self.visited.union(&energized_down);
                            break 'beam_loop;  // Self light beam is no longer valid, stop going to next position
                        },
                        DirectionType::Up | DirectionType::Down => {},  // Do nothing, keep moving through
                    }
                }
                '.' => {},  // Empty space, do nothing
                _ => panic!("Unexpected character"),
            }
            
        } // beam_loop

        self.visited
    }
}