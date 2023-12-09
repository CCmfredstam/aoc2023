use std::fs::read_to_string;

fn main_part1() {
    // Read todays input
    let data = read_to_string("input/day6.txt").unwrap();
    let lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    let race_times: Vec<i64> = lines[0].split_whitespace().skip(1).filter_map(|s| s.parse().ok()).collect();
    let best_distances: Vec<i64> = lines[1].split_whitespace().skip(1).filter_map(|s| s.parse().ok()).collect();


    let mut multiplied_wins = 1;
    for (race_time, best_distance) in race_times.iter().zip(best_distances.iter()) {
        let mut total_wins = 0;
        for held_millisec in 0..*race_time {
            let distance_covered = (race_time - held_millisec) * held_millisec;
            if distance_covered > *best_distance {
                total_wins += 1;
            }
        }
        multiplied_wins *= total_wins;
    }

    println!("Part1: {}", multiplied_wins);

}

fn main_part2() {
    // Read todays input
    let data = read_to_string("input/day6.txt").unwrap();
    let lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    let race_times: Vec<String> = lines[0].split_whitespace().skip(1).map(|s| s.to_string()).collect();
    let best_distances: Vec<String> = lines[1].split_whitespace().skip(1).map(|s| s.to_string()).collect();

    let race_time: i64 = race_times.join("").parse().unwrap();
    let best_distance: i64 = best_distances.join("").parse().unwrap();


    let mut total_wins = 0;
    for held_millisec in 0..race_time {
        let distance_covered = (race_time - held_millisec) * held_millisec;
        if distance_covered > best_distance {
            total_wins += 1;
        }
    }

    println!("Part2: {}", total_wins);

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