use std::fs::read_to_string;

fn main() {
    let data = read_to_string("input/day1.txt").unwrap();
    let lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    let mut sum_calibration = 0;

    for line in lines {
        let num_line: String = line.chars().filter(|c| c.is_ascii_digit()).collect();
        let first_digit = num_line.chars().next().unwrap();
        let last_digit = num_line.chars().last().unwrap();
        let calibration = format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap();
        sum_calibration += calibration;
    }

    println!("{}", sum_calibration);
}
