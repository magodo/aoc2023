use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let content = read_to_string(&args[1]).unwrap();
    let mut sum1 = 0;
    let mut sum2 = 0;
    content.lines().for_each(|line| {
        let mut numbers: Vec<i64> = line
            .split_whitespace()
            .map(|e| e.parse().unwrap())
            .collect();
        let mut cumulator1 = 0;
        let mut cumulator2 = 0;
        let mut factor = 1;
        loop {
            cumulator1 += numbers.last().unwrap();
            cumulator2 += factor * numbers.first().unwrap();
            factor *= -1;
            numbers = numbers
                .iter()
                .skip(1)
                .zip(numbers.iter())
                .map(|(x, y)| x - y)
                .collect();
            if numbers.iter().all(|n| *n == 0) {
                break;
            }
        }
        sum1 += cumulator1;
        sum2 += cumulator2;
    });
    println!("{}", sum1);
    println!("{}", sum2);
}
