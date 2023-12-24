use std::{fs::read_to_string, collections::{HashMap, HashSet, VecDeque}, sync::Arc, default};

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


    println!("Part1: {}", farthest_point);

}

fn main_part2() {
    // Read todays input
    let lines = read_input_data(TEST_DATA);

    let (maze, start_position) = parse_puzzle_input(&lines);

    println!("Part2: {}", 0);

}

fn main() {
    main_part1();
    main_part2();
}


fn find_farthest_point(maze: &HashMap<(i64, i64), char>, start_pos: (i64, i64)) -> i64 {
    let mut current_state = get_first_move(maze, start_pos);

    dbg!(&current_state);

    while current_state.position != start_pos {
        current_state.move_along(maze);
    }

    current_state.tile_count / 2
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct State {
    position: (i64, i64),
    direction: Direction,
    tile_count: i64,
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

    if let Some(up) = maze.get(&(x-1, y)) {
        match up {
            '|' => { return State{ position: (x-1,  y), direction: Direction::Up, tile_count: 1 }; },
            '7' => { return State{ position: (x-1,  y), direction: Direction::Left, tile_count: 1  }; },
            'F' => { return State{ position: (x-1,  y), direction: Direction::Right, tile_count: 1  }; },
            _ => {}
        }
    }

    if let Some(down) = maze.get(&(x+1, y)) {
        match down {
            '|' => { return State{ position: (x+1,  y), direction: Direction::Down, tile_count: 1  }; },
            'J' => { return State{ position: (x+1,  y), direction: Direction::Left, tile_count: 1  }; },
            'L' => { return State{ position: (x+1,  y), direction: Direction::Right, tile_count: 1  }; },
            _ => {}
        }
    }

    if let Some(left) = maze.get(&(x, y-1)) {
        match left {
            '-' => { return State{ position: (x,  y-1), direction: Direction::Left, tile_count: 1  }; },
            'L' => { return State{ position: (x,  y-1), direction: Direction::Up, tile_count: 1  }; },
            'F' => { return State{ position: (x,  y-1), direction: Direction::Down, tile_count: 1  }; },
            _ => {}
        }
    }

    if let Some(right) = maze.get(&(x, y+1)) {
        match right {
            '-' => { return State{ position: (x,  y+1), direction: Direction::Right, tile_count: 1  }; },
            'J' => { return State{ position: (x,  y+1), direction: Direction::Up, tile_count: 1  }; },
            '7' => { return State{ position: (x,  y+1), direction: Direction::Down, tile_count: 1  }; },
            _ => {}
        }
    }

    State { position: (0, 0), direction: Direction::None, tile_count: 1 }
}