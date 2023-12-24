use std::{fs::read_to_string, collections::{HashMap, HashSet}};

const TEST_DATA: bool = false;

fn parse_puzzle_input(lines: &Vec<String>) -> (HashMap<(i64, i64), char>, (i64, i64)) {
    let mut input: HashMap<(i64, i64), char> = Default::default();
    let mut start_position: (i64, i64) = (0, 0);

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start_position = (row as i64, col as i64);
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
        let data = read_to_string("input/day10.txt").unwrap();
        data.split('\n').map(|s| s.to_string()).collect()
    }
}

fn main_part1() {
    // Read todays input
    let lines = read_input_data(TEST_DATA);

    let (maze, start_position) = parse_puzzle_input(&lines);

    let farthest_point = find_farthest_point(&maze, start_position);


    println!("Part1: {}", farthest_point.0);

}

fn main_part2() {
    // Read todays input
    let lines = read_input_data(TEST_DATA);

    let (maze, start_position) = parse_puzzle_input(&lines);

    let farthest_point = find_farthest_point(&maze, start_position);

    println!("Part2: {}", farthest_point.1);

}

fn main() {
    //main_part1();
    main_part2();
}


fn find_farthest_point(maze: &HashMap<(i64, i64), char>, start_pos: (i64, i64)) -> (i64, i64) {
    let mut current_state = get_first_move(maze, start_pos);

    while current_state.position != start_pos {
        current_state.move_along(maze);
    }

    dbg!(current_state.visited.len() / 2);

    let outside_visited = find_outside_loop(&current_state.visited, maze);

    print_visited(&current_state.visited, &maze, &outside_visited);

    (current_state.tile_count / 2, maze.len() as i64 - current_state.visited.len() as i64 - outside_visited.len() as i64)
}

fn find_outside_loop(visited: &HashSet<(i64, i64)>, maze: &HashMap<(i64, i64), char>) -> HashSet<(i64, i64)> {

    let (minx, maxx, miny, maxy) = find_bounds(&maze.keys().cloned().collect());
    let mut outside_nodes: HashSet<(i64, i64)> = HashSet::default();
    let mut inside_loop = false;
    let mut stop_insertion = false;
    let mut curr_tile = ' ';

    for row in minx..=maxx {
        for col in miny..=maxy {
            if visited.contains(&(row, col)) {
                match maze.get(&(row, col)).unwrap() {
                    '|' => { inside_loop = !inside_loop; },
                    'F' | 'L' => { curr_tile = *maze.get(&(row, col)).unwrap(); },
                    '7' => {
                        if curr_tile == 'L' {
                            inside_loop = !inside_loop;
                        }
                    },
                    'J' => {
                        if curr_tile == 'F' {
                            inside_loop = !inside_loop;
                        }
                    },
                    _ => {},
                }
            } else if !inside_loop && !stop_insertion {
                outside_nodes.insert((row, col));
            }
        }
        inside_loop = false;
        stop_insertion = false;
    }

    dbg!(outside_nodes.len());

    outside_nodes
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

fn print_visited(visited: &HashSet<(i64, i64)>, maze: &HashMap<(i64, i64), char>, outside_visited: &HashSet<(i64, i64)>) {
    let (minx, maxx, miny, maxy) = find_bounds(&maze.keys().cloned().collect());

    println!();
    for row in minx..=maxx {
        for col in miny..=maxy {
            if maze.contains_key(&(row, col)) {
                if visited.contains(&(row, col)) {
                    print!("{}", maze.get(&(row, col)).unwrap());
                } else
                if outside_visited.contains(&(row, col)) {
                    print!("o");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
    println!();
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Clone, PartialEq, Debug)]
struct State {
    position: (i64, i64),
    direction: Direction,
    tile_count: i64,
    visited: HashSet<(i64, i64)>,
}

impl State {
    fn move_along(&mut self, maze: &HashMap<(i64, i64), char>) {
        // Make move in direction
        match self.direction {
            Direction::Up => self.position.0 -= 1,
            Direction::Down => self.position.0 += 1,
            Direction::Left => self.position.1 -= 1,
            Direction::Right => self.position.1 += 1,
            _ => {},
        }

        self.visited.insert(self.position);

        self.tile_count += 1;

        // Change direction if needed depending on current position
        self.change_direction(maze);
    }

    fn change_direction(&mut self, maze: &HashMap<(i64, i64), char>) {
        let mut tile = '^';
        if let Some(t) = maze.get(&self.position) {
            tile = *t;
        } else {
            return;  // Could not find tile in grid
        }

        match (self.direction, tile) {
            (Direction::Up, '7') => self.direction = Direction::Left,
            (Direction::Up, 'F') => self.direction = Direction::Right,
            (Direction::Down, 'J') => self.direction = Direction::Left,
            (Direction::Down, 'L') => self.direction = Direction::Right,
            (Direction::Left, 'L') => self.direction = Direction::Up,
            (Direction::Left, 'F') => self.direction = Direction::Down,
            (Direction::Right, '7') => self.direction = Direction::Down,
            (Direction::Right, 'J') => self.direction = Direction::Up,
            _ => {},
        }
    }
}

fn get_first_move(maze: &HashMap<(i64, i64), char>, start_pos: (i64, i64)) -> State {
    let (x, y) = start_pos;
    let mut visit: HashSet<(i64, i64)> = HashSet::default();
    visit.insert(start_pos);

    if let Some(up) = maze.get(&(x-1, y)) {
        match up {
            '|' => {
                visit.insert((x-1,  y));
                return State{ position: (x-1,  y), direction: Direction::Up, tile_count: 1, visited: visit };
            },
            '7' => {
                visit.insert((x-1,  y));
                return State{ position: (x-1,  y), direction: Direction::Left, tile_count: 1, visited: visit };
            },
            'F' => {
                visit.insert((x-1,  y));
                return State{ position: (x-1,  y), direction: Direction::Right, tile_count: 1, visited: visit };
            },
            _ => {}
        }
    }

    if let Some(down) = maze.get(&(x+1, y)) {
        match down {
            '|' => {
                visit.insert((x+1,  y));
                return State{ position: (x+1,  y), direction: Direction::Down, tile_count: 1, visited: visit };
            },
            'J' => {
                visit.insert((x+1,  y));
                return State{ position: (x+1,  y), direction: Direction::Left, tile_count: 1, visited: visit };
            },
            'L' => {
                visit.insert((x+1,  y));
                return State{ position: (x+1,  y), direction: Direction::Right, tile_count: 1, visited: visit };
            },
            _ => {}
        }
    }

    if let Some(left) = maze.get(&(x, y-1)) {
        match left {
            '-' => {
                visit.insert((x,  y-1));
                return State{ position: (x,  y-1), direction: Direction::Left, tile_count: 1, visited: visit };
            },
            'L' => {
                visit.insert((x,  y-1));
                return State{ position: (x,  y-1), direction: Direction::Up, tile_count: 1, visited: visit };
            },
            'F' => {
                visit.insert((x,  y-1));
                return State{ position: (x,  y-1), direction: Direction::Down, tile_count: 1, visited: visit };
            },
            _ => {}
        }
    }

    if let Some(right) = maze.get(&(x, y+1)) {
        match right {
            '-' => {
                visit.insert((x,  y+1));
                return State{ position: (x,  y+1), direction: Direction::Right, tile_count: 1, visited: visit };
            },
            'J' => {
                visit.insert((x,  y+1));
                return State{ position: (x,  y+1), direction: Direction::Up, tile_count: 1, visited: visit };
            },
            '7' => {
                visit.insert((x,  y+1));
                return State{ position: (x,  y+1), direction: Direction::Down, tile_count: 1, visited: visit };
            },
            _ => {}
        }
    }

    State { position: (0, 0), direction: Direction::None, tile_count: 1, visited: HashSet::default() }
}