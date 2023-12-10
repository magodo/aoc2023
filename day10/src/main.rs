use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Start,
    Ground,
}

struct Area(Vec<Vec<Pipe>>);

#[derive(Debug)]
struct NextMove {
    pos: Option<(usize, usize)>,
    step: usize,
}

impl Area {
    fn new(input: &str) -> Self {
        let mut vvs: Vec<Vec<Pipe>> = Vec::new();
        input.lines().for_each(|line| {
            let mut vs: Vec<Pipe> = Vec::new();
            line.chars().for_each(|c| {
                vs.push(match c {
                    '|' => Pipe::NS,
                    '-' => Pipe::EW,
                    'L' => Pipe::NE,
                    'J' => Pipe::NW,
                    '7' => Pipe::SW,
                    'F' => Pipe::SE,
                    '.' => Pipe::Ground,
                    'S' => Pipe::Start,
                    _ => panic!("unreachable"),
                });
            });
            vvs.push(vs);
        });
        Area(vvs)
    }

    fn start_pipe(&self) -> (usize, usize) {
        for (x, vs) in self.0.iter().enumerate() {
            for (y, v) in vs.iter().enumerate() {
                if *v == Pipe::Start {
                    return (x, y);
                }
            }
        }
        panic!("unreachable");
    }

    fn farest_step(self: &Self) -> usize {
        let mut m: HashMap<(usize, usize), usize> = HashMap::new();
        let (x, y) = self.start_pipe();
        m.insert((x, y), 0);
        let mut start_points: Vec<(usize, usize)> = Vec::new();
        if let Some(vs) = self.0.get(x) {
            if y > 0 {
                if let Some(v) = vs.get(y - 1) {
                    if *v == Pipe::EW || *v == Pipe::NE || *v == Pipe::SE {
                        start_points.push((x, y - 1));
                    }
                }
            }
            if let Some(v) = vs.get(y + 1) {
                if *v == Pipe::EW || *v == Pipe::NW || *v == Pipe::SW {
                    start_points.push((x, y + 1));
                }
            }
        }
        if x > 0 {
            if let Some(vs) = self.0.get(x - 1) {
                if let Some(v) = vs.get(y) {
                    if *v == Pipe::NS || *v == Pipe::SW || *v == Pipe::SE {
                        start_points.push((x - 1, y));
                    }
                }
            }
        }
        if let Some(vs) = self.0.get(x + 1) {
            if let Some(v) = vs.get(y) {
                if *v == Pipe::NS || *v == Pipe::NW || *v == Pipe::NE {
                    start_points.push((x + 1, y));
                }
            }
        }
        assert!(start_points.len() == 2);

        let mut p1: (usize, usize) = start_points[0];
        let mut p2: (usize, usize) = start_points[1];

        m.insert(p1, 1);
        m.insert(p2, 1);

        loop {
            let next = self.move_step(p1, &mut m);
            if let Some(p) = next.pos {
                p1 = p;
            } else {
                break next.step;
            }
            let next = self.move_step(p2, &mut m);
            if let Some(p) = next.pos {
                p2 = p;
            } else {
                break next.step;
            }
        }
    }

    fn move_step(
        self: &Self,
        p: (usize, usize),
        m: &mut HashMap<(usize, usize), usize>,
    ) -> NextMove {
        let (x, y) = p;
        let step = m.get(&(x, y)).unwrap().clone();
        let mut pos: Option<(usize, usize)> = None;
        match self.0.get(x).unwrap().get(y).unwrap() {
            Pipe::NS => {
                if !m.contains_key(&(x + 1, y)) {
                    pos = Some((x + 1, y));
                } else if !m.contains_key(&(x - 1, y)) {
                    pos = Some((x - 1, y));
                }
            }
            Pipe::EW => {
                if !m.contains_key(&(x, y + 1)) {
                    pos = Some((x, y + 1));
                } else if !m.contains_key(&(x, y - 1)) {
                    pos = Some((x, y - 1));
                }
            }
            Pipe::NE => {
                if !m.contains_key(&(x, y + 1)) {
                    pos = Some((x, y + 1));
                } else if !m.contains_key(&(x - 1, y)) {
                    pos = Some((x - 1, y));
                }
            }
            Pipe::NW => {
                if !m.contains_key(&(x, y - 1)) {
                    pos = Some((x, y - 1));
                } else if !m.contains_key(&(x - 1, y)) {
                    pos = Some((x - 1, y));
                }
            }
            Pipe::SW => {
                if !m.contains_key(&(x, y - 1)) {
                    pos = Some((x, y - 1));
                } else if !m.contains_key(&(x + 1, y)) {
                    pos = Some((x + 1, y));
                }
            }
            Pipe::SE => {
                if !m.contains_key(&(x + 1, y)) {
                    pos = Some((x + 1, y));
                } else if !m.contains_key(&(x, y + 1)) {
                    pos = Some((x, y + 1));
                }
            }
            _ => panic!("unreachable"),
        };
        if let Some(pos) = pos {
            m.insert(pos, step + 1);
        }
        return NextMove {
            pos,
            step: step + 1,
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let area = Area::new(&read_to_string(&args[1]).unwrap());
    let step = area.farest_step();
    println!("{}", step);
}
