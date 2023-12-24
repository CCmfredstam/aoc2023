use std::{fs::read_to_string, collections::{HashMap, BinaryHeap}, cmp::Ordering};

fn parse_puzzle_input(lines: &[String]) -> Vec<Vec<Edge>> {
    let mut input: Vec<Vec<Edge>> = vec![];

    let mut tmp_grid: HashMap<(i64, i64), i64> = Default::default();

    for (row, line) in lines.iter().enumerate() {
        input.push(vec![]);
        for (col, ch) in line.chars().enumerate() {
            let val: i64 = ch.to_digit(10).expect("Unexpected input character, expected a digit") as i64;
            tmp_grid.insert((row as i64, col as i64), val);
        }
    }

    // Create node network/grid
    for (row, line) in lines.iter().enumerate() {
        for (col, _ch) in line.chars().enumerate() {
            input.push(vec![]);
            let line_len = line.len();
            let curr_idx = row * line_len + col;

            // Push above
            if tmp_grid.contains_key(&(row as i64 - 1, col as i64)) {
                let key_value = tmp_grid.get_key_value(&(row as i64 - 1, col as i64)).unwrap();
                let node_nr = *key_value.0;
                let node_nr = ((node_nr.0 * line_len as i64) + (node_nr.1)) as usize;
                let val = *key_value.1 as usize;
                input[curr_idx].push(Edge { node: node_nr, cost: val });
            }

            // Push right
            if tmp_grid.contains_key(&(row as i64, col as i64 + 1)) {
                let key_value = tmp_grid.get_key_value(&(row as i64, col as i64 + 1)).unwrap();
                let node_nr = *key_value.0;
                let node_nr = ((node_nr.0 * line_len as i64) + (node_nr.1)) as usize;
                let val = *key_value.1 as usize;
                input[curr_idx].push(Edge { node: node_nr, cost: val });
            }

            // Push below
            if tmp_grid.contains_key(&(row as i64 + 1, col as i64)) {
                let key_value = tmp_grid.get_key_value(&(row as i64 + 1, col as i64)).unwrap();
                let node_nr = *key_value.0;
                let node_nr = ((node_nr.0 * line_len as i64) + (node_nr.1)) as usize;
                let val = *key_value.1 as usize;
                input[curr_idx].push(Edge { node: node_nr, cost: val });
            }

            // Push left
            if tmp_grid.contains_key(&(row as i64, col as i64 - 1)) {
                let key_value = tmp_grid.get_key_value(&(row as i64, col as i64 - 1)).unwrap();
                let node_nr = *key_value.0;
                let node_nr = ((node_nr.0 * line_len as i64) + (node_nr.1)) as usize;
                let val = *key_value.1 as usize;
                input[curr_idx].push(Edge { node: node_nr, cost: val });
            }
        }
    }

    input
}

fn read_input_data(test_data: bool) -> Vec<String> {
    if test_data {
        let data = read_to_string("input/test_input.txt").unwrap();
        data.split('\n').map(|s| s.to_string()).collect()
    } else {
        let data = read_to_string("input/day17.txt").unwrap();
        data.split('\n').map(|s| s.to_string()).collect()        
    }
}

fn main_part1() {
    // Read todays input
    let lines = read_input_data(true);

    let cities: Vec<Vec<Edge>> = parse_puzzle_input(&lines);

    //println!("{cities:#?}");

    let start_node: usize = 0;
    let goal_node: usize = lines.len() * lines[0].len() - 1;
    let max_consecutive_moves: usize = 3;

    dbg!(start_node, goal_node);

    let heat_loss = shortest_path(&cities, start_node, goal_node, max_consecutive_moves, lines.len());
    
    println!("Part1: {}", heat_loss.unwrap());
    println!("Expecting 902 (test 102)");

}

fn main_part2() {
    // Read todays input
    let data = read_to_string("input/day17.txt").unwrap();
    let _lines: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();
    let _data = read_to_string("input/test_input.txt").unwrap();
    let _lines: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();

    println!("Part2: {}", 0);

}

fn main() {
    main_part1();
    main_part2();
}



// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize, max_consecutive_moves: usize, grid_side_len: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State { cost: 0, position: start, consecutive_moves: 0, prev_position: 0 });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position, consecutive_moves, prev_position }) = heap.pop() {
        dbg!(position, prev_position);
        // Alternatively we could have continued to find all shortest paths
        if position == goal { return Some(cost); }

        // Important as we may have already found a better way
        if cost > dist[position] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {

            let next = {
                let moves = calc_consecutive_moves(position, edge.node, prev_position, consecutive_moves, grid_side_len);
                if moves == usize::MAX { continue; }
                State { cost: cost + edge.cost, position: edge.node, consecutive_moves: moves, prev_position: position }
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] && next.consecutive_moves < max_consecutive_moves {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn calc_consecutive_moves(position: usize, next_position: usize, prev_position: usize, consecutive_moves: usize, grid_side_len: usize) -> usize {
    let pos = position as i64;
    let next_pos = next_position as i64;
    let prev_pos = prev_position as i64;
    let side = grid_side_len as i64;

    if pos == 0 && prev_pos == pos {
        return 1;
    }

    let mut prev_direction = DirectionType::None;
    if pos - prev_pos == 1 {
        prev_direction = DirectionType::Right;
    } else if pos - prev_pos == -1 {
        prev_direction = DirectionType::Left;
    } else if pos - prev_pos == side {
        prev_direction = DirectionType::Down;
    } else if pos - prev_pos == -side {
        prev_direction = DirectionType::Up;
    }

    let mut next_direction = DirectionType::None;
    if pos - next_pos == 1 {
        next_direction = DirectionType::Left;
    } else if pos - next_pos == -1 {
        next_direction = DirectionType::Right;
    } else if pos - next_pos == side {
        next_direction = DirectionType::Up;
    } else if pos - next_pos == -side {
        next_direction = DirectionType::Down;
    }

    // Can't move backwards
    match (prev_direction, next_direction) {
        (DirectionType::Down, DirectionType::Up) |
        (DirectionType::Up, DirectionType::Down) |
        (DirectionType::Left, DirectionType::Right) |
        (DirectionType::Right, DirectionType::Left) => { return usize::MAX; }
        _ => {}
    }

    let mut moves = 1;
    if prev_direction == next_direction {
        moves = consecutive_moves + 1;
    }

    moves
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum DirectionType {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
    consecutive_moves: usize,
    prev_position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}