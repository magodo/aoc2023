use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Tile {
    Rock,
    Garden,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn offset(&self, x_offset: isize, y_offset: isize) -> Point {
        let x = self.x + x_offset;
        let y = self.y + y_offset;
        Point { x, y }
    }
}

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    wl: Vec<Point>,
    reached: HashMap<Point, u64>,
}

impl Map {
    fn new(content: &str) -> Self {
        let mut width = 0;
        let height = content.lines().count();
        let mut tiles = Vec::new();
        let mut wl = Vec::new();
        content.lines().enumerate().for_each(|(x, line)| {
            width = line.chars().count();
            line.chars().enumerate().for_each(|(y, c)| {
                let tile;
                match c {
                    '#' => tile = Tile::Rock,
                    '.' => tile = Tile::Garden,
                    'S' => {
                        tile = Tile::Garden;
                        wl.push(Point {
                            x: x as isize,
                            y: y as isize,
                        });
                    }
                    _ => unreachable!(),
                };
                tiles.push(tile)
            });
        });
        Self {
            width,
            height,
            tiles,
            wl,
            reached: HashMap::new(),
        }
    }

    fn get(&self, p: Point) -> Tile {
        self.tiles[(p.x.rem_euclid(self.height as isize) * self.height as isize
            + p.y.rem_euclid(self.width as isize)) as usize]
    }

    fn walk_one_step(&mut self, step: u64) {
        let mut new_wl = Vec::new();
        for p in self.wl.clone() {
            for offset in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let next_p = p.offset(offset.0, offset.1);
                let next_tile = self.get(next_p);
                if next_tile == Tile::Rock {
                    continue;
                }
                if self.reached.contains_key(&next_p) {
                    continue;
                }
                self.reached.insert(next_p, step);
                new_wl.push(next_p);
            }
        }
        self.wl = new_wl;
    }

    fn walk(&mut self, step: u64) -> u64 {
        for step in 0..step {
            self.walk_one_step(step);
        }
        self.reached.iter().fold(0, |acc, (_, reach_step)| {
            if reach_step % 2 == (step - 1) % 2 {
                acc + 1
            } else {
                acc
            }
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let content = read_to_string(&args[1]).unwrap();
    let mut map = Map::new(&content);
    println!("{}", map.walk(26501365));
}
