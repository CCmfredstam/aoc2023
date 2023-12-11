use std::fs::read_to_string;

#[derive(Debug)]
struct Galaxy {
    name: String,
    pos_row: i64,
    pox_col: i64, 
}

impl Galaxy {
    fn find_shortest_path(&self, other: &Galaxy) -> i64 {
        if std::ptr::eq(self, other) {
            return 0;
        }
        (self.pos_row - other.pos_row).abs() + (self.pox_col - other.pox_col).abs()
    }
}

/// Expands the universe. When no row or column has any galaxies, add another row/column.
fn expand_universe(universe: &mut Vec<Vec<char>>) {
    // Find rows that contain only '.'
    let mut rows_to_insert: Vec<usize> = universe
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&c| c == '.'))
        .map(|(i, _)| i)
        .collect();

    // Find columns that contain only '.'
    let mut columns_to_insert: Vec<usize> = (0..universe[0].len())
        .filter(|&col| universe.iter().all(|row| row[col] == '.'))
        .collect();

    for (offset, value) in rows_to_insert.iter_mut().enumerate() {
        *value += offset;
    }
    
    for (offset, value) in columns_to_insert.iter_mut().enumerate() {
        *value += offset;
    }

    // Insert new rows
    for row_idx in rows_to_insert {
        universe.insert(row_idx, vec!['.'; universe[0].len()]);
    }

    // Insert new columns
    for col_idx in columns_to_insert {
        for row in &mut *universe {
            row.insert(col_idx, '.');
        }
    }
}

/// Find the location of all galaxies in the universe
fn find_galaxies(lines: Vec<Vec<char>>) -> Vec<Galaxy> {
    let mut galaxies: Vec<Galaxy> = vec![];

    for (row, rows) in lines.iter().enumerate(){
        for (col, symbol) in rows.iter().enumerate() {
            if *symbol == '#' {
                galaxies.push(Galaxy { name: format!("{row}{col}"), pos_row: row as i64, pox_col: col as i64 } );
            }
        }
    }

    galaxies
}

/// Calculates the shortests paths between every pair of galaxies
fn calculate_shortest_paths(galaxies: Vec<Galaxy>) -> Vec<i64> {
    let mut shortest_paths: Vec<i64> = Vec::new();

    for galaxy_i_idx in 0..galaxies.len() {
        for galaxy_j_idx in galaxy_i_idx+1..galaxies.len() {
            let shortest_path = galaxies[galaxy_i_idx].find_shortest_path(&galaxies[galaxy_j_idx]);
            shortest_paths.push(shortest_path);
        }
    }

    shortest_paths
}

fn _compare_example_expansion(lines: &Vec<Vec<char>>) {
    let expected_uni: Vec<Vec<char>> = vec![
                                        "....#........".chars().collect(),
                                        ".........#...".chars().collect(),
                                        "#............".chars().collect(),
                                        ".............".chars().collect(),
                                        ".............".chars().collect(),
                                        "........#....".chars().collect(),
                                        ".#...........".chars().collect(),
                                        "............#".chars().collect(),
                                        ".............".chars().collect(),
                                        ".............".chars().collect(),
                                        ".........#...".chars().collect(),
                                        "#....#.......".chars().collect(),
                                    ];

    for idx in 0..lines.len() {
        for jdx in 0..lines[0].len() {
            if lines[idx][jdx] != expected_uni[idx][jdx] {
                panic!("Expanded universe does not match!");
            }
        }
    }
}

fn _print_universe(lines: &Vec<Vec<char>>) {
    for i in lines {
        for j in i {
            print!("{}", j);
        }
        println!();
    }
}

fn main_part1() {
    // Read todays input
    let data = read_to_string("input/day11.txt").unwrap();
    let mut lines: Vec<Vec<char>> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.chars().collect()).collect();
    let mut _lines: Vec<Vec<char>> = vec!["...#......".chars().collect(),
                                         ".......#..".chars().collect(),
                                         "#.........".chars().collect(),
                                         "..........".chars().collect(),
                                         "......#...".chars().collect(),
                                         ".#........".chars().collect(),
                                         ".........#".chars().collect(),
                                         "..........".chars().collect(),
                                         ".......#..".chars().collect(),
                                         "#...#.....".chars().collect(),
                                    ];

    expand_universe(&mut lines);

    //_compare_example_expansion(&lines);

    // Find location to all galaxies
    let galaxies: Vec<Galaxy> = find_galaxies(lines);
    
    // Find shortest length between every pair of galaxies
    let galaxies_length: Vec<i64> = calculate_shortest_paths(galaxies);

    // Calculate the sum of all lengths
    let total_length: i64 = galaxies_length.iter().sum();
    println!("Part1: {}", total_length);

}

fn main_part2() {
    // Read todays input
    let data = read_to_string("input/day10.txt").unwrap();
    let _lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    println!("Part2: {}", 0);

}

fn main() {
    main_part1();
    main_part2();
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_placeholder() {
    }
}
