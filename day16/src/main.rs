use std::cmp::max;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Ground,
    HSplit,
    VSplit,
    ForwardMirror,
    BackwardMirror,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    tile_map: HashMap<Beam, ()>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Beam {
    direction: Direction,
    x: usize,
    y: usize,
}

impl Grid {
    fn new(content: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut tiles: Vec<Tile> = Vec::new();
        content.lines().for_each(|line| {
            height += 1;
            let mut c_num = 0;
            line.chars().for_each(|c| {
                tiles.push(match c {
                    '.' => Tile::Ground,
                    '|' => Tile::VSplit,
                    '-' => Tile::HSplit,
                    '/' => Tile::ForwardMirror,
                    '\\' => Tile::BackwardMirror,
                    _ => unreachable!(),
                });
                c_num += 1;
            });
            width = c_num;
        });

        Grid {
            tiles,
            width,
            height,
            tile_map: HashMap::new(),
        }
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(x * self.height + y)
    }

    fn is_tile_visited(&self, beam: Beam) -> bool {
        self.tile_map.get(&beam).is_some()
    }

    fn step(&mut self, beam: Beam) -> Option<(Beam, Option<Beam>)> {
        if self.is_tile_visited(beam) {
            return None;
        }

        self.tile_map.insert(beam, ());

        match (self.get_tile(beam.x, beam.y).unwrap(), beam.direction) {
            (Tile::Ground | Tile::VSplit, Direction::Up)
            | (Tile::ForwardMirror, Direction::Right)
            | (Tile::BackwardMirror, Direction::Left) => {
                if beam.x <= 0 {
                    None
                } else {
                    Some((
                        Beam {
                            direction: Direction::Up,
                            x: beam.x - 1,
                            ..beam
                        },
                        None,
                    ))
                }
            }
            (Tile::Ground | Tile::VSplit, Direction::Down)
            | (Tile::ForwardMirror, Direction::Left)
            | (Tile::BackwardMirror, Direction::Right) => {
                if beam.x >= self.height - 1 {
                    None
                } else {
                    Some((
                        Beam {
                            direction: Direction::Down,
                            x: beam.x + 1,
                            ..beam
                        },
                        None,
                    ))
                }
            }
            (Tile::Ground | Tile::HSplit, Direction::Left)
            | (Tile::ForwardMirror, Direction::Down)
            | (Tile::BackwardMirror, Direction::Up) => {
                if beam.y <= 0 {
                    None
                } else {
                    Some((
                        Beam {
                            direction: Direction::Left,
                            y: beam.y - 1,
                            ..beam
                        },
                        None,
                    ))
                }
            }
            (Tile::Ground | Tile::HSplit, Direction::Right)
            | (Tile::ForwardMirror, Direction::Up)
            | (Tile::BackwardMirror, Direction::Down) => {
                if beam.y >= self.width - 1 {
                    None
                } else {
                    Some((
                        Beam {
                            direction: Direction::Right,
                            y: beam.y + 1,
                            ..beam
                        },
                        None,
                    ))
                }
            }
            (Tile::HSplit, Direction::Up | Direction::Down) => {
                let mut beam1 = None;
                let mut beam2 = None;
                if beam.y < self.width - 1 {
                    beam1 = Some(Beam {
                        direction: Direction::Right,
                        y: beam.y + 1,
                        ..beam
                    });
                }
                if beam.y > 0 {
                    beam2 = Some(Beam {
                        direction: Direction::Left,
                        y: beam.y - 1,
                        ..beam
                    });
                }
                Self::return_beams(beam1, beam2)
            }
            (Tile::VSplit, Direction::Left | Direction::Right) => {
                let mut beam1 = None;
                let mut beam2 = None;
                if beam.x < self.height - 1 {
                    beam1 = Some(Beam {
                        direction: Direction::Down,
                        x: beam.x + 1,
                        ..beam
                    });
                }
                if beam.x > 0 {
                    beam2 = Some(Beam {
                        direction: Direction::Up,
                        x: beam.x - 1,
                        ..beam
                    });
                }
                Self::return_beams(beam1, beam2)
            }
        }
    }

    fn return_beams(beam1: Option<Beam>, beam2: Option<Beam>) -> Option<(Beam, Option<Beam>)> {
        match (beam1, beam2) {
            (Some(beam1), Some(beam2)) => Some((beam1, Some(beam2))),
            (Some(beam1), None) => Some((beam1, None)),
            (None, Some(beam2)) => Some((beam2, None)),
            (None, None) => None,
        }
    }

    fn run_from(&mut self, beam: Beam) -> usize {
        let mut wl = vec![beam];

        while wl.len() != 0 {
            wl = wl
                .into_iter()
                .flat_map(|beam| {
                    let res = self.step(beam);
                    if let Some(res) = res {
                        let beam1 = res.0;
                        if let Some(beam2) = res.1 {
                            vec![beam1, beam2]
                        } else {
                            vec![beam1]
                        }
                    } else {
                        vec![]
                    }
                })
                .collect();
        }
        let mut m = HashMap::new();
        self.tile_map.keys().for_each(|k| {
            m.insert((k.x, k.y), ());
        });
        return m.len();
    }

    fn clear_tile_map(&mut self) {
        self.tile_map.clear();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let content = read_to_string(&args[1]).unwrap();
    let mut grid = Grid::new(&content);

    let energy = grid.run_from(Beam {
        direction: Direction::Right,
        x: 0,
        y: 0,
    });

    let mut max_energy = 0;
    for x in 0..grid.height {
        for y in 0..grid.width {
            if x != 0 && x != grid.height - 1 && y != 0 && y != grid.width - 1 {
                continue;
            }
            let mut directions = vec![];
            if x == 0 {
                directions.push(Direction::Down);
            }
            if x == grid.height - 1 {
                directions.push(Direction::Up);
            }
            if y == 0 {
                directions.push(Direction::Right);
            }
            if y == grid.width - 1 {
                directions.push(Direction::Left);
            }
            for direction in directions {
                let energy = grid.run_from(Beam { direction, x, y });
                grid.clear_tile_map();
                max_energy = max(max_energy, energy);
            }
        }
    }
    println!("{}", energy);
    println!("{}", max_energy);
}
