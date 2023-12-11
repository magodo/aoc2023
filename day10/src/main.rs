use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Copy, Clone)]
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

struct Area {
    area: Vec<Vec<Pipe>>,
    main_loop_vec: Vec<(usize, usize)>,
    main_loop_set: HashSet<(usize, usize)>,
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
        let mut area = Area {
            area: vvs,
            main_loop_vec: Vec::new(),
            main_loop_set: HashSet::new(),
        };
        let main_loop = area.build_loop();
        area.main_loop_vec = main_loop;
        let loop_set: HashSet<(usize, usize)> = area.main_loop_vec.clone().into_iter().collect();
        area.main_loop_set = loop_set;

        let start = area.main_loop_vec.first().unwrap();
        let before_start = area.main_loop_vec.last().unwrap();
        let after_start = area.main_loop_vec.get(1).unwrap();

        let gap = (
            after_start.0 as i64 - before_start.0 as i64,
            after_start.1 as i64 - before_start.1 as i64,
        );

        let start_pipe = match gap {
            (2, 0) | (-2, 0) => Pipe::NS,
            (0, 2) | (0, -2) => Pipe::EW,
            (1, 1) => {
                if start.0 == after_start.0 {
                    Pipe::NE
                } else {
                    Pipe::SW
                }
            }
            (1, -1) => {
                if start.0 == after_start.0 {
                    Pipe::NW
                } else {
                    Pipe::SE
                }
            }
            (-1, 1) => {
                if start.0 == after_start.0 {
                    Pipe::SE
                } else {
                    Pipe::NW
                }
            }
            (-1, -1) => {
                if start.0 == after_start.0 {
                    Pipe::SW
                } else {
                    Pipe::NE
                }
            }
            _ => panic!("unreachable"),
        };
        area.area[start.0][start.1] = start_pipe;
        area
    }

    fn start_pipe(&self) -> (usize, usize) {
        for (x, vs) in self.area.iter().enumerate() {
            for (y, v) in vs.iter().enumerate() {
                if *v == Pipe::Start {
                    return (x, y);
                }
            }
        }
        panic!("unreachable");
    }

    fn build_loop(self: &Self) -> Vec<(usize, usize)> {
        let mut vec: Vec<(usize, usize)> = Vec::new();
        let start = self.start_pipe();
        let (x, y) = start;
        vec.push(start);
        loop {
            if let Some(vs) = self.area.get(x) {
                if y > 0 {
                    if let Some(v) = vs.get(y - 1) {
                        if *v == Pipe::EW || *v == Pipe::NE || *v == Pipe::SE {
                            vec.push((x, y - 1));
                            break;
                        }
                    }
                }
                if let Some(v) = vs.get(y + 1) {
                    if *v == Pipe::EW || *v == Pipe::NW || *v == Pipe::SW {
                        vec.push((x, y + 1));
                        break;
                    }
                }
            }
            if x > 0 {
                if let Some(vs) = self.area.get(x - 1) {
                    if let Some(v) = vs.get(y) {
                        if *v == Pipe::NS || *v == Pipe::SW || *v == Pipe::SE {
                            vec.push((x - 1, y));
                            break;
                        }
                    }
                }
            }
            if let Some(vs) = self.area.get(x + 1) {
                if let Some(v) = vs.get(y) {
                    if *v == Pipe::NS || *v == Pipe::NW || *v == Pipe::NE {
                        vec.push((x + 1, y));
                        break;
                    }
                }
            }
        }
        assert!(vec.len() == 2);

        loop {
            if let Some(pos) = self.next(&vec) {
                vec.push(pos);
            } else {
                break vec;
            }
        }
    }

    fn next(self: &Self, vec: &Vec<(usize, usize)>) -> Option<(usize, usize)> {
        let (x_ll, y_ll) = vec.get(vec.len() - 2).unwrap().clone();
        let (x, y) = vec.last().unwrap().clone();
        let mut pos = (0, 0);
        match self.area.get(x).unwrap().get(y).unwrap() {
            Pipe::NS => {
                if (x_ll, y_ll) != (x + 1, y) {
                    pos = (x + 1, y);
                } else if (x_ll, y_ll) != (x - 1, y) {
                    pos = (x - 1, y);
                }
            }
            Pipe::EW => {
                if (x_ll, y_ll) != (x, y + 1) {
                    pos = (x, y + 1);
                } else if (x_ll, y_ll) != (x, y - 1) {
                    pos = (x, y - 1);
                }
            }
            Pipe::NE => {
                if (x_ll, y_ll) != (x, y + 1) {
                    pos = (x, y + 1);
                } else if (x_ll, y_ll) != (x - 1, y) {
                    pos = (x - 1, y);
                }
            }
            Pipe::NW => {
                if (x_ll, y_ll) != (x, y - 1) {
                    pos = (x, y - 1);
                } else if (x_ll, y_ll) != (x - 1, y) {
                    pos = (x - 1, y);
                }
            }
            Pipe::SW => {
                if (x_ll, y_ll) != (x, y - 1) {
                    pos = (x, y - 1);
                } else if (x_ll, y_ll) != (x + 1, y) {
                    pos = (x + 1, y);
                }
            }
            Pipe::SE => {
                if (x_ll, y_ll) != (x, y + 1) {
                    pos = (x, y + 1);
                } else if (x_ll, y_ll) != (x + 1, y) {
                    pos = (x + 1, y);
                }
            }
            _ => panic!("unreachable"),
        };
        if pos == vec.first().unwrap().clone() {
            return None;
        }
        Some(pos)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let area = Area::new(&read_to_string(&args[1]).unwrap());
    println!("{}", area.main_loop_vec.len() / 2);

    let mut cnt = 0;
    area.area.iter().enumerate().for_each(|(x, line)| {
        let mut in_loop = false;
        let mut half_boundary: Option<Pipe> = None;
        line.iter().enumerate().for_each(|(y, p)| {
            if area.main_loop_set.contains(&(x, y)) {
                match *p {
                    Pipe::NS => {
                        in_loop = !in_loop;
                        half_boundary = None;
                    }
                    Pipe::SE => {
                        half_boundary = Some(Pipe::SE);
                    }
                    Pipe::NE => {
                        half_boundary = Some(Pipe::NE);
                    }
                    Pipe::SW => {
                        let hb = half_boundary.take().unwrap();
                        if hb == Pipe::NE {
                            in_loop = !in_loop;
                        }
                    }
                    Pipe::NW => {
                        let hb = half_boundary.take().unwrap();
                        if hb == Pipe::SE {
                            in_loop = !in_loop;
                        }
                    }
                    _ => {}
                }
                return;
            }
            if in_loop {
                cnt += 1;
            }
        })
    });
    println!("{}", cnt);
}
