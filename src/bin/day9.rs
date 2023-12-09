use std::fs::read_to_string;


fn extrapolate(datapoints: Vec<i64>) -> i64 {
    let mut extrapolation = *datapoints.last().unwrap();

    let mut diffs: Vec<i64> = datapoints.windows(2).map(|x| x[1] - x[0]).collect();

    extrapolation += diffs.last().unwrap();

    loop {
        diffs = diffs.windows(2).map(|x| x[1] - x[0]).collect();
        if diffs.iter().all(|x| *x == 0) {
            return extrapolation;
        }
        extrapolation += diffs.last().unwrap();
    }

}

fn main_part1() {
    // Read todays input
    let data = read_to_string("input/day9.txt").unwrap();
    let lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
    let _lines: Vec<String> = vec!["0 3 6 9 12 15".to_string(),
                                  "1 3 6 10 15 21".to_string(),
                                  "10 13 16 21 30 45".to_string(),
                                ];

    let mut total_extrapolation: i64 = 0;
    for line in lines {
        let datapoints: Vec<i64> = line.split_whitespace().map(|s|s.parse().unwrap()).collect();
        let next_point = extrapolate(datapoints);
        total_extrapolation += next_point;
    }

    println!("Part1: {}", total_extrapolation);

}

fn main_part2() {
    // Read todays input
    let data = read_to_string("input/day9.txt").unwrap();
    let lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    let mut total_extrapolation: i64 = 0;
    for line in lines {
        let datapoints: Vec<i64> = line.split_whitespace().map(|s|s.parse().unwrap()).rev().collect();
        let next_point = extrapolate(datapoints);
        total_extrapolation += next_point;
    }

    println!("Part2: {}", total_extrapolation);

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
