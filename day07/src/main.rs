use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Ord)]
struct Input {
    card: Hand,
    bid: u32,
}

#[derive(Debug, PartialEq)]
struct Card(String);

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for (c1, c2) in self.0.chars().zip(other.0.chars()) {
            return c1.partial_cmp(&c2);
        }
        None
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {}
}

#[derive(Debug, PartialEq, Ord)]
enum Hand {
    Five(Card),
    Four(Card),
    FullHouse(Card),
    Three(Card),
    Two(Card),
    One(Card),
    High(Card),
}

impl Card {
    fn new(input: &str) -> Self {
        Card(String::from(input))
    }
}

impl Input {
    fn new(input: &str) -> Self {
        let mut line = input.split_whitespace();
        let card = Hand::new(line.next().unwrap());
        let bid = line.next().unwrap().parse().unwrap();
        Input { card, bid }
    }
}

impl Hand {
    fn new(input: &str) -> Self {
        let mut m: HashMap<char, u8> = HashMap::new();
        String::from(input).chars().for_each(|c| {
            let entry = m.entry(c).or_insert(0);
            *entry += 1;
        });
        let mut cnt: Vec<u8> = m.into_values().collect();
        cnt.sort();
        let inner = Card::new(input);
        match cnt.len() {
            1 => Hand::Five(inner),
            2 => match cnt[0] {
                4 => Hand::Four(inner),
                3 => Hand::FullHouse(inner),
                _ => panic!("nonreachable"),
            },
            3 => match cnt[0] {
                3 => Hand::Three(inner),
                2 => Hand::Two(inner),
                _ => panic!("nonreachable"),
            },
            4 => Hand::One(inner),
            _ => Hand::High(inner),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Hand::Five(self_inner) => match other {
                Hand::Five(other_inner) => self_inner.partial_cmp(other_inner),
                _ => Some(Ordering::Greater),
            },
            Hand::Four(self_inner) => match other {
                Hand::Five(_) => Some(Ordering::Less),
                Hand::Four(other_inner) => self_inner.partial_cmp(other_inner),
                _ => Some(Ordering::Greater),
            },
            Hand::FullHouse(self_inner) => match other {
                Hand::Five(_) | Hand::Four(_) => Some(Ordering::Less),
                Hand::FullHouse(other_inner) => self_inner.partial_cmp(other_inner),
                _ => Some(Ordering::Greater),
            },
            Hand::Three(self_inner) => match other {
                Hand::Five(_) | Hand::Four(_) | Hand::FullHouse(_) => Some(Ordering::Less),
                Hand::Three(other_inner) => self_inner.partial_cmp(other_inner),
                _ => Some(Ordering::Greater),
            },
            Hand::Two(self_inner) => match other {
                Hand::Five(_) | Hand::Four(_) | Hand::FullHouse(_) | Hand::Three(_) => {
                    Some(Ordering::Less)
                }
                Hand::Two(other_inner) => self_inner.partial_cmp(other_inner),
                _ => Some(Ordering::Greater),
            },
            Hand::One(self_inner) => match other {
                Hand::Five(_)
                | Hand::Four(_)
                | Hand::FullHouse(_)
                | Hand::Three(_)
                | Hand::Two(_) => Some(Ordering::Less),
                Hand::One(other_inner) => self_inner.partial_cmp(other_inner),
                _ => Some(Ordering::Greater),
            },
            Hand::High(self_inner) => match other {
                Hand::Five(_)
                | Hand::Four(_)
                | Hand::FullHouse(_)
                | Hand::Three(_)
                | Hand::Two(_)
                | Hand::One(_) => Some(Ordering::Less),
                Hand::High(other_inner) => self_inner.partial_cmp(other_inner),
            },
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: ./exe <file>");
    }

    let mut v: Vec<Input> = read_to_string(&args[1])
        .unwrap()
        .lines()
        .map(|line| Input::new(line))
        .collect();
    v.sort();
}
