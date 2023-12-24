use std::{fs::read_to_string, collections::HashMap};
use regex::Regex;

const TEST_DATA: bool = false;

fn parse_puzzle_input(lines: &Vec<String>) -> (Workflows, Vec<Part>) {
    let mut workflows= Workflows::new();
    let mut parts = vec![];
    let mut workflow_parsing = true;

    for line in lines {
        if line.is_empty() {
            workflow_parsing = false;
            continue;
        }

        if workflow_parsing {
            let (name, flow) = parse_workflow(&line);
            workflows.push(name, flow);
        } else {
            parts.push(parse_part(&line));
        }
    }

    (workflows, parts)
}

fn parse_workflow(line: &String) -> (String, Workflow) {
    let (workflow_name, workflow_rules) = line.split_once('{').unwrap();
    let workflow_name = workflow_name.to_string();
    let rules = workflow_rules.strip_suffix('}').unwrap();

    let mut workflow_rules = vec![];
    for rule in rules.split(',') {
        if let Some((cond, act)) = rule.split_once(':') {
            let cond = parse_condition(cond);
            let act = parse_action(act);
            workflow_rules.push(Rule { condition: cond, action: act });
        } else {
            let cond = Condition::True;
            let act = parse_action(rule);
            workflow_rules.push(Rule { condition: cond, action: act });
        }
    }

    (workflow_name, Workflow { rules: workflow_rules })
}

fn parse_action(act: &str) -> Action {
    let mut action = Action::Reject;

    if act == "A" {
        action = Action::Accept;
    } else if act == "R" {
        action = Action::Reject;
    } else {
        action = Action::NextWorkflow(act.to_string());
    }

    action
}

fn parse_condition(cond: &str) -> Condition {
    let mut condition = Condition::True;

    if let Some((cat, val)) = cond.split_once('<') {
        condition = Condition::LessThan(cat.to_string(), val.parse().unwrap());
    } else if let Some((cat, val)) = cond.split_once('>') {
        condition = Condition::GreaterThan(cat.to_string(), val.parse().unwrap());
    }

    condition
}

fn parse_part(line: &String) -> Part {

    let part_pattern = r"\{x=(?P<x_val>\d+),m=(?P<m_val>\d+),a=(?P<a_val>\d+),s=(?P<s_val>\d+)\}";
    let re_part = Regex::new(part_pattern).expect("re_part: Invalid regex pattern...");

    let Some(part) = re_part.captures(line) else {panic!("re_part: no match");};

    Part {
        x_value: part["x_val"].parse().unwrap(),
        m_value: part["m_val"].parse().unwrap(),
        a_value: part["a_val"].parse().unwrap(),
        s_value: part["s_val"].parse().unwrap(),
    }
}

fn read_input_data() -> Vec<String> {
    if TEST_DATA {
        let data = read_to_string("input/test_input.txt").unwrap();
        data.split('\n').map(|s| s.to_string()).collect()
    } else {
        let data = read_to_string("input/day19.txt").unwrap();
        data.split('\n').map(|s| s.to_string()).collect()
    }
}

fn main_part1() {
    // Read todays input
    let lines = read_input_data();

    let (workflows, parts) = parse_puzzle_input(&lines);

    let mut total_part_sum: i64 = 0;
    for part in parts {
        if workflows.accept_part(&part) {
            total_part_sum += part.sum();
        }
    }

    println!("Part1: {}", total_part_sum);
}

fn main_part2() {
    // Read todays input
    let lines = read_input_data();

    let x = parse_puzzle_input(&lines);

    println!("Part2: {}", 0);
}

fn main() {
    main_part1();
    main_part2();
}

#[derive(Debug, Clone)]
enum Condition {
    LessThan(String, i64),
    GreaterThan(String, i64),
    True,
}

#[derive(Debug, Clone, PartialEq)]
enum Action {
    Accept,
    Reject,
    NextWorkflow(String),
}

#[derive(Debug, Clone)]
struct Rule {
    condition: Condition,
    action: Action,
}

impl Rule {
    fn condition_ok(&self, part: &Part) -> bool {
        if let Condition::LessThan(cat, value) = &self.condition {
            match cat.as_str() {
                "x" => { return part.x_value < *value; },
                "m" => { return part.m_value < *value; },
                "a" => { return part.a_value < *value; },
                "s" => { return part.s_value < *value; },
                _ => panic!("Bad category! ONLY xmas is ok!")
            }
        } else if let Condition::GreaterThan(cat, value) = &self.condition {
            match cat.as_str() {
                "x" => { return part.x_value > *value; },
                "m" => { return part.m_value > *value; },
                "a" => { return part.a_value > *value; },
                "s" => { return part.s_value > *value; },
                _ => panic!("Bad category! ONLY xmas is ok!")
            }
        } else {
            return true;
        }

        false
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn next_action(&self, part: &Part) -> Action {
        for rule in &self.rules {
            if rule.condition_ok(part) {
                return rule.action.clone();
            }
        }
        Action::Reject
    }
}

#[derive(Debug)]
struct Workflows{
    workflows: HashMap<String, Workflow>,
}

impl Workflows {
    fn new() -> Self {
        Self { workflows: HashMap::default() }
    }

    fn push(&mut self, name: String, workflow: Workflow) {
        self.workflows.insert(name, workflow);
    }

    fn accept_part(&self, part: &Part) -> bool {
        let mut current_workflow = "in".to_string();

        while let Some(workflow) = self.workflows.get(&current_workflow) {
            let next_action = workflow.next_action(part);
            if next_action == Action::Accept {
                return true;
            } else if next_action == Action::Reject {
                return false;
            } else if let Action::NextWorkflow(wf) = next_action {
                current_workflow = wf;
            }
        }

        false
    }
}

#[derive(Debug)]
struct Part {
    x_value: i64,
    m_value: i64,
    a_value: i64,
    s_value: i64,
}

impl Part {
    fn sum(&self) -> i64 {
        self.x_value + self.m_value + self.a_value + self.s_value
    }
}