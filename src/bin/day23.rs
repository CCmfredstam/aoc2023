use std::{fs::read_to_string, collections::{HashMap, HashSet, VecDeque}};

const TEST_DATA: bool = false;

fn parse_puzzle_input(lines: &[String]) -> (HashMap<(i64, i64), char>, (i64, i64)) {
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
        let data = read_to_string("input/day23.txt").unwrap();
        data.split('\n').map(|s| s.to_string()).collect()
    }
}

fn main_part1() {
    // Read todays input
    let lines = read_input_data(TEST_DATA);

    let (forest, start_position) = parse_puzzle_input(&lines);

    let longest_walk = find_longest_walk_dfs(forest, start_position);


    println!("Part1: {}", longest_walk);

}

fn main_part2() {
    // Read todays input
    let lines = read_input_data(TEST_DATA);

    let (forest, start_position) = parse_puzzle_input(&lines);

    let longest_walk = find_longest_walk_dfs(forest, start_position);

    println!("Part2: {}", longest_walk);
    // Correct 6526 after bruteforcing for 10h

}

fn main() {
    //main_part1();
    main_part2();
}


fn find_longest_walk_dfs(forest: HashMap<(i64, i64), char>, start_pos: (i64, i64)) -> usize {
    let visited = HashSet::new();
    let mut stack = VecDeque::new();
    let mut longest_walk: usize = 0;

    // Push the start node onto the stack
    stack.push_back((start_pos, start_pos, longest_walk, visited));

    'walking: while let Some((node, prev_node, length, mut seen)) = stack.pop_back() {
            if length >= 20_000 {
                dbg!("20.000 position");
                dbg!(node);
                continue 'walking;
            }
            if !seen.insert(node) {
                continue 'walking;
            }

            if let Some(pos) = forest.get(&node) {
                if *pos == 'E' {
                    let prev_longest = longest_walk;
                    longest_walk = longest_walk.max(length);
                    if prev_longest < longest_walk {
                        dbg!("New longest walk:");
                        dbg!(&longest_walk);
                    }
                    continue 'walking;
                }
            }

            //println!("visiting: {:#?}", node);
            //seen.insert(node);

            // Get the neighbors of the current node
            let neighbors = get_neighbors(node, prev_node, &forest);

            // Push unvisited neighbors onto the stack
            for &neighbor in &neighbors {
                if !seen.contains(&neighbor) {
                    stack.push_back((neighbor, node, length+1, seen.clone()));
                }
            }
    }

    longest_walk
}

fn get_neighbors(node: (i64, i64), prev_node: (i64, i64), forest: &HashMap<(i64, i64), char>) -> Vec<(i64, i64)> {
    let (x, y) = node;
    let neighbors = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
    let mut valid_neighbors = vec![];

    // If we find a slope, we can only go in that direction.
    let slope = false;
    // if let Some(current) = forest.get(&node) {
    //     match *current {
    //         '>' => {
    //             valid_neighbors.push((x, y + 1));
    //             slope = true;
    //         },
    //         'v' => {
    //             valid_neighbors.push((x + 1, y));
    //             slope = true;
    //         },
    //         '<' => {
    //             valid_neighbors.push((x, y - 1));
    //             slope = true;
    //         },
    //         '^' => {
    //             valid_neighbors.push((x - 1, y));
    //             slope = true;
    //         },
    //         _ => {},
    //     }
    // }

    // If no slope, check all paths that are not trees
    if !slope {
        for neighbor in neighbors {
            if neighbor == prev_node {
                continue;
            }
            if let Some(ch) = forest.get(&neighbor) {
                if *ch != '#' {
                    valid_neighbors.push(neighbor);
                }
            }
        }
    }

    valid_neighbors
}