use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
struct Box {
    lens: Vec<Len>,
    // Stores the label -> index in the lens list for each len
    idx_map: HashMap<String, usize>,
}

impl Box {
    fn new() -> Self {
        Self {
            lens: Vec::new(),
            idx_map: HashMap::new(),
        }
    }

    fn add(&mut self, label: &str, focal_length: u64) {
        let label = label.to_string();
        let len = Len {
            label,
            focal_length,
        };
        if let Some(idx) = self.idx_map.get(&len.label) {
            let mut iter = self.lens.iter_mut();
            for _ in 0..*idx {
                iter.next();
            }
            let v = iter.next().unwrap();
            *v = len;
        } else {
            self.idx_map.insert(len.label.clone(), self.lens.len());
            self.lens.push(len);
        }
    }

    fn remove(&mut self, label: &str) {
        if let Some(idx) = self.idx_map.remove(label) {
            self.lens.remove(idx);
            self.idx_map
                .values_mut()
                .filter(|&&mut v| v > idx)
                .for_each(|v| *v -= 1);
        }
    }
}

#[derive(Debug)]
struct Len {
    label: String,
    focal_length: u64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let content = read_to_string(&args[1]).unwrap();
    let input = content.lines().next().unwrap();
    let mut result = 0;
    input.split(',').for_each(|str| {
        result += hash(str);
    });
    println!("{}", result);

    let mut boxes: HashMap<usize, Box> = HashMap::new();
    input.split(',').for_each(|str| {
        let (label, focal_length) = str.split_once(|c| c == '-' || c == '=').unwrap();
        let hash_value = hash(label);
        let boks = boxes.entry(hash_value).or_insert(Box::new());
        if str.contains("-") {
            boks.remove(label);
            if boks.lens.len() == 0 {
                boxes.remove(&hash_value);
            }
        } else {
            boks.add(label, focal_length.parse().unwrap());
        }
    });
    //dbg!(&boxes);
    let focusing_power = boxes.iter().fold(0, |acc, (box_idx, boks)| {
        acc + boks.lens.iter().enumerate().fold(0, |acc, (len_idx, len)| {
            acc + (box_idx + 1) * (len_idx + 1) * len.focal_length as usize
        })
    });
    println!("{}", focusing_power);
}

fn hash(str: &str) -> usize {
    str.chars().fold(0, |acc, c| (acc + c as usize) * 17 % 256)
}
