use std::env;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Point {
    Round,
    Ground,
    Cube,
}

#[derive(Debug)]
struct Space {
    points: Vec<Point>,
    width: usize,
    height: usize,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    N,
    S,
    W,
    E,
}

#[derive(Debug, Eq, PartialEq)]
struct Record {
    digest: String,
    load: usize,
}

struct InplaceVec<'a, T>(&'a mut [&'a mut T]);

impl<'a, T> InplaceVec<'a, T>
where
    T: Ord + Copy,
{
    fn new(v: &'a mut [&'a mut T]) -> Self {
        Self(v)
    }

    fn sort(&mut self) {
        // buble...
        for i in 0..self.0.len().saturating_sub(1) {
            for j in i..self.0.len() {
                let lh = &self.0[i];
                let rh = &self.0[j];
                if **rh < **lh {
                    let tmp = *self.0[j];
                    *self.0[j] = *self.0[i];
                    *self.0[i] = tmp;
                }
            }
        }
    }

    fn reverse(&mut self) {
        for i in 0..self.0.len() / 2 {
            let j = self.0.len() - 1 - i;
            let tmp = *self.0[i];
            *self.0[i] = *self.0[j];
            *self.0[j] = tmp;
        }
    }
}

impl Space {
    fn new(input: &str) -> Self {
        let mut points = Vec::new();
        let mut height = 0;
        input.lines().for_each(|line| {
            height += 1;
            line.chars().for_each(|c| {
                let p = match c {
                    '.' => Point::Ground,
                    '#' => Point::Cube,
                    'O' => Point::Round,
                    _ => unreachable!(),
                };
                points.push(p);
            });
        });
        let width = input.len() / height - 1; // 1 is the newline
        Self {
            points,
            width,
            height,
        }
    }

    fn tilt(&mut self, direction: Direction) {
        let step;
        let skip_unit;
        let cycle;
        let rev;
        let size;
        match direction {
            Direction::N => {
                step = self.width;
                skip_unit = 1;
                cycle = self.width;
                rev = false;
                size = self.height;
            }
            Direction::S => {
                step = self.width;
                skip_unit = 1;
                cycle = self.width;
                rev = true;
                size = self.height;
            }
            Direction::W => {
                step = 1;
                skip_unit = self.width;
                cycle = self.height;
                rev = false;
                size = self.width;
            }
            Direction::E => {
                step = 1;
                skip_unit = self.width;
                cycle = self.height;
                rev = true;
                size = self.width;
            }
        }
        for i in 0..cycle {
            let mut l: Vec<&mut Point> = self
                .points
                .iter_mut()
                .skip(i * skip_unit)
                .step_by(step)
                .take(size)
                .collect();
            l.split_mut(|p| **p == Point::Cube).for_each(|part| {
                let mut part = InplaceVec::new(part);
                part.sort();
                if rev {
                    part.reverse();
                }
            });
        }
    }

    fn load(&self) -> usize {
        let mut n = 0;
        for i in 0..self.width {
            n += self
                .points
                .iter()
                .skip(i)
                .step_by(self.width)
                .enumerate()
                .fold(0, |acc, (i, e)| {
                    if *e == Point::Round {
                        acc + self.width - i
                    } else {
                        acc
                    }
                });
        }
        n
    }

    fn digest(&self) -> String {
        self.points
            .iter()
            .map(|e| match e {
                Point::Round => "O",
                Point::Ground => ".",
                Point::Cube => "#",
            })
            .collect()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let content = read_to_string(&args[1]).unwrap();

    let mut space = Space::new(&content);
    space.tilt(Direction::N);
    println!("{}", space.load());

    space = Space::new(&content);
    let mut records: Vec<Record> = Vec::new();
    let loop_cnt = 1000000000;
    for i in 0..loop_cnt {
        space.tilt(Direction::N);
        space.tilt(Direction::W);
        space.tilt(Direction::S);
        space.tilt(Direction::E);
        let record = Record {
            digest: space.digest(),
            load: space.load(),
        };
        //dbg!(space.load());
        if let Some(first_idx) = records.iter().position(|e| e.digest == record.digest) {
            let cycle = i - first_idx;
            println!(
                "{}",
                records[first_idx + (loop_cnt - 1 - first_idx) % cycle].load
            );
            //dbg!(first_idx, i, cycle);
            break;
        } else {
            records.push(record);
        }
    }
}

#[test]

fn test() {
    let content = read_to_string("input_demo.txt").unwrap();
    let mut space = Space::new(&content);
    space.tilt(Direction::W);

    let w_content = r#"O....#....
OOO.#....#
.....##...
OO.#OO....
OO......#.
O.#O...#.#
O....#OO..
O.........
#....###..
#OO..#...."#;
    let w_space = Space::new(&w_content);
    assert_eq!(space.points, w_space.points);

    let mut space = Space::new(&content);
    space.tilt(Direction::E);
    let e_content = r#"....O#....
.OOO#....#
.....##...
.OO#....OO
......OO#.
.O#...O#.#
....O#..OO
.........O
#....###..
#..OO#...."#;
    let e_space = Space::new(&e_content);
    assert_eq!(space.points, e_space.points);

    let mut space = Space::new(&content);
    space.tilt(Direction::S);
    let s_content = r#".....#....
....#....#
...O.##...
...#......
O.O....O#O
O.#..O.#.#
O....#....
OO....OO..
#OO..###..
#OO.O#...O"#;
    let s_space = Space::new(&s_content);
    assert_eq!(space.points, s_space.points);
}
