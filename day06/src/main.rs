use std::{env, fs::read_to_string};

#[derive(Debug)]
struct Input {
    time: u64,
    distance: u64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>")
    }
    part1(&args[1]);
    part2(&args[1]);
}

fn part1(path: &String) {
    let content = read_to_string(path).unwrap();
    let input = parse_input1(content);

    let mut sum = 1;
    input.into_iter().for_each(|input| {
        let mut cnt = 0;
        (1..input.time).for_each(|t| {
            if (input.time - t) * t > input.distance {
                cnt += 1;
            }
        });
        sum *= cnt;
    });
    println!("{}", sum);
}

fn part2(path: &String) {
    let content = read_to_string(path).unwrap();
    let input = parse_input2(content);

    let mut cnt = 0;
    (1..input.time).for_each(|t| {
        if (input.time - t) * t > input.distance {
            cnt += 1;
        }
    });
    println!("{}", cnt);
}

fn parse_input1(content: String) -> Vec<Input> {
    let mut lines = content.lines();
    let mut input: Vec<Input> = Vec::new();
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|e| e.parse().unwrap())
        .collect();
    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|e| e.parse().unwrap())
        .collect();
    times.iter().zip(distances.iter()).for_each(|element| {
        let (&time, &distance) = element;
        input.push(Input { time, distance });
    });
    input
}

fn parse_input2(content: String) -> Input {
    let mut lines = content.lines();
    let time = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();
    Input { time, distance }
}
