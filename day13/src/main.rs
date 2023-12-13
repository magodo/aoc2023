use array2d::Array2D;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Point {
    Ash,
    Rock,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let mut content = read_to_string(&args[1]).unwrap();
    content.push_str("\n");

    let mut space: Vec<Vec<Point>> = Vec::new();
    let mut sum1 = 0;
    let mut sum2 = 0;
    content.lines().for_each(|line| {
        if line.is_empty() {
            sum1 += part1(Array2D::from_rows(&space).unwrap());
            sum2 += part2(Array2D::from_rows(&space).unwrap());
            space.clear();
            return;
        }
        let line = line
            .chars()
            .map(|c| match c {
                '.' => Point::Ash,
                '#' => Point::Rock,
                _ => unreachable!(),
            })
            .collect();
        space.push(line);
    });

    println!("{}", sum1);
    println!("{}", sum2);
}

fn part1(space: Array2D<Point>) -> u64 {
    let row_score = scan_lines(space.as_rows(), 1);
    if row_score != 0 {
        row_score
    } else {
        scan_lines(space.as_columns(), 100)
    }
}

fn scan_lines(lines: Vec<Vec<Point>>, factor: u64) -> u64 {
    let mut mirror: Option<HashSet<usize>> = None;
    for line in lines {
        let set = find_mirror(&line);
        if mirror.is_none() {
            mirror = Some(set);
            continue;
        }
        if let Some(ref mut mirror) = mirror {
            mirror.retain(|e| set.contains(e));
            if mirror.len() == 0 {
                break;
            }
        }
    }
    if let Some(v) = mirror.unwrap().iter().next() {
        return *v as u64 * factor;
    }

    return 0;
}

fn find_mirror(line: &[Point]) -> HashSet<usize> {
    let mut set = HashSet::new();
    for i in 1..line.len() {
        let size = min(i, line.len() - i);
        if line[0..i]
            .iter()
            .rev()
            .take(size)
            .eq(line[i..line.len()].iter().take(size))
        {
            set.insert(i);
        }
    }
    set
}

fn part2(space: Array2D<Point>) -> u64 {
    let row_score = scan_lines2(space.as_rows(), 1);
    if row_score != 0 {
        row_score
    } else {
        scan_lines2(space.as_columns(), 100)
    }
}

fn scan_lines2(lines: Vec<Vec<Point>>, factor: u64) -> u64 {
    let mut dist_vec_map: HashMap<usize, Vec<u64>> = HashMap::new();
    for line in lines {
        for (idx, dist) in mirror_distance(&line) {
            let dist_vec = dist_vec_map.entry(idx).or_insert(Vec::new());
            dist_vec.push(dist);
        }
    }

    for (idx, dist_vec) in dist_vec_map {
        if dist_vec.iter().sum::<u64>() == 1 {
            return idx as u64 * factor;
        }
    }

    return 0;
}

fn mirror_distance(line: &[Point]) -> HashMap<usize, u64> {
    let mut m = HashMap::new();
    for i in 1..line.len() {
        let size = min(i, line.len() - i);
        let dist = line[0..i]
            .iter()
            .rev()
            .take(size)
            .zip(line[i..line.len()].iter().take(size))
            .fold(0, |acc, (x, y)| if x == y { acc } else { acc + 1 });
        m.insert(i, dist);
    }
    m
}
