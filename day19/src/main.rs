use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::ops::Range;

#[derive(Debug)]
enum EvaResult {
    Reject,
    Accept,
    Move(String),
}

impl From<&str> for EvaResult {
    fn from(value: &str) -> Self {
        match value {
            "A" => EvaResult::Accept,
            "R" => EvaResult::Reject,
            _ => EvaResult::Move(value.to_string()),
        }
    }
}

#[derive(Debug)]
struct Workflows<'a>(HashMap<&'a str, Workflow>);

impl<'a> Workflows<'a> {
    fn eva(&self, part: &Part) -> Option<u64> {
        let mut workflow = &self.0["in"];
        loop {
            let res = workflow.eva(part);
            match res {
                EvaResult::Reject => return None,
                EvaResult::Accept => return Some(part.sum()),
                EvaResult::Move(next) => {
                    workflow = &self.0[next.as_str()];
                }
            }
        }
    }

    fn acceptable_ranges(&self) -> Vec<PartRange> {
        let mut wl = vec![(
            "in".to_string(),
            PartRange {
                x: 1..4001,
                s: 1..4001,
                a: 1..4001,
                m: 1..4001,
            },
        )];
        let mut valid_ranges = Vec::new();
        while wl.len() != 0 {
            let mut new_wl = Vec::new();
            for (name, range) in wl {
                let moves = self.0[name.as_str()].next_moves(&range);
                for mov in moves {
                    match mov.next {
                        EvaResult::Reject => continue,
                        EvaResult::Accept => valid_ranges.push(mov.range),
                        EvaResult::Move(name) => new_wl.push((name, mov.range.clone())),
                    }
                }
            }
            wl = new_wl;
        }
        valid_ranges
    }
}

#[derive(Debug)]
struct Workflow {
    rule: String,
}

impl Workflow {
    fn rules(&self) -> Vec<(Option<&str>, EvaResult)> {
        let mut rules = Vec::new();
        for rule in self.rule.split(",") {
            if rule.contains(":") {
                let [condition, res] = rule.split(":").collect::<Vec<&str>>().try_into().unwrap();
                rules.push((Some(condition), EvaResult::from(res)));
            } else {
                rules.push((None, EvaResult::from(rule)));
            }
        }
        rules
    }

    fn eva(&self, part: &Part) -> EvaResult {
        for (condition, res) in self.rules() {
            if let Some(condition) = condition {
                if part.meet(condition) {
                    return res;
                }
            } else {
                return res;
            }
        }
        unreachable!();
    }

    fn next_moves(&self, range: &PartRange) -> Vec<Move> {
        let mut moves = Vec::new();
        let mut range = range.clone();
        for (condition, res) in self.rules() {
            if let Some(condition) = condition {
                if let Ok((valid_next, invalid_next)) = range.narrow(condition) {
                    moves.push(Move {
                        range: valid_next,
                        next: res,
                    });
                    if let Some(invalid_next) = invalid_next {
                        range = invalid_next;
                    } else {
                        break;
                    }
                }
            } else {
                moves.push(Move {
                    range: range.clone(),
                    next: res,
                });
            }
        }
        moves
    }
}

#[derive(Debug)]
struct Part {
    x: u64,
    s: u64,
    m: u64,
    a: u64,
}

#[derive(Debug, Clone)]
struct PartRange {
    x: Range<u64>,
    s: Range<u64>,
    m: Range<u64>,
    a: Range<u64>,
}

impl PartRange {
    fn narrow(&self, rule: &str) -> Result<(PartRange, Option<PartRange>), ()> {
        let sep;
        if rule.contains("<") {
            sep = "<";
        } else {
            sep = ">";
        }

        let [part, val] = rule.split(sep).collect::<Vec<&str>>().try_into().unwrap();
        let val: u64 = val.parse().unwrap();
        match part {
            "x" => {
                let (valid_range, invalid_range) = Self::narrow_condition(&self.x, sep, val)?;
                let valid_range = PartRange {
                    x: valid_range,
                    ..self.clone()
                };
                let invalid_range = if let Some(invalid_range) = invalid_range {
                    Some(PartRange {
                        x: invalid_range,
                        ..self.clone()
                    })
                } else {
                    None
                };
                Ok((valid_range, invalid_range))
            }
            "a" => {
                let (valid_range, invalid_range) = Self::narrow_condition(&self.a, sep, val)?;
                let valid_range = PartRange {
                    a: valid_range,
                    ..self.clone()
                };
                let invalid_range = if let Some(invalid_range) = invalid_range {
                    Some(PartRange {
                        a: invalid_range,
                        ..self.clone()
                    })
                } else {
                    None
                };
                Ok((valid_range, invalid_range))
            }
            "s" => {
                let (valid_range, invalid_range) = Self::narrow_condition(&self.s, sep, val)?;
                let valid_range = PartRange {
                    s: valid_range,
                    ..self.clone()
                };
                let invalid_range = if let Some(invalid_range) = invalid_range {
                    Some(PartRange {
                        s: invalid_range,
                        ..self.clone()
                    })
                } else {
                    None
                };
                Ok((valid_range, invalid_range))
            }
            "m" => {
                let (valid_range, invalid_range) = Self::narrow_condition(&self.m, sep, val)?;
                let valid_range = PartRange {
                    m: valid_range,
                    ..self.clone()
                };
                let invalid_range = if let Some(invalid_range) = invalid_range {
                    Some(PartRange {
                        m: invalid_range,
                        ..self.clone()
                    })
                } else {
                    None
                };
                Ok((valid_range, invalid_range))
            }
            _ => unreachable!(),
        }
    }

    fn narrow_condition(
        range: &Range<u64>,
        op: &str,
        val: u64,
    ) -> Result<(Range<u64>, Option<Range<u64>>), ()> {
        match op {
            "<" => {
                if range.start >= val {
                    Err(())
                } else if range.contains(&val) {
                    Ok((
                        Range {
                            start: range.start,
                            end: val,
                        },
                        Some(Range {
                            start: val,
                            end: range.end,
                        }),
                    ))
                } else {
                    Ok((range.clone(), None))
                }
            }
            ">" => {
                if range.end <= val {
                    Err(())
                } else if range.contains(&val) {
                    Ok((
                        Range {
                            start: val + 1,
                            end: range.end,
                        },
                        Some(Range {
                            start: range.start,
                            end: val + 1,
                        }),
                    ))
                } else {
                    Ok((range.clone(), None))
                }
            }
            _ => unreachable!(),
        }
    }

    fn sum(&self) -> u64 {
        (self.x.end - self.x.start)
            * (self.a.end - self.a.start)
            * (self.s.end - self.s.start)
            * (self.m.end - self.m.start)
    }
}

#[derive(Debug)]
struct Move {
    range: PartRange,
    next: EvaResult,
}

impl Part {
    fn meet(&self, rule: &str) -> bool {
        let sep;
        if rule.contains("<") {
            sep = "<";
        } else {
            sep = ">";
        }

        let [part, val] = rule.split(sep).collect::<Vec<&str>>().try_into().unwrap();
        let val: u64 = val.parse().unwrap();
        match part {
            "x" => Self::meet_condition(self.x, sep, val),
            "s" => Self::meet_condition(self.s, sep, val),
            "m" => Self::meet_condition(self.m, sep, val),
            "a" => Self::meet_condition(self.a, sep, val),
            _ => unreachable!(),
        }
    }

    fn meet_condition(left: u64, op: &str, right: u64) -> bool {
        match op {
            "<" => left < right,
            ">" => left > right,
            _ => unreachable!(),
        }
    }

    fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

fn parse(content: &str) -> (Workflows, Vec<Part>) {
    let mut is_part = false;
    let mut parts = Vec::new();
    let mut workflows = HashMap::new();
    content.lines().for_each(|line| {
        if line.is_empty() {
            is_part = true;
            return;
        }
        if !is_part {
            let [name, line] = line.split("{").collect::<Vec<&str>>().try_into().unwrap();
            let rule = line.strip_suffix("}").unwrap().to_string();
            workflows.insert(name, Workflow { rule });
        } else {
            let line = line.strip_prefix("{").unwrap().strip_suffix("}").unwrap();
            let m: HashMap<&str, u64> = HashMap::from_iter(
                line.split(",")
                    .map(|category| category.split("=").collect::<Vec<&str>>())
                    .map(|v| (v[0], v[1].parse().unwrap())),
            );
            parts.push(Part {
                x: m["x"],
                s: m["s"],
                m: m["m"],
                a: m["a"],
            });
        }
    });
    (Workflows(workflows), parts)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let content = read_to_string(&args[1]).unwrap();
    let (workflows, parts) = parse(&content);

    let part1 = parts.iter().fold(0, |acc, part| {
        if let Some(v) = workflows.eva(part) {
            acc + v
        } else {
            acc
        }
    });
    println!("{}", part1);

    let mut sum = 0;
    for range in workflows.acceptable_ranges() {
        sum += range.sum();
    }
    println!("{}", sum);
}
