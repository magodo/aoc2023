use std::cmp::max;
use std::collections::{HashMap, HashSet};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    start: Point,
    end: Point,
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
            start: Point { x: 0, y: 1 },
            end: Point {
                x: height - 1,
                y: width - 2,
            },
        }
    }

    fn get(&self, p: Point) -> Tile {
        self.tiles[p.x * self.width + p.y]
    }

    fn neighbours(&self, point: Point, ignore_slope: bool) -> Vec<Point> {
        let offsets: Vec<_> = match &self.get(point) {
            Tile::Path => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
            Tile::Forest => vec![],
            Tile::Slope(dir) => {
                if ignore_slope {
                    vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
                } else {
                    vec![match dir {
                        Direction::U => (-1, 0),
                        Direction::D => (1, 0),
                        Direction::L => (0, -1),
                        Direction::R => (0, 1),
                    }]
                }
            }
        };

        let mut neighbours = Vec::new();
        for offset in offsets {
            if let Some(p) = point.offset(offset.0, offset.1, self.height, self.width) {
                if self.get(p) != Tile::Forest {
                    neighbours.push(p);
                }
            }
        }
        neighbours
    }
}

#[derive(Debug)]
struct Graph {
    start: Point,
    end: Point,
    edges: HashMap<Point, HashMap<Point, usize>>,
}

impl Graph {
    fn new(map: &Map, ignore_slope: bool) -> Self {
        let mut nodes = HashMap::from([
            (
                map.start,
                vec![Point {
                    x: map.start.x + 1,
                    y: map.start.y,
                }],
            ),
            (
                map.end,
                vec![Point {
                    x: map.end.x - 1,
                    y: map.end.y,
                }],
            ),
        ]);
        map.tiles.iter().enumerate().for_each(|(idx, t)| {
            if *t == Tile::Forest {
                return;
            }
            let pos = Point {
                x: idx / map.width,
                y: idx % map.width,
            };
            let neighbours = map.neighbours(pos, ignore_slope);
            if neighbours.len() >= 3 {
                nodes.insert(pos, neighbours);
            }
        });

        let mut edges = HashMap::new();
        let targets: HashSet<_> = nodes.keys().map(|e| e.clone()).collect();
        nodes.iter().for_each(|(node, neighbours)| {
            for np in neighbours {
                let mut hike = SingleHike::new(*np, HashSet::from([*np, *node]), map, &targets);
                let walk_res = hike.walk(ignore_slope);
                if let Some(peer) = walk_res {
                    let edge = edges.entry(*node).or_insert(HashMap::new());
                    assert!(!edge.contains_key(&peer));
                    edge.insert(peer, hike.reached.len());
                }
            }
        });
        Self {
            start: map.start,
            end: map.end,
            edges,
        }
    }

    fn max_step(&self) -> usize {
        let step = self.dfs(self.start, HashSet::from([self.start]));
        step as usize
    }

    fn dfs(&self, point: Point, seen: HashSet<Point>) -> isize {
        if point == self.end {
            return 0;
        }
        let edges = &self.edges[&point];
        let mut step = isize::MIN;
        for (next_point, d) in edges {
            if seen.contains(&next_point) {
                continue;
            }
            let mut seen = seen.clone();
            seen.insert(*next_point);
            let follow_step = self.dfs(*next_point, seen);
            if (*d as isize + follow_step) > step {
                step = max(step, *d as isize + follow_step);
            }
        }
        step
    }
}

#[derive(Debug)]
struct SingleHike<'a> {
    map: &'a Map,
    pos: Point,
    targets: &'a HashSet<Point>,
    reached: HashSet<Point>,
}

impl<'a> SingleHike<'a> {
    fn new(
        start: Point,
        reached: HashSet<Point>,
        map: &'a Map,
        targets: &'a HashSet<Point>,
    ) -> Self {
        SingleHike {
            map,
            pos: start,
            targets,
            reached,
        }
    }

    fn walk(&mut self, ignore_slope: bool) -> Option<Point> {
        loop {
            let neighbours: Vec<_> = self
                .map
                .neighbours(self.pos, ignore_slope)
                .into_iter()
                .filter(|p| !self.reached.contains(p))
                .collect();
            if neighbours.len() == 0 {
                return None;
            }
            assert!(neighbours.len() == 1);
            let np = neighbours[0];
            if self.targets.contains(&np) {
                return Some(np);
            }
            self.pos = np;
            self.reached.insert(np);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let content = read_to_string(&args[1]).unwrap();

    let map = Map::new(&content);

    let graph = Graph::new(&map, false);
    println!("{}", graph.max_step());

    let graph = Graph::new(&map, true);
    println!("{}", graph.max_step());
}
