use std::fs::read_to_string;

fn main() {
    println!("Hello, day1!");
    let data = read_to_string("input/day1.txt").unwrap();
    let lines: Vec<String> = data.split("\n").filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
    print!("String print: ");
    println!("{}", data);
    print!("Lines print: ");
    for line in lines {
        println!("{}", line)
    }
}
