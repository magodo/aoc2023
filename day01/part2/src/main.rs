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
        sum += get_number(line);
    });
    println!("{}", sum);
}

fn get_number(line: &str) -> u32 {
    let targets = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    let idx1 = targets
        .into_iter()
        .enumerate()
        .map(|(i, target)| (i, line.find(target)))
        .filter(|(_, y)| y.is_some())
        .map(|(x, y)| (x, y.unwrap()))
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap()
        .0;

    let idx2 = targets
        .into_iter()
        .enumerate()
        .map(|(i, target)| (i, line.rfind(target)))
        .filter(|(_, y)| y.is_some())
        .map(|(x, y)| (x, y.unwrap()))
        .max_by(|x, y| x.1.cmp(&y.1))
        .unwrap()
        .0;

    let num = number_value(idx1) * 10 + number_value(idx2);
    println!("{}", num);
    num
}

fn number_value(idx: usize) -> u32 {
    if idx < 9 {
        return idx as u32 + 1;
    } else {
        return idx as u32 - 8;
    }
}
