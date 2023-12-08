use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Ord)]
struct Card {
    value: String,
    joker_enabled: bool,
}

impl Card {
    fn new(input: &str, joker_enabled: bool) -> Self {
        Card {
            value: String::from(input),
            joker_enabled,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut sorted_card: Vec<char> = "AKQJT98765432".chars().collect();
        if self.joker_enabled {
            sorted_card = "AKQT98765432J".chars().collect();
        }
        let len = sorted_card.len();
        let m: HashMap<char, usize> = sorted_card
            .into_iter()
            .zip((0..len).rev().collect::<Vec<usize>>())
            .collect();
        let mut order: Option<Ordering> = None;
        for (c1, c2) in self.value.chars().zip(other.value.chars()) {
            order = m.get(&c1).unwrap().partial_cmp(m.get(&c2).unwrap());
            if let Some(ord) = order {
                if ord != Ordering::Equal {
                    return order;
                }
            }
        }
        return order;
    }
}

#[derive(Debug, PartialEq, Eq, Ord)]
enum Hand {
    Five(Card),
    Four(Card),
    FullHouse(Card),
    Three(Card),
    Two(Card),
    One(Card),
    High(Card),
}

impl Hand {
    fn new(input: &str, joker_enabled: bool) -> Self {
        let mut m: HashMap<char, u8> = HashMap::new();
        String::from(input).chars().for_each(|c| {
            let entry = m.entry(c).or_insert(0);
            *entry += 1;
        });

        let mut joker_cnt = 0;
        if let Some(v) = m.get(&'J') {
            joker_cnt = *v;
        }

        let mut cnt: Vec<u8> = m.into_values().collect();
        cnt.sort();
        cnt.reverse();

        if joker_enabled {
            if let Some(idx) = cnt.iter().position(|&e| e == joker_cnt) {
                if cnt.len() != 1 {
                    let v = cnt.remove(idx);
                    cnt[0] += v;
                }
            }
        }

        let inner = Card::new(input, joker_enabled);
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

#[derive(Debug, PartialEq, Eq, Ord)]
struct Input {
    card: Hand,
    bid: u32,
}

impl PartialOrd for Input {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.card.partial_cmp(&other.card)
    }
}

impl Input {
    fn new(input: &str, joker_enabled: bool) -> Self {
        let mut line = input.split_whitespace();
        let card = Hand::new(line.next().unwrap(), joker_enabled);
        let bid = line.next().unwrap().parse().unwrap();
        Input { card, bid }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: ./exe <file>");
    }
    run(&args[1], false);
    run(&args[1], true);
}

fn run(path: &str, joker_enabled: bool) {
    let mut v: Vec<Input> = read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| Input::new(line, joker_enabled))
        .collect();
    v.sort();
    let mut sum: u32 = 0;
    v.iter().enumerate().for_each(|(i, v)| {
        sum += (i as u32 + 1) * v.bid;
    });
    println!("{}", sum);
}
