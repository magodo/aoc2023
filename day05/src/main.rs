use std::env;
use std::fs::read_to_string;
use std::iter::Peekable;
use std::str::Lines;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: ./exe <file>");
    }
    let content = read_to_string(&args[1]).unwrap();
    let mut lines = content.lines().peekable();
    let seed_line = lines.next().unwrap();
    let seed_line_numbers: Vec<u64> = seed_line
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut part1_states = seed_line_numbers.clone();
    let mut part2_states: Vec<u64> = Vec::new();
    seed_line_numbers.chunks(2).for_each(|chunk| {
        let start = chunk[0];
        let length = chunk[1];
        part2_states.extend(start..start + length);
    });
    lines.next();

    loop {
        if lines.peek().is_none() {
            break;
        }
        let mapper = build_mapper(&mut lines);
        part1_states = part1_states.into_iter().map(&mapper).collect();
        part2_states = part2_states.into_iter().map(&mapper).collect();
    }

    println!("{}", part1_states.iter().min().unwrap());
    println!("{}", part2_states.iter().min().unwrap());
}

#[derive(Debug, Copy, Clone)]
struct Rule {
    target_start: u64,
    source_start: u64,
    length: u64,
}

impl From<Vec<u64>> for Rule {
    fn from(v: Vec<u64>) -> Self {
        assert!(v.len() == 3);
        Self {
            target_start: v[0],
            source_start: v[1],
            length: v[2],
        }
    }
}

fn build_mapper(lines: &mut Peekable<Lines<'_>>) -> impl Fn(u64) -> u64 {
    lines.next();
    let mut rules: Vec<Rule> = Vec::new();
    loop {
        let line = lines.next();
        match line {
            Some(line) => {
                if line.is_empty() {
                    break;
                }
                let rule = line
                    .split_whitespace()
                    .map(|e| e.parse().unwrap())
                    .collect::<Vec<u64>>()
                    .into();
                rules.push(rule);
            }
            None => break,
        }
    }
    move |input: u64| -> u64 {
        for Rule {
            target_start,
            source_start,
            length,
        } in &rules
        {
            if (*source_start..*source_start + *length).contains(&input) {
                return target_start + (input - *source_start);
            }
        }
        input
    }
}
