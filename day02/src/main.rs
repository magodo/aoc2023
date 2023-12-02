use std::cmp::max;
use std::env;
use std::fs::read_to_string;
use std::iter::Peekable;
use std::process::exit;
use std::str::Chars;

#[derive(Debug)]
struct RGB(u32, u32, u32);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("./<exe> <file>");
        exit(1);
    }
    let mut games = Vec::new();
    read_to_string(&args[1]).unwrap().lines().for_each(|line| {
        let mut rgb = RGB(0, 0, 0);
        let mut iter = line.chars().peekable();
        get_number(&mut iter);
        while iter.peek().is_some() {
            let v = get_number(&mut iter);
            match get_color(&mut iter) {
                Color::R => {
                    rgb.0 = max(rgb.0, v);
                }
                Color::G => {
                    rgb.1 = max(rgb.1, v);
                }
                Color::B => {
                    rgb.2 = max(rgb.2, v);
                }
            }
        }
        //println!("{:#?}", rgb);
        games.push(rgb);
    });
    let mut answer1 = 0;
    let mut answer2 = 0;
    for (i, rgb) in games.iter().enumerate() {
        if rgb.0 <= 12 && rgb.1 <= 13 && rgb.2 <= 14 {
            answer1 += i + 1;
        }
        answer2 += rgb.0 * rgb.1 * rgb.2;
    }
    println!("{}", answer1);
    println!("{}", answer2);
}

// get_number iterates to the digit and consume and return the number
fn get_number(iter: &mut Peekable<Chars<'_>>) -> u32 {
    while let Some(e) = iter.peek() {
        if e.is_ascii_digit() {
            break;
        }
        iter.next();
    }
    let mut n = 0;
    while let Some(e) = iter.peek() {
        if !e.is_ascii_digit() {
            break;
        }
        n = 10 * n + e.to_digit(10).unwrap();
        iter.next();
    }
    n
}

enum Color {
    R,
    G,
    B,
}

fn get_color(iter: &mut Peekable<Chars<'_>>) -> Color {
    while let Some(e) = iter.peek() {
        match e {
            'r' => {
                iter.nth("red".len());
                return Color::R;
            }
            'g' => {
                iter.nth("green".len());
                return Color::G;
            }
            'b' => {
                iter.nth("blue".len());
                return Color::B;
            }
            _ => iter.next(),
        };
    }
    panic!("non reachable");
}
