use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Key {
    input: String,
    record: String,
    hint: Option<char>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let content = read_to_string(&args[1]).unwrap();
    let mut cnt = 0;
    let mut memo: HashMap<Key, u64> = HashMap::new();

    content.lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let row = parts[0];
        let record: Vec<u64> = parts[1].split(',').map(|e| e.parse().unwrap()).collect();
        let x = enumerate(row, &record, &mut memo, None);
        cnt += x;
    });

    println!("{}", cnt);

    cnt = 0;
    content.lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let row = [parts[0]].repeat(5).join("?");
        let record: Vec<u64> = parts[1]
            .split(',')
            .map(|e| e.parse().unwrap())
            .collect_vec()
            .repeat(5);
        let x = enumerate(&row, &record, &mut memo, None);
        cnt += x;
    });

    println!("{}", cnt);
}

fn enumerate(row: &str, record: &[u64], memo: &mut HashMap<Key, u64>, hint: Option<char>) -> u64 {
    let key = Key {
        input: row.to_string(),
        record: record.iter().map(|e| e.to_string()).join(","),
        hint,
    };

    if let Some(n) = memo.get(&key) {
        return *n;
    }

    if record.len() == 0 {
        if row.chars().any(|c| c == '#') {
            memo.insert(key, 0);
            return 0;
        }
        memo.insert(key, 1);
        return 1;
    }

    if row.len() == 0 {
        memo.insert(key, 0);
        return 0;
    }

    match row.chars().nth(0).unwrap() {
        '.' => {
            if let Some(v) = hint {
                if v != '.' {
                    memo.insert(key, 0);
                    return 0;
                }
            }
            let cnt = enumerate(&row[1..], record, memo, None);
            memo.insert(key, cnt);
            cnt
        }
        '#' => {
            if let Some(v) = hint {
                if v != '#' {
                    memo.insert(key, 0);
                    return 0;
                }
            }
            let cnt;
            if record[0] == 1 {
                cnt = enumerate(&row[1..], &record[1..], memo, Some('.'));
            } else {
                cnt = enumerate(
                    &row[1..],
                    &[&[record[0] - 1], &record[1..]].concat(),
                    memo,
                    Some('#'),
                );
            }
            memo.insert(key, cnt);
            cnt
        }
        '?' => {
            let cnt;
            if let Some(v) = hint {
                if v == '.' {
                    cnt = enumerate(&[".", &row[1..]].concat(), record, memo, None);
                } else {
                    cnt = enumerate(&["#", &row[1..]].concat(), record, memo, None);
                }
            } else {
                cnt = enumerate(&[".", &row[1..]].concat(), record, memo, None)
                    + enumerate(&["#", &row[1..]].concat(), record, memo, None);
            }
            memo.insert(key, cnt);
            cnt
        }
        _ => unreachable!(),
    }
}
