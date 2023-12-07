use std::{fs::read_to_string, collections::HashSet};

fn main() {
    // Read todays input
    let data = read_to_string("input/day4.txt").unwrap();
    let lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    // Part 1
    let mut total_wins: i64 = 0;
    for line in lines {
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning_numbers, choosen_numbers) = numbers.split_once(" | ").unwrap();

        let winning_numbers: HashSet<i64> = winning_numbers.split_whitespace()
                                                           .map(|d| d.parse::<i64>().unwrap())
                                                           .collect();

        let choosen_numbers: HashSet<i64> = choosen_numbers.split_whitespace()
                                                           .map(|d| d.parse::<i64>().unwrap())
                                                           .collect();

        let count = winning_numbers.intersection(&choosen_numbers).count();
        if count > 0 {
            total_wins += 1 << (count - 1);
        }
    }
    
    println!("{}", total_wins);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_placeholder() {
    }
}
