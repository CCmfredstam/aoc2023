use std::fs::read_to_string;

fn main() {
    // Read todays input
    let data = read_to_string("input/day6.txt").unwrap();
    let lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    // Part 1
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




    // for each race time
    //     for each held_millisec in race_time
    //         potential_race_distance = calc_distance(race_time, held_millisec)
    //         if potetntil_race_distance > best_distances
    //             winning_race += 1;


    // fn calc_distance(race_time, held_millisec) {
    //     time_left = race_time - held_millisec
    //     return time_left * held_millisec
    // }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_placeholder() {
    }
}