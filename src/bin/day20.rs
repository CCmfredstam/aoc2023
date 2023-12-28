use std::{fs::read_to_string, collections::{HashMap, VecDeque}};

const TEST_DATA: bool = false;

fn parse_puzzle_input(lines: &[String]) -> HashMap<String, Module> {
    let mut modules = HashMap::default();
    let mut conjunctions = HashMap::<String, HashMap::<String, Pulse>>::new();

    for line in lines {
        // Fetch all modules
        let (module_name, output_modules) = line.split_once(" -> ").unwrap();
        let output_modules: Vec<String> = output_modules.split(", ").map(|s| s.to_string()).collect();

        if let Some(name) = module_name.strip_prefix("%") {
            // Create flipflop
            modules.insert(name.to_string(), Module { module_type: ModuleType::FlipFlop(Pulse::LOW), outputs: output_modules });
        } else if let Some(name) = module_name.strip_prefix("&") {
            // Create conjunction
            conjunctions.insert(name.to_string(), HashMap::new());
            modules.insert(name.to_string(), Module { module_type: ModuleType::Conjunction(HashMap::new()), outputs: output_modules });
        } else {
            // Create broadcaster
            modules.insert(module_name.to_string(), Module { module_type: ModuleType::Broadcaster, outputs: output_modules });
        }
    }

    // Populate the conjunctions with all modules that have them as output.
    for (module_name, module) in &modules {
        for output in &module.outputs {
            if let Some(conj) = conjunctions.get_mut(output) {
                conj.insert(module_name.clone(), Pulse::LOW);
            }
        }
    }

    for (module_name, module) in &mut modules {
        if let Some(conj) = conjunctions.get(module_name) {
            module.module_type = ModuleType::Conjunction(conj.clone());
        }
    }


    //dbg!(&modules);

    modules
}

fn read_input_data() -> Vec<String> {
    if TEST_DATA {
        let data = read_to_string("input/test_input.txt").unwrap();
        data.split('\n').map(|s| s.to_string()).collect()
    } else {
        let data = read_to_string("input/day20.txt").unwrap();
        data.split('\n').map(|s| s.to_string()).collect()
    }
}

fn main_part1() {
    // Read todays input
    let lines = read_input_data();

    // Parse the input data, i.e., fetch all modules.
    let mut modules = parse_puzzle_input(&lines);

    // Find number of low and high pulses for one button press.
    let mut low_pulse_cnt = 0;
    let mut high_pulse_cnt = 0;
    for _ in 0..1000 {
        let (low_cnt, high_cnt) = send_button_pulse(&mut modules);
        low_pulse_cnt += low_cnt;
        high_pulse_cnt += high_cnt;
    }

    println!("Part1: {}", low_pulse_cnt * high_pulse_cnt);
}

fn main_part2() {
    // Read todays input
    let _lines = read_input_data();

    println!("Part2: {}", 0);
}

fn main() {
    main_part1();
    main_part2();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    HIGH,
    LOW,
}

#[derive(Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop(Pulse),
    Conjunction(HashMap<String, Pulse>),
}

#[derive(Debug)]
struct Module {
    module_type: ModuleType,
    outputs: Vec<String>,
}

fn send_button_pulse(modules: &mut HashMap<String, Module>) -> (i64, i64) {
    let mut low_pulse_counter = 0;
    let mut high_pulse_counter = 0;
    let mut fifo_queue = VecDeque::from_iter([("button".to_string(), "broadcaster".to_string(), Pulse::LOW)]);

    while let Some((sender, receiver, pulse)) = fifo_queue.pop_front() {

        if pulse == Pulse::LOW {
            low_pulse_counter += 1;
        } else if pulse == Pulse::HIGH {
            high_pulse_counter += 1;
        }


        if let Some(Module { module_type, outputs }) = modules.get_mut(&receiver) {
            match module_type {
                ModuleType::Broadcaster => {
                    for out in outputs {
                        fifo_queue.push_back((receiver.clone(), out.clone(), pulse));
                    }
                },
                ModuleType::FlipFlop(p) => {
                    if pulse == Pulse::LOW {

                        *p = match *p {
                            Pulse::LOW => Pulse::HIGH,
                            Pulse::HIGH => Pulse::LOW,
                        };

                        for out in outputs {
                            fifo_queue.push_back((receiver.clone(), out.clone(), *p));
                        }
                    }
                },
                ModuleType::Conjunction(p) => {
                    p.insert(sender.to_string(), pulse);

                    let conj_output = match !p.values().all(|x| *x == Pulse::HIGH) {
                        false => Pulse::LOW,
                        true => Pulse::HIGH,
                    };

                    for out in outputs {
                        fifo_queue.push_back((receiver.clone(), out.clone(), conj_output));
                    }
                },
            }
        }
    }

    (low_pulse_counter, high_pulse_counter)
}