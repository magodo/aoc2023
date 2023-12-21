use num::integer::lcm;
use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::fs::read_to_string;
use std::mem;

#[derive(Debug, Clone)]
struct PulseInput {
    src: String,
    pulse: Pulse,
}

#[derive(Debug, Clone)]
struct Task {
    src: String,
    dst: String,
    pulse: Pulse,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{} -{}-> {}", self.src, self.pulse, self.dst).as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    L,
    H,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if mem::discriminant(self) == mem::discriminant(&Self::L) {
            f.write_str("low")
        } else {
            f.write_str("high")
        }
    }
}

trait Module {
    fn handle_pulse(&mut self, pulse: &PulseInput, press_cnt: u64) -> Option<PulseInput>;
    fn children(&self) -> Vec<String>;
    fn digest(&self) -> String;
    fn cycle(&self) -> Option<u64>;
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    enabled: bool,
    children: Vec<String>,
}

impl FlipFlop {
    fn new(name: &str, children: Vec<String>) -> Self {
        let m = FlipFlop {
            name: String::from(name),
            enabled: false,
            children,
        };
        m
    }
}

impl Module for FlipFlop {
    fn handle_pulse(&mut self, pulse: &PulseInput, _: u64) -> Option<PulseInput> {
        match pulse.pulse {
            Pulse::L => {
                self.enabled = !self.enabled;
                if self.enabled {
                    Some(PulseInput {
                        src: self.name.clone(),
                        pulse: Pulse::H,
                    })
                } else {
                    Some(PulseInput {
                        src: self.name.clone(),
                        pulse: Pulse::L,
                    })
                }
            }
            Pulse::H => None,
        }
    }

    fn children(&self) -> Vec<String> {
        self.children.clone()
    }

    fn digest(&self) -> String {
        if self.enabled {
            format!("{}|{}|", self.name, 1)
        } else {
            format!("{}|{}|", self.name, 0)
        }
    }

    fn cycle(&self) -> Option<u64> {
        None
    }
}

struct Conjunction {
    name: String,
    states: HashMap<String, Pulse>,
    children: Vec<String>,
    init_digest: String,
    cycle: Option<u64>,
    active_count: Option<u64>,
}

impl Conjunction {
    fn new(name: &str, parents: &Vec<String>, children: &Vec<String>) -> Self {
        let mut m = Self {
            name: String::from(name),
            states: HashMap::from_iter(
                parents
                    .iter()
                    .map(|p| p.clone())
                    .zip([Pulse::L].repeat(parents.len())),
            ),
            children: children.to_vec(),
            init_digest: "".to_string(),
            cycle: None,
            active_count: None,
        };
        m.init_digest = m.digest();
        m
    }
}

impl Module for Conjunction {
    fn handle_pulse(&mut self, pulse: &PulseInput, press_cnt: u64) -> Option<PulseInput> {
        self.states.insert(pulse.src.clone(), pulse.pulse);

        if self.cycle.is_none() {
            if self.states.iter().all(|(_, state)| *state == Pulse::H) {
                if let Some(active_count) = self.active_count {
                    self.cycle = Some(press_cnt - active_count);
                } else {
                    self.active_count = Some(press_cnt);
                }
            }
        }

        if self.states.values().all(|&p| p == Pulse::H) {
            Some(PulseInput {
                src: self.name.clone(),
                pulse: Pulse::L,
            })
        } else {
            Some(PulseInput {
                src: self.name.clone(),
                pulse: Pulse::H,
            })
        }
    }

    fn children(&self) -> Vec<String> {
        self.children.clone()
    }

    fn digest(&self) -> String {
        let mut state_keys: Vec<&String> = self.states.keys().collect();
        state_keys.sort();
        let states: Vec<String> = state_keys
            .iter()
            .map(|k| match self.states[*k] {
                Pulse::L => "0".to_string(),
                Pulse::H => "1".to_string(),
            })
            .collect();
        let states = states.join("");
        format!("{}|{}|", self.name, states)
    }

    fn cycle(&self) -> Option<u64> {
        self.cycle
    }
}

struct Broadcase {
    name: String,
    children: Vec<String>,
}

impl Broadcase {
    fn new(name: &str, children: Vec<String>) -> Self {
        Broadcase {
            name: String::from(name),
            children,
        }
    }
}

impl Module for Broadcase {
    fn handle_pulse(&mut self, _: &PulseInput, _: u64) -> Option<PulseInput> {
        Some(PulseInput {
            src: self.name.clone(),
            pulse: Pulse::L,
        })
    }

    fn children(&self) -> Vec<String> {
        self.children.clone()
    }

    fn digest(&self) -> String {
        "".to_string()
    }

    fn cycle(&self) -> Option<u64> {
        None
    }
}

struct Probe {
    name: String,
    states: Vec<Pulse>,
}

impl Probe {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            states: Vec::new(),
        }
    }
}

impl Module for Probe {
    fn handle_pulse(&mut self, pulse: &PulseInput, _: u64) -> Option<PulseInput> {
        //dbg!(pulse);
        self.states.push(pulse.pulse);
        None
    }

    fn children(&self) -> Vec<String> {
        Vec::new()
    }

    fn digest(&self) -> String {
        let mut out = vec![self.name.clone()];
        for state in &self.states {
            match state {
                Pulse::L => out.push("0".to_string()),
                Pulse::H => out.push("1".to_string()),
            }
        }
        let out = out.join("|");
        format!("{}|", out)
    }

    fn cycle(&self) -> Option<u64> {
        None
    }
}

struct Modules {
    map: HashMap<String, Box<dyn Module>>,
    cnt: u64,
}

impl Modules {
    fn new(content: &str) -> Modules {
        let mut parents = HashMap::new();
        content.lines().for_each(|line| {
            let [name_line, children_line] =
                line.split("->").collect::<Vec<&str>>().try_into().unwrap();
            let children: Vec<String> = children_line
                .split(",")
                .map(|c| c.trim().to_string())
                .collect();

            let name_line = name_line.trim();
            let name: String;
            match name_line.chars().nth(0).unwrap() {
                '%' | '&' => {
                    name = name_line[1..].to_string();
                }
                _ => {
                    name = name_line.to_string();
                }
            }
            children.iter().for_each(|child| {
                parents
                    .entry(child.clone())
                    .or_insert(Vec::new())
                    .push(name.clone());
            });
        });
        let mut map = HashMap::new();
        content.lines().for_each(|line| {
            let module: Box<dyn Module>;
            let name: String;
            let [name_line, children_line] =
                line.split("->").collect::<Vec<&str>>().try_into().unwrap();
            let children: Vec<String> = children_line
                .split(",")
                .map(|c| c.trim().to_string())
                .collect();

            let name_line = name_line.trim();
            match name_line.chars().nth(0).unwrap() {
                '%' => {
                    name = name_line[1..].to_string();
                    module = Box::new(FlipFlop::new(&name, children));
                }
                '&' => {
                    name = name_line[1..].to_string();
                    module = Box::new(Conjunction::new(&name, &parents[&name], &children));
                }
                _ => {
                    name = name_line.to_string();
                    module = Box::new(Broadcase::new(&name, children));
                }
            }
            map.insert(name.clone(), module);
        });

        for k in parents.keys() {
            if !map.contains_key(k) {
                map.insert(k.clone(), Box::new(Probe::new(k)));
            }
        }
        Modules { map, cnt: 0 }
    }

    fn digest(&self) -> String {
        let mut digests = Vec::new();
        let mut keys: Vec<&String> = self.map.keys().collect();
        keys.sort();
        for k in keys {
            digests.push(self.map[k.as_str()].digest());
        }
        digests.join(",")
    }

    // Process one pulse cycle, returns a digest, #low and #high
    fn pulse(&mut self) -> (String, u64, u64) {
        let mut n_low: u64 = 1;
        let mut n_high: u64 = 0;
        let mut tasks = vec![Task {
            src: "button".to_string(),
            dst: "broadcaster".to_string(),
            pulse: Pulse::L,
        }];

        while tasks.len() != 0 {
            let mut new_tasks = Vec::new();
            // for task in &tasks {
            //     println!("{}", task);
            // }
            tasks.iter().for_each(|task| {
                let module = self.map.get_mut(&task.dst);
                // if module.is_none() {
                //     // This is for the output module
                //     return;
                // }
                let module = module.unwrap();
                if let Some(pulse_input) = module.handle_pulse(
                    &PulseInput {
                        src: task.src.clone(),
                        pulse: task.pulse,
                    },
                    self.cnt,
                ) {
                    let children = module.children();
                    match pulse_input.pulse {
                        Pulse::L => n_low += children.len() as u64,
                        Pulse::H => n_high += children.len() as u64,
                    }
                    for child in children {
                        new_tasks.push(Task {
                            src: pulse_input.src.clone(),
                            dst: child,
                            pulse: pulse_input.pulse,
                        });
                    }
                }
            });
            tasks = new_tasks;
        }

        self.cnt += 1;

        (self.digest(), n_low, n_high)
    }

    fn pulse_n_times(&mut self, n: usize) -> u64 {
        let mut sum_low = 0;
        let mut sum_high = 0;
        let mut digests: Vec<(String, u64, u64)> = Vec::new();
        let mut cycle_start = None;
        for _ in 0..n {
            let (digest, n_low, n_high) = self.pulse();
            if let Some(idx) = digests.iter().position(|(v, _, _)| *v == digest) {
                cycle_start = Some(idx);
                break;
            }
            digests.push((digest, n_low, n_high));
            sum_low += n_low;
            sum_high += n_high;
        }

        if cycle_start.is_none() {
            return sum_low * sum_high;
        }

        let cycle_start = cycle_start.unwrap();
        let cycle_len = digests.len() - cycle_start;
        let cycle_n_low: u64 = digests[cycle_start..].iter().map(|(_, n, _)| *n).sum();
        let cycle_n_high: u64 = digests[cycle_start..].iter().map(|(_, _, n)| *n).sum();

        let head_n_low: u64 = digests[..cycle_start].iter().map(|(_, n, _)| *n).sum();
        let head_n_high: u64 = digests[..cycle_start].iter().map(|(_, _, n)| *n).sum();

        let remainder = (n - cycle_start) % cycle_len;
        let cycle_n = (n - cycle_start) / cycle_len;

        dbg!(
            &digests,
            head_n_low,
            head_n_low,
            cycle_n,
            cycle_n_low,
            cycle_start,
            cycle_len,
            digests[..remainder].iter().map(|(_, n, _)| *n).sum::<u64>()
        );
        let n_low = head_n_low
            + cycle_n as u64 * cycle_n_low
            + digests[..remainder].iter().map(|(_, n, _)| *n).sum::<u64>();
        let n_high = head_n_high
            + cycle_n as u64 * cycle_n_high
            + digests[..remainder].iter().map(|(_, _, n)| *n).sum::<u64>();

        //dbg!(n_low, n_high);
        n_low * n_high
    }

    fn wait_until_cycle(&mut self, names: Vec<&str>) -> Vec<u64> {
        while !names
            .iter()
            .map(|&name| self.map[name].cycle())
            .all(|cycle| cycle.is_some())
        {
            self.pulse();
        }
        names
            .iter()
            .map(|&name| self.map[name].cycle().unwrap())
            .collect()
    }
}

fn dot(content: &str) -> String {
    let mut diag = "digraph G {\n".to_string();

    content.lines().for_each(|line| {
        let [name_line, children_line] =
            line.split("->").collect::<Vec<&str>>().try_into().unwrap();
        let children: Vec<String> = children_line
            .split(",")
            .map(|c| c.trim().to_string())
            .collect();

        let name_line = name_line.trim();
        let name: String;
        let shape: &str;
        match name_line.chars().nth(0).unwrap() {
            '%' => {
                name = name_line[1..].to_string();
                shape = "box";
            }
            '&' => {
                name = name_line[1..].to_string();
                shape = "oval";
            }
            _ => {
                name = name_line.to_string();
                shape = "polyglon";
            }
        }
        let children = "{".to_string() + &children.join(" ") + "}";
        diag += format!("  {} [shape={}]\n", name, shape).as_str();
        diag += format!("  {} -> {}\n", name, children).as_str();
    });
    diag += "}";
    diag
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let content = read_to_string(&args[1]).unwrap();
    println!("{}", dot(&content));

    let mut modules = Modules::new(&content);
    let sum = modules.pulse_n_times(1000);
    println!("{}", sum);

    modules = Modules::new(&content);
    let cycles = modules.wait_until_cycle(vec!["bl", "mr", "pv", "vv"]);
    dbg!(&cycles);
    let cnt = cycles.iter().fold(1 as u64, |acc, &n| lcm(acc, n));
    println!("{}", cnt);
}
