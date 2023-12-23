use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn offset(
        &self,
        x_offset: isize,
        y_offset: isize,
        height: usize,
        width: usize,
    ) -> Option<Point> {
        let x = self.x as isize + x_offset;
        let y = self.y as isize + y_offset;
        if (0..height as isize).contains(&x) && (0..width as isize).contains(&y) {
            Some(Point {
                x: x as usize,
                y: y as usize,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct Hike<'a> {
    map: &'a Map,
    pos: Point,
    end: Point,
    reached: HashSet<Point>,
}

impl Map {
    fn new(content: &str) -> Self {
        let height = content.lines().count();
        let width = content.lines().next().unwrap().chars().count();
        let mut tiles = Vec::new();
        content.lines().for_each(|line| {
            line.chars().for_each(|c| match c {
                '.' => tiles.push(Tile::Path),
                '#' => tiles.push(Tile::Forest),
                '^' => tiles.push(Tile::Slope(Direction::U)),
                'v' => tiles.push(Tile::Slope(Direction::D)),
                '<' => tiles.push(Tile::Slope(Direction::L)),
                '>' => tiles.push(Tile::Slope(Direction::R)),
                _ => unreachable!(),
            });
        });
        Self {
            tiles,
            width,
            height,
        }
    }

    fn get(&self, p: Point) -> Tile {
        self.tiles[p.x * self.width + p.y]
    }

    fn all_hikes(&self) -> Vec<Hike> {
        let mut hikes = vec![Hike::new(
            Point { x: 0, y: 1 },
            Point {
                x: self.height - 1,
                y: self.width - 2,
            },
            &self,
        )];

        while !hikes.iter().all(|hike| hike.is_end()) {
            hikes = hikes.into_iter().flat_map(|hike| hike.next()).collect();
        }

        hikes
    }
}

impl<'a> Hike<'a> {
    fn new(start: Point, end: Point, map: &'a Map) -> Hike {
        Hike {
            map,
            pos: start,
            end,
            reached: HashSet::from([start]),
        }
    }

    fn next(self) -> Vec<Self> {
        if self.is_end() {
            return vec![self];
        }
        let mut next_steps = Vec::new();
        let offsets: Vec<_> = match &self.map.get(self.pos) {
            Tile::Path => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],

            Tile::Forest => vec![],
            Tile::Slope(dir) => vec![match dir {
                Direction::U => (-1, 0),
                Direction::D => (1, 0),
                Direction::L => (0, -1),
                Direction::R => (0, 1),
            }],
        };

        for offset in offsets {
            if let Some(p) = self
                .pos
                .offset(offset.0, offset.1, self.map.height, self.map.width)
            {
                next_steps.push(p);
            }
        }

        let mut hikes = Vec::new();
        for np in next_steps {
            if self.reached.contains(&np) {
                continue;
            }
            let mut reached = self.reached.clone();
            reached.insert(np);
            hikes.push(Hike {
                pos: np,
                reached,
                ..self
            });
        }

        hikes
    }

    fn is_end(&self) -> bool {
        self.pos == self.end
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let content = read_to_string(&args[1]).unwrap();
    let map = Map::new(&content);
    let hikes = map.all_hikes();
    let hike_steps: Vec<_> = hikes.iter().map(|hike| hike.reached.len() - 1).collect();
    println!("{:?}", hike_steps.iter().max().unwrap());
}
