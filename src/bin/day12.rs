use std::fs::read_to_string;

fn variant_accepted(variant: &String, format: &Vec<i64>) -> bool {
    // Split variant on dots "."
    let damaged_record_groups: Vec<String> = variant.split('.')
                                                    .map(|s|s.to_string())
                                                    .filter(|s|s.contains('#'))
                                                    .collect();

    // If we don't have the same number of groups of continuous broken springs, skip.
    if damaged_record_groups.len() != format.len() {
        return false;
    }

    // For each "#" group zipped with format, does the length equal the format number?
    for (group, format_n) in damaged_record_groups.iter().zip(format.iter()) {
        //println!("Group x{group:#?}x -> formatN: {format_n:#?}");
        if group.len() != *format_n as usize {
            return false;
        }
    }

    true
}

fn all_record_combinations(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    //let mut memo: HashMap<String, Vec<String>> = HashMap::new();

    //combinations_recursive(input, &mut current, &mut result, &mut memo);
    combinations_recursive(input, &mut current, &mut result);

    result
}

fn combinations_recursive(input: &str, current: &mut String, result: &mut Vec<String>) {
    if let Some(ch) = input.chars().next() {
        if ch == '?' {
            current.push('.');
            combinations_recursive(&input[1..], current, result);

            current.pop();
            current.push('#');
            combinations_recursive(&input[1..], current, result);

            current.pop(); // backtrack for further iterations
        } else {
            current.push(ch);
            combinations_recursive(&input[1..], current, result);
            current.pop(); // backtrack for further iterations
        }
    } else {
        result.push(current.clone());
    }
}

fn find_arragements(record: String) -> i64 {
    let parts: Vec<&str> = record.split_whitespace().collect::<Vec<&str>>();
    let damaged_records = parts[0];
    let other_format = parts[1].split(',').map(|s|s.parse().unwrap()).collect::<Vec<i64>>();

    let mut count = 0;
    // Brute force that sh....
    for variant in all_record_combinations(damaged_records) {
        if variant_accepted(&variant, &other_format) {
            count += 1;
        }
    }

    count
}

fn main_part1() {
    // Read todays input
    let data = read_to_string("input/day12.txt").unwrap();
    let lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
    let _data = read_to_string("input/test_input.txt").unwrap();
    let _lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    let mut total_arrangements = 0;
    for record in lines {
        total_arrangements += find_arragements(record);
    }

    println!("Part1: {}", total_arrangements);

}

fn main_part2() {
    // Read todays input
    let data = read_to_string("input/day12.txt").unwrap();
    let lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
    
    let mut total_arrangements = 0;
    for record in lines {
        total_arrangements += find_arragements(record);
    }

    println!("Part2: {}", total_arrangements);

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
