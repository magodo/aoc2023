use std::env;
use std::fs::read_to_string;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("./<exe> <file>");
        exit(1);
    }
    let mut sum = 0;
    read_to_string(&args[1]).unwrap().lines().for_each(|line| {
        let mut n1: Option<u32> = None;
        let mut n2: Option<u32> = None;
        for char in line.chars() {
            if let Some(n) = char.to_digit(10) {
                if n1.is_none() {
                    n1 = Some(n);
                    continue;
                }
                n2 = Some(n);
            }
        }
        if let Some(n1) = n1 {
            if let Some(n2) = n2 {
                println!("{}{}", n1, n2);
                sum += n1 * 10 + n2;
            } else {
                println!("{}{}", n1, n1);
                sum += n1 * 11;
            }
        }
    });
    println!("{}", sum);
}
