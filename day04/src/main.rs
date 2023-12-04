use std::cmp::min;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("./<exe> <file>");
        exit(1);
    }

    let mut sum_score = 0;
    let mut card_counter = HashMap::new();
    let input = read_to_string(&args[1]).unwrap();
    let count = input.lines().count();
    input.lines().enumerate().for_each(|(idx, line)| {
        let idx = idx as u32 + 1; // starts from card 1
        let this_count = card_counter.entry(idx).or_insert(0);
        *this_count += 1;
        let this_count = *this_count;

        let line: String = line.chars().skip_while(|c| *c != ':').skip(1).collect();
        let numbers: Vec<_> = line.split("|").collect();
        let win_numbers = get_numbers(numbers[0]);
        let user_numbers = get_numbers(numbers[1]);
        let mut matches = 0;

        user_numbers.iter().for_each(|(k, v)| {
            if win_numbers.get(k).is_some() {
                matches += v;
            }
        });

        // For part 1
        let mut score = 0;
        for _ in 0..matches {
            if score == 0 {
                score = 1;
            } else {
                score *= 2;
            }
        }
        sum_score += score;

        // For part 2
        for i in 1..min(matches, count as u32) + 1 {
            let counter = card_counter.entry(idx + i).or_insert(0);
            *counter += this_count;
        }
    });
    println!("{}", sum_score);

    let sum_count = card_counter.iter().fold(0, |sum, (_, cnt)| sum + cnt);
    println!("{}", sum_count);
}

fn get_numbers(line: &str) -> HashMap<u32, u32> {
    let mut out = HashMap::new();

    let mut num = 0;
    line.chars().for_each(|c| {
        if let Some(n) = c.to_digit(10) {
            num = 10 * num + n;
            return;
        }
        if num != 0 {
            let v = out.entry(num).or_insert(0);
            *v += 1;
            num = 0;
        }
    });
    if num != 0 {
        let v = out.entry(num).or_insert(0);
        *v += 1;
    }
    return out;
}
