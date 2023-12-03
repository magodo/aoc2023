use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::process::exit;

#[derive(Debug, PartialEq)]
enum Point {
    Number(IdentNumber),
    Dot,
    Symbol(char),
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct IdentNumber {
    value: u32,
    x: usize,
    y: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("./<exe> <file>");
        exit(1);
    }
    let mut schema: Vec<Vec<Point>> = vec![];
    let mut map: HashMap<IdentNumber, bool> = HashMap::new();
    let mut x: usize = 0;
    read_to_string(&args[1]).unwrap().lines().for_each(|line| {
        let mut vec: Vec<Point> = vec![];
        let mut pline = line.chars().peekable();

        while let Some(_) = pline.peek() {
            let mut size = 0;
            let mut number = 0;
            loop {
                let pv = pline.peek();
                if pv.is_none() || !pv.unwrap().is_ascii_digit() {
                    let y = vec.len();
                    let ident_num = IdentNumber {
                        value: number,
                        x,
                        y,
                    };
                    for _ in 0..size {
                        map.insert(ident_num, false);
                        vec.push(Point::Number(ident_num));
                    }
                    break;
                }
                let e = pline.next().unwrap();
                size += 1;
                number = 10 * number + e.to_digit(10).unwrap();
            }
            if let Some(e) = pline.next() {
                if e == '.' {
                    vec.push(Point::Dot);
                } else {
                    vec.push(Point::Symbol(e));
                }
            } else {
                break;
            }
        }
        schema.push(vec);
        x += 1;
    });
    //println!("{:#?}", schema);

    // Part1
    for (x, vec) in schema.iter().enumerate() {
        for (y, point) in vec.iter().enumerate() {
            if let Point::Symbol(_) = *point {
                for i in x.checked_sub(1).unwrap_or(0)..x + 2 {
                    for j in y.checked_sub(1).unwrap_or(0)..y + 2 {
                        if let Some(vec) = schema.get(i) {
                            if let Some(point) = vec.get(j) {
                                if let Point::Number(num) = point {
                                    map.insert(*num, true);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let sum = map
        .iter()
        .fold(0, |sum, (k, v)| if !v { sum } else { sum + k.value });
    println!("{}", sum);

    // Part2
    let mut sum = 0;
    for (x, vec) in schema.iter().enumerate() {
        for (y, point) in vec.iter().enumerate() {
            if let Point::Symbol('*') = *point {
                let mut map: HashMap<IdentNumber, bool> = HashMap::new();
                for i in x.checked_sub(1).unwrap_or(0)..x + 2 {
                    for j in y.checked_sub(1).unwrap_or(0)..y + 2 {
                        if let Some(vec) = schema.get(i) {
                            if let Some(point) = vec.get(j) {
                                if let Point::Number(num) = point {
                                    map.insert(*num, true);
                                }
                            }
                        }
                    }
                }
                if map.keys().len() == 2 {
                    sum += map.keys().fold(1, |sum, k| sum * k.value);
                }
            }
        }
    }
    println!("{}", sum);
}
