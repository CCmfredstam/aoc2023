use std::{fs::read_to_string, collections::HashSet};

fn main() {
    // Read todays input
    let data = read_to_string("input/day3.txt").unwrap();
    let lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    // Part 1
    let mut part_num: Option<PartNumber> = None;
    let mut numbers: Vec<PartNumber> = vec![];
    let mut symbols: HashSet<(i64, i64)> = Default::default();
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                // If previous character was a number, increase the value
                if let Some(ref mut pnum) = part_num {
                    pnum.extend_digit(row as i64, col as i64, ch);
                } else { // Create new part number
                    part_num = Some(PartNumber::new(row as i64, col as i64, ch));
                }
            } else {
                if let Some(num) = part_num.take() {
                    numbers.push(num);
                }
                if ch != '.' {
                    symbols.insert((row as i64, col as i64));
                }
            }
        }
    }

    let part1_total: i64 = numbers.iter()
                                  .filter(|n| n.next_to_symbol(&symbols))
                                  .map(|n| n.number)
                                  .sum();
    println!("Part1: {}", part1_total);

}

#[derive(Debug)]
struct PartNumber {
    number: i64,
    points: HashSet<(i64, i64)>
}

impl PartNumber {
    fn new(row: i64, col: i64, ch: char) -> Self {
        let points = HashSet::from([
            (row-1, col-1), (row-1, col), (row-1, col+1),
            (row, col-1), (row, col+1),
            (row+1, col-1), (row+1, col), (row+1, col+1)
        ]);
        Self { number: (ch as u8 - b'0') as i64, points }
    }

    fn extend_digit(&mut self, row: i64, col: i64, ch: char) {
        self.number = self.number * 10 + (ch as u8 - b'0') as i64;
        self.points.extend([(row-1, col+1), (row, col+1), (row+1, col+1)]);
    }

    fn next_to_symbol(&self, symbols: &HashSet<(i64, i64)>) -> bool {
        self.points.intersection(symbols).next().is_some()
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_placeholder() {
    }
}