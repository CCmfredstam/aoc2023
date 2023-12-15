use std::fs::read_to_string;

fn ascii_code(ch: char) -> u8 {
    let mut code: u8 = 0;
    if ch.is_ascii() {
        code = ch as u8;
    }
    code
}

fn run_hashing(seq: String) -> i64 {
    let mut hash_sum: u32 = 0;
    for ch in seq.chars() {
        hash_sum = ((hash_sum + ascii_code(ch) as u32) * 17) % 256;
    }
    hash_sum.into()
}

fn main_part1() {
    // Read todays input
    let data = read_to_string("input/day15.txt").unwrap();
    let _data = read_to_string("input/test_input.txt").unwrap();

    let init_sequence: Vec<String> = data.split(',').map(|s|s.to_string()).collect();

    let mut sum: i64 = 0;
    for seq in init_sequence {
        sum += run_hashing(seq);
    }

    println!("Part1: {}", sum);

}

fn main_part2() {
    // Read todays input
    let _data = read_to_string("input/day15.txt").unwrap();
    let _lines: Vec<String> = _data.split('\n').map(|s| s.to_string()).collect();
    let data = read_to_string("input/test_input.txt").unwrap();
    let lines: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();

    

    let total_sum = 0;
    for _line in lines {
    }

    println!("Part2: {}", total_sum);

}

fn main() {
    main_part1();
    main_part2();
}
