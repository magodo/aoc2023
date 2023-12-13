use std::cmp::{max, min};
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let content = read_to_string(&args[1]).unwrap();

    let raw_input: Vec<Vec<char>> = parse_input(content);

    let mut empty_row: HashSet<usize> = HashSet::new();
    let mut empty_col: HashSet<usize> = HashSet::new();
    raw_input.iter().enumerate().for_each(|(i, line)| {
        if line.iter().all(|c| *c == '.') {
            empty_row.insert(i);
        }
    });
    for i in 0..raw_input[0].len() {
        if raw_input.iter().map(|line| line[i]).all(|c| c == '.') {
            empty_col.insert(i);
        }
    }

    // part1
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    raw_input.iter().enumerate().for_each(|(x, line)| {
        line.iter().enumerate().for_each(|(y, c)| {
            if *c == '#' {
                galaxies.push((x, y));
            }
        })
    });

    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum1 += distance(galaxies[i], galaxies[j], &empty_row, &empty_col, 2);
            sum2 += distance(galaxies[i], galaxies[j], &empty_row, &empty_col, 1000000);
        }
    }

    println!("{}", sum1);
    println!("{}", sum2);
}

fn distance(
    i: (usize, usize),
    j: (usize, usize),
    empty_row: &HashSet<usize>,
    empty_col: &HashSet<usize>,
    factor: usize,
) -> usize {
    let row_exp = (min(i.0, j.0)..max(i.0, j.0))
        .filter(|idx| empty_row.contains(idx))
        .count();

    let col_exp = (min(i.1, j.1)..max(i.1, j.1))
        .filter(|idx| empty_col.contains(idx))
        .count();

    let x_diff;
    if i.0 > j.0 {
        x_diff = i.0 - j.0;
    } else {
        x_diff = j.0 - i.0;
    }

    let y_diff;
    if i.1 > j.1 {
        y_diff = i.1 - j.1;
    } else {
        y_diff = j.1 - i.1;
    }

    return x_diff + y_diff + (row_exp + col_exp) * (factor - 1);
}

fn parse_input(content: String) -> Vec<Vec<char>> {
    let mut out: Vec<Vec<char>> = Vec::new();
    content.lines().for_each(|line| {
        let mut v: Vec<char> = Vec::new();
        line.chars().for_each(|c| {
            v.push(c);
        });
        out.push(v);
    });

    out
}
