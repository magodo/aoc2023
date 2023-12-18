use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

// Part1
// const MAX_STEP: usize = 3;
// const MIN_STEP: usize = 1;
// Part2
const MAX_STEP: usize = 10;
const MIN_STEP: usize = 4;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum SubState {
    Left(usize),
    Right(usize),
    Top(usize),
    Bottom(usize),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct CellState {
    x: usize,
    y: usize,
    substate: SubState,
}

impl CellState {
    fn next_states(&self, width: usize, height: usize) -> Vec<CellState> {
        let mut l = vec![];
        match self.substate {
            SubState::Left(step) | SubState::Right(step) => {
                if step < MAX_STEP {
                    let y;
                    let substate;
                    if let SubState::Left(_) = self.substate {
                        y = self.y.saturating_add(1);
                        substate = SubState::Left(step + 1);
                    } else {
                        y = self.y.saturating_sub(1);
                        substate = SubState::Right(step + 1);
                    }
                    l.push(CellState {
                        x: self.x,
                        y,
                        substate,
                    });
                }
                if step >= MIN_STEP {
                    l.extend([
                        CellState {
                            x: self.x.saturating_add(1),
                            y: self.y,
                            substate: SubState::Top(1),
                        },
                        CellState {
                            x: self.x.saturating_sub(1),
                            y: self.y,
                            substate: SubState::Bottom(1),
                        },
                    ]);
                }
            }
            SubState::Top(step) | SubState::Bottom(step) => {
                if step < MAX_STEP {
                    let x;
                    let substate;
                    if let SubState::Top(_) = self.substate {
                        x = self.x.saturating_add(1);
                        substate = SubState::Top(step + 1);
                    } else {
                        x = self.x.saturating_sub(1);
                        substate = SubState::Bottom(step + 1);
                    }
                    l.push(CellState {
                        x,
                        y: self.y,
                        substate,
                    });
                }
                if step >= MIN_STEP {
                    l.extend([
                        CellState {
                            x: self.x,
                            y: self.y.saturating_add(1),
                            substate: SubState::Left(1),
                        },
                        CellState {
                            x: self.x,
                            y: self.y.saturating_sub(1),
                            substate: SubState::Right(1),
                        },
                    ]);
                }
            }
        }

        // Filter out invalid states
        l.into_iter()
            .filter(|state| {
                (state.x, state.y) != (self.x, self.y) && state.x < height && state.y < width
            })
            .collect()
    }
}

struct Space {
    dists: Vec<u64>,
    cell_states: HashMap<CellState, u64>,
    width: usize,
    height: usize,
    prev: HashMap<CellState, CellState>,
}

impl Space {
    fn new(content: &str) -> Self {
        let mut height = 0;
        let mut width = 0;
        let mut dists = Vec::new();
        let mut cell_states = HashMap::new();
        content.lines().enumerate().for_each(|(x, line)| {
            height += 1;
            let mut n = 0;
            line.chars().enumerate().for_each(|(y, c)| {
                let v = if (x, y) == (0, 0) { 0 } else { u64::MAX };
                n += 1;
                dists.push(c as u64 - 48);
                for i in 1..=MAX_STEP {
                    cell_states.insert(
                        CellState {
                            x,
                            y,
                            substate: SubState::Left(i),
                        },
                        v,
                    );
                    cell_states.insert(
                        CellState {
                            x,
                            y,
                            substate: SubState::Right(i),
                        },
                        v,
                    );
                    cell_states.insert(
                        CellState {
                            x,
                            y,
                            substate: SubState::Top(i),
                        },
                        v,
                    );
                    cell_states.insert(
                        CellState {
                            x,
                            y,
                            substate: SubState::Bottom(i),
                        },
                        v,
                    );
                }
            });
            width = n;
        });
        Self {
            dists,
            width,
            height,
            cell_states,
            prev: HashMap::new(),
        }
    }

    fn get_distance(&self, x: usize, y: usize) -> u64 {
        self.dists[x * self.width + y]
    }

    fn run_once(&mut self, wl: Vec<CellState>) -> Vec<CellState> {
        let mut next_wl = Vec::new();
        for state in wl {
            let next_states = state.next_states(self.width, self.height);
            for next_state in next_states {
                let dist = self.cell_states[&state] + self.get_distance(next_state.x, next_state.y);
                if dist < self.cell_states[&next_state] {
                    self.cell_states.insert(next_state, dist);
                    self.prev.insert(next_state, state);
                    next_wl.push(next_state);
                }
            }
        }
        next_wl
    }

    fn run(&mut self) -> u64 {
        let mut wl = vec![
            CellState {
                x: 0,
                y: 0,
                substate: SubState::Left(1),
            },
            CellState {
                x: 0,
                y: 0,
                substate: SubState::Top(1),
            },
        ];
        loop {
            wl = self.run_once(wl);
            if wl.len() == 0 {
                break;
            }
        }
        self.min_val(self.height - 1, self.width - 1)
    }

    fn min_val(&self, x: usize, y: usize) -> u64 {
        self.cell_states[&self.min_val_state(x, y)]
    }

    fn min_val_state(&self, x: usize, y: usize) -> CellState {
        let states: Vec<_> = (MIN_STEP..=MAX_STEP)
            .flat_map(|step| {
                [
                    CellState {
                        x,
                        y,
                        substate: SubState::Top(step),
                    },
                    CellState {
                        x,
                        y,
                        substate: SubState::Bottom(step),
                    },
                    CellState {
                        x,
                        y,
                        substate: SubState::Left(step),
                    },
                    CellState {
                        x,
                        y,
                        substate: SubState::Right(step),
                    },
                ]
            })
            .collect();
        let v = states.iter().min_by_key(|v| self.cell_states[v]).unwrap();
        *v
    }

    fn trace(&self) -> Vec<(usize, usize)> {
        let mut p = self.min_val_state(self.height - 1, self.width - 1);
        let mut l = vec![(p.x, p.y)];
        loop {
            if (p.x, p.y) == (0, 0) {
                break;
            }
            p = self.prev[&p];
            l.push((p.x, p.y));
        }
        l.reverse();
        l
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let content = read_to_string(&args[1]).unwrap();
    let mut space = Space::new(&content);
    let score = space.run();
    println!("{}", score);
    //println!("{:?}", space.trace());
}
