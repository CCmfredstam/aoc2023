use std::fs::read_to_string;

#[derive(Debug)]
struct Group {
    ground: Vec<String>,
}

impl Group {
    fn new(group: Vec<String>) -> Self {
        Self { ground: group }
    }

    fn column_pattern_correct(&self, idx: usize) -> bool {
        for col_idx in idx..self.ground[0].len() {
            for row_idx in 0..self.ground.len() {
                let rev_idx = idx as i64 - 1 - (col_idx as i64 - idx as i64);
                if rev_idx < 0 {
                    return true;
                }
                if self.ground[row_idx].chars().nth(col_idx) != self.ground[row_idx].chars().nth(rev_idx as usize) {
                    return false;
                }
            }
        }

        true
    }

    fn summarize_columns_part1(&self) -> i64 {
        let mut col_value: i64 = 0;

        for col_idx in 1..self.ground[0].len() {
            let mut col_bad = false;
            for row_idx in 0..self.ground.len() {
                if self.ground[row_idx].chars().nth(col_idx) != self.ground[row_idx].chars().nth(col_idx-1) {
                    col_bad = true;
                }
            }
            if !col_bad && self.column_pattern_correct(col_idx) {
                col_value = col_idx as i64;
                break;
            }
        }

        col_value
    }

    fn modify_column(&mut self, idx: usize) {
        for row_idx in 0..self.ground.len() {            
            let _: String = self.ground[row_idx].chars()
                                .enumerate()
                                .map(|(i, c)| {
                                    if i == idx { 
                                        self.ground[row_idx].chars().nth(idx-1).unwrap()
                                    } else {
                                        c
                                    }
                                }).collect();
        }
    }

    fn summarize_columns_part2(&mut self) -> i64 {
        let mut col_value: i64 = 0;

        for col_idx in 1..self.ground[0].len() {
            let mut col_cnt = 0;
            for row_idx in 0..self.ground.len() {
                if self.ground[row_idx].chars().nth(col_idx) != self.ground[row_idx].chars().nth(col_idx-1) {
                    col_cnt += 1;
                }
            }
            if col_cnt == 1 {
                dbg!("One char differs in column");
                dbg!(col_idx);
                self.modify_column(col_idx);
                col_cnt = 0;
            }
            if col_cnt == 0 && self.column_pattern_correct(col_idx) {
                col_value = col_idx as i64;
                break;
            }
        }

        col_value
    }

    fn row_pattern_correct(&self, idx: usize) -> bool {
        for row_idx in idx..self.ground.len() {
            let rev_idx = idx as i64 - 1 - (row_idx as i64 - idx as i64);
            if rev_idx < 0 {
                return true;
            }
            if self.ground[row_idx] != self.ground[rev_idx as usize] {
                return false;
            }
        }

        true
    }

    fn summarize_rows_part1(&self) -> i64 {
        let mut row_value: i64 = 0;

        for row_idx in 1..self.ground.len() {
            if self.ground[row_idx] == self.ground[row_idx-1] && self.row_pattern_correct(row_idx) {
                row_value = row_idx as i64;
                break;
            }
        }

        row_value * 100  // Row value should be multiplied by 100
    }

    fn summarize_rows_part2(&mut self) -> i64 {
        let mut row_value: i64 = 0;

        for row_idx in 1..self.ground.len() {
            if self.ground[row_idx].chars().zip(self.ground[row_idx-1].chars()).filter(|(a, b)| a != b).count() == 1 {
                dbg!("Only one char differ between:");
                dbg!(row_idx);
                dbg!(&self.ground[row_idx]);
                dbg!(&self.ground[row_idx-1]);

                let tmp = self.ground[row_idx-1].clone();
                self.ground[row_idx-1] = self.ground[row_idx].clone();
                if !self.row_pattern_correct(row_idx) {
                    self.ground[row_idx-1] = tmp;
                    self.ground[row_idx] = self.ground[row_idx-1].clone();
                }
                
                dbg!("After modify");
                dbg!(&self.ground[row_idx]);
                dbg!(&self.ground[row_idx-1]);
                
            }
            if self.ground[row_idx] == self.ground[row_idx-1] && self.row_pattern_correct(row_idx) {
                row_value = row_idx as i64;
                break;
            }
        }

        row_value * 100  // Row value should be multiplied by 100
    }

    fn summarize_pattern_part1(&self) -> i64 {
        self.summarize_rows_part1() + self.summarize_columns_part1()
    }

    fn summarize_pattern_part2(&mut self) -> i64 {
        self.summarize_rows_part2() + self.summarize_columns_part2()
    }

}

fn parse_puzzle_input(lines: &Vec<String>) -> Vec<Group> {
    let mut groups: Vec<Group> = vec![];
    let mut current_group: Vec<String> = vec![];

    for line in lines {
        if !line.is_empty() {
            current_group.push(line.clone());
        } else {
            if !current_group.is_empty() {
                groups.push(Group::new(current_group.clone()));
            }
            current_group.clear();
        }
    }

    // Push last group
    if !current_group.is_empty() {
        groups.push(Group::new(current_group.clone()));
    }

    groups
}

fn main_part1() {
    // Read todays input
    let data = read_to_string("input/day13.txt").unwrap();
    let lines: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();
    let _data = read_to_string("input/test_input.txt").unwrap();
    let _lines: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();

    let puzzle_groups: Vec<Group> = parse_puzzle_input(&lines);

    let mut total_sum = 0;
    for group in puzzle_groups {
        total_sum += group.summarize_pattern_part1();
    }

    println!("Part1: {}", total_sum);

}

fn main_part2() {
    // Read todays input
    let _data = read_to_string("input/day13.txt").unwrap();
    let _lines: Vec<String> = _data.split('\n').map(|s| s.to_string()).collect();
    let data = read_to_string("input/test_input.txt").unwrap();
    let lines: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();

    let puzzle_groups: Vec<Group> = parse_puzzle_input(&lines);

    let mut total_sum = 0;
    for mut group in puzzle_groups {
        dbg!(&group);
        total_sum += group.summarize_pattern_part2();
    }

    println!("Part2: {}", total_sum);

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
