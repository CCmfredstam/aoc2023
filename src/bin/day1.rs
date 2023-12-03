use std::fs::read_to_string;

fn main() {
    let data = read_to_string("input/day1.txt").unwrap();
    let lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();


    // Part 1
    let mut sum_calibration_part1 = 0;

    for line in &lines {
        sum_calibration_part1 += get_calibration(line);
    }

    println!("Part 1: {}", sum_calibration_part1);

    // Part 2
    let mut sum_calibration_part2 = 0;

    for line in &lines {
        let digit_line = convert_string_digit_to_digit(line);
        println!("{} -> {}", line, digit_line);
        sum_calibration_part2 += get_calibration(&digit_line);
    }

    println!("Part 2: {}", sum_calibration_part2);
}

fn get_calibration(line: &String) -> i32 {
    let num_line: String = line.chars().filter(|c| c.is_ascii_digit()).collect();
    let first_digit = num_line.chars().next().unwrap();
    let last_digit = num_line.chars().last().unwrap();
    let calibration = format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap();
    calibration
}

fn convert_string_digit_to_digit(line: &String) -> String {
    let mut converted_line = String::new();
    let mut word_buffer = String::new();

    for c in line.chars() {
        if c.is_alphabetic() {
            word_buffer.push(c);
        } else {
            if !word_buffer.is_empty() {
                converted_line.extend(word_buffer.chars());
                word_buffer.clear();
            }
            converted_line.push(c);
        }
        if let Some(digit) = word_to_digit(&word_buffer) {
            converted_line.push(digit);
            word_buffer.clear();
        }
    }

    converted_line
}

fn word_to_digit(word: &String) -> Option<char> {
    match word.as_str() {
        "zero" => Some('0'),
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_word_to_digit() {
        assert_eq!(word_to_digit(&"zero".to_string()).unwrap(), '0');
        assert_eq!(word_to_digit(&"one".to_string()).unwrap(), '1');
        assert_eq!(word_to_digit(&"two".to_string()).unwrap(), '2');
        assert_eq!(word_to_digit(&"three".to_string()).unwrap(), '3');
        assert_eq!(word_to_digit(&"four".to_string()).unwrap(), '4');
        assert_eq!(word_to_digit(&"five".to_string()).unwrap(), '5');
        assert_eq!(word_to_digit(&"six".to_string()).unwrap(), '6');
        assert_eq!(word_to_digit(&"seven".to_string()).unwrap(), '7');
        assert_eq!(word_to_digit(&"eight".to_string()).unwrap(), '8');
        assert_eq!(word_to_digit(&"nine".to_string()).unwrap(), '9');
        assert_eq!(word_to_digit(&"xxxnine".to_string()), None);
    }

    #[test]
    fn test_convert_digits_at_beginning() {
        assert_eq!(convert_string_digit_to_digit(&"zero67".to_string()), "067".to_string());
        assert_eq!(convert_string_digit_to_digit(&"one78".to_string()), "178".to_string());
        assert_eq!(convert_string_digit_to_digit(&"two89".to_string()), "289".to_string());
        assert_eq!(convert_string_digit_to_digit(&"three90".to_string()), "390".to_string());
        assert_eq!(convert_string_digit_to_digit(&"four01".to_string()), "401".to_string());
        assert_eq!(convert_string_digit_to_digit(&"five12".to_string()), "512".to_string());
        assert_eq!(convert_string_digit_to_digit(&"six23".to_string()), "623".to_string());
        assert_eq!(convert_string_digit_to_digit(&"seven34".to_string()), "734".to_string());
        assert_eq!(convert_string_digit_to_digit(&"eight45".to_string()), "845".to_string());
        assert_eq!(convert_string_digit_to_digit(&"nine56".to_string()), "956".to_string());
        assert_eq!(convert_string_digit_to_digit(&"nin56".to_string()), "nin56".to_string());
    }

    #[test]
    fn test_convert_digits_at_end() {
        assert_eq!(convert_string_digit_to_digit(&"67zero".to_string()), "670".to_string());
        assert_eq!(convert_string_digit_to_digit(&"78one".to_string()), "781".to_string());
        assert_eq!(convert_string_digit_to_digit(&"89two".to_string()), "892".to_string());
        assert_eq!(convert_string_digit_to_digit(&"90three".to_string()), "903".to_string());
        assert_eq!(convert_string_digit_to_digit(&"01four".to_string()), "014".to_string());
        assert_eq!(convert_string_digit_to_digit(&"12five".to_string()), "125".to_string());
        assert_eq!(convert_string_digit_to_digit(&"23six".to_string()), "236".to_string());
        assert_eq!(convert_string_digit_to_digit(&"34seven".to_string()), "347".to_string());
        assert_eq!(convert_string_digit_to_digit(&"45eight".to_string()), "458".to_string());
        assert_eq!(convert_string_digit_to_digit(&"56nine".to_string()), "569".to_string());
    }

    #[test]
    fn test_convert_digits_at_middle() {
        assert_eq!(convert_string_digit_to_digit(&"6zero7".to_string()), "607".to_string());
        assert_eq!(convert_string_digit_to_digit(&"7one8".to_string()), "718".to_string());
        assert_eq!(convert_string_digit_to_digit(&"8two9".to_string()), "829".to_string());
        assert_eq!(convert_string_digit_to_digit(&"9three0".to_string()), "930".to_string());
        assert_eq!(convert_string_digit_to_digit(&"0four1".to_string()), "041".to_string());
        assert_eq!(convert_string_digit_to_digit(&"1five2".to_string()), "152".to_string());
        assert_eq!(convert_string_digit_to_digit(&"2six3".to_string()), "263".to_string());
        assert_eq!(convert_string_digit_to_digit(&"3seven4".to_string()), "374".to_string());
        assert_eq!(convert_string_digit_to_digit(&"4eight5".to_string()), "485".to_string());
        assert_eq!(convert_string_digit_to_digit(&"5nine6".to_string()), "596".to_string());
    }

    #[test]
    fn test_convert_digits_chars_after() {
        assert_eq!(convert_string_digit_to_digit(&"zeroa67".to_string()), "0a67".to_string());
        assert_eq!(convert_string_digit_to_digit(&"oneb78".to_string()), "1b78".to_string());
        assert_eq!(convert_string_digit_to_digit(&"twoc89".to_string()), "2c89".to_string());
        assert_eq!(convert_string_digit_to_digit(&"threed90".to_string()), "3d90".to_string());
        assert_eq!(convert_string_digit_to_digit(&"foure01".to_string()), "4e01".to_string());
        assert_eq!(convert_string_digit_to_digit(&"fivef12".to_string()), "5f12".to_string());
        assert_eq!(convert_string_digit_to_digit(&"sixg23".to_string()), "6g23".to_string());
        assert_eq!(convert_string_digit_to_digit(&"sevenh34".to_string()), "7h34".to_string());
        assert_eq!(convert_string_digit_to_digit(&"eighti45".to_string()), "8i45".to_string());
        assert_eq!(convert_string_digit_to_digit(&"ninej56".to_string()), "9j56".to_string());
    }

    #[test]
    fn test_convert_digits_chars_before() {
        assert_eq!(convert_string_digit_to_digit(&"azero67".to_string()), "a067".to_string());
        // assert_eq!(convert_string_digit_to_digit(&"oneb78".to_string()), "1b78".to_string());
        // assert_eq!(convert_string_digit_to_digit(&"twoc89".to_string()), "2c89".to_string());
        // assert_eq!(convert_string_digit_to_digit(&"threed90".to_string()), "3d90".to_string());
        // assert_eq!(convert_string_digit_to_digit(&"foure01".to_string()), "4e01".to_string());
        // assert_eq!(convert_string_digit_to_digit(&"fivef12".to_string()), "5f12".to_string());
        // assert_eq!(convert_string_digit_to_digit(&"sixg23".to_string()), "6g23".to_string());
        // assert_eq!(convert_string_digit_to_digit(&"sevenh34".to_string()), "7h34".to_string());
        // assert_eq!(convert_string_digit_to_digit(&"eighti45".to_string()), "8i45".to_string());
        // assert_eq!(convert_string_digit_to_digit(&"ninej56".to_string()), "9j56".to_string());
    }
}