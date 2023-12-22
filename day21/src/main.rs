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
    fn offset(
        &self,
        x_offset: isize,
        y_offset: isize,
        height: usize,
        width: usize,
    ) -> Option<Point> {
        let x = self.x + x_offset;
        let y = self.y + y_offset;
        if !(0..height as isize).contains(&x) || !(0..width as isize).contains(&y) {
            None
        } else {
            Some(Point { x, y })
        }
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
                let next_p = p.offset(offset.0, offset.1, self.height, self.width);
                if let Some(next_p) = next_p {
                    let next_tile = self.get(next_p);
                    if next_tile == Tile::Rock {
                        continue;
                    }
                    if self.reached.contains_key(&next_p) {
                        continue;
                    }
                    self.reached.insert(next_p, step + 1);
                    new_wl.push(next_p);
                }
            }
        }
        self.wl = new_wl;
    }

    fn walk(&mut self, step: u64) {
        for step in 0..step {
            self.walk_one_step(step);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let content = read_to_string(&args[1]).unwrap();
    let mut map = Map::new(&content);

    let step = 64;
    map.walk(step);
    let p1 = map.reached.iter().fold(0, |acc, (_, reach_step)| {
        if reach_step % 2 == (step) % 2 {
            acc + 1
        } else {
            acc
        }
    });
    println!("{}", p1);

    let mut map = Map::new(&content);
    let step = 131;
    map.walk(step);
    let odd_full = map.reached.iter().fold(
        0,
        |acc, (_, reach_step)| {
            if reach_step % 2 == 1 {
                acc + 1
            } else {
                acc
            }
        },
    );
    let even_full = map.reached.iter().fold(
        0,
        |acc, (_, reach_step)| {
            if reach_step % 2 == 0 {
                acc + 1
            } else {
                acc
            }
        },
    );
    let odd_corners = map.reached.iter().fold(0, |acc, (_, reach_step)| {
        if reach_step % 2 == 1 && *reach_step > 65 {
            acc + 1
        } else {
            acc
        }
    });
    let even_corners = map.reached.iter().fold(0, |acc, (_, reach_step)| {
        if reach_step % 2 == 0 && *reach_step > 65 {
            acc + 1
        } else {
            acc
        }
    });
    let n = ((26501365 - (131 / 2)) / 131) as usize;
    assert_eq!(n, 202300);
    let p2 = ((n + 1) * (n + 1)) * odd_full + (n * n) * even_full - (n + 1) * odd_corners
        + n * even_corners;

    println!("{}", p2);

    // println!(
    //     "{:?}",
    //     map.reached
    //         .iter()
    //         .filter(|r| *r.1 == 130)
    //         .collect::<Vec<_>>()
    // );
}
