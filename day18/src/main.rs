use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::mem::size_of;
use std::ops::Range;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    L,
    R,
    U,
    D,
}

#[derive(Copy, Clone, Debug)]
struct Dig {
    direction: Direction,
    step: usize,
}

struct Plan(Vec<Dig>);

impl Plan {
    fn new(content: &str) -> Self {
        let mut inner = Vec::new();
        content.lines().for_each(|line| {
            let mut line = line.split_whitespace();
            let direction = match line.next().unwrap() {
                "R" => Direction::R,
                "L" => Direction::L,
                "U" => Direction::U,
                "D" => Direction::D,
                _ => unreachable!(),
            };
            let step = line.next().unwrap().parse().unwrap();
            inner.push(Dig { direction, step });
        });

        // Merge the lines on the same direction to guarantee one direction has only one line
        if inner[0].direction == inner.last().unwrap().direction {
            let first_step = inner.first().unwrap().step;
            let last = inner.last_mut().unwrap();
            last.step += first_step;
            inner = inner[1..].to_vec();
        }

        Self(inner)
    }
    fn new2(content: &str) -> Self {
        let mut inner = Vec::new();
        content.lines().for_each(|line| {
            let mut line = line.split_whitespace();
            line.next();
            line.next();
            let last = line
                .next()
                .unwrap()
                .strip_prefix("(#")
                .unwrap()
                .strip_suffix(")")
                .unwrap();
            let step_bytes =
                hex::decode([String::from("0"), last.chars().take(5).collect()].concat()).unwrap();
            let step_bytes = [
                [0].repeat(size_of::<usize>() - step_bytes.len()),
                step_bytes,
            ]
            .concat();
            let step = usize::from_be_bytes(step_bytes.try_into().unwrap());
            let direction_str: String = last.chars().skip(5).take(1).collect();
            let direction = match direction_str.as_str() {
                "0" => Direction::R,
                "1" => Direction::D,
                "2" => Direction::L,
                "3" => Direction::U,
                _ => unreachable!(),
            };
            inner.push(Dig { direction, step });
        });

        // Merge the lines on the same direction to guarantee one direction has only one line
        if inner[0].direction == inner.last().unwrap().direction {
            let first_step = inner.first().unwrap().step;
            let last = inner.last_mut().unwrap();
            last.step += first_step;
            inner = inner[1..].to_vec();
        }

        Self(inner)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn offset(&self, x: isize, y: isize) -> Option<Point> {
        let x = self.x as isize + x;
        let y = self.y as isize + y;
        if x < 0 || y < 0 {
            return None;
        } else {
            Some(Point {
                x: x as usize,
                y: y as usize,
            })
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Line {
    start: Point,
    end: Point,
    direction: Direction,
}

#[derive(Debug)]
struct Edges {
    // key is the x axis of the horizontal lines
    h_lines: Lines,
    // key is the y axis of the vertical lines
    v_lines: Lines,
}

#[derive(Debug)]
struct Lines(HashMap<usize, HashSet<Line>>);

impl Lines {
    fn new() -> Self {
        Self(HashMap::new())
    }
    fn insert(&mut self, axis: usize, line: Line) {
        self.0.entry(axis).or_insert(HashSet::new()).insert(line);
    }
    fn all_lines_cross(&self, range: Range<usize>) -> Vec<&Line> {
        let mut l = Vec::new();
        self.0.iter().for_each(|(_, set)| {
            set.iter().for_each(|line| {
                let line_range;
                match line.direction {
                    Direction::L | Direction::R => {
                        line_range = min(line.start.y, line.end.y)..=max(line.start.y, line.end.y);
                    }
                    Direction::D | Direction::U => {
                        line_range = min(line.start.x, line.end.x)..=max(line.start.x, line.end.x);
                    }
                }
                if line_range.contains(&range.start) && line_range.contains(&range.end) {
                    l.push(line);
                }
            });
        });
        l
    }
}

impl Edges {
    fn new(plan: &Plan) -> Self {
        let mut h_lines = Lines::new();
        let mut v_lines = Lines::new();

        let mut p = (0, 0);
        let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);
        plan.0.iter().for_each(|dig| {
            let step = dig.step as isize;
            let direction = dig.direction;
            match direction {
                Direction::L => {
                    min_y = min(min_y, p.1 - step);
                    p = (p.0, p.1 - step);
                }
                Direction::R => {
                    max_y = max(max_y, p.1 + step);
                    p = (p.0, p.1 + step);
                }
                Direction::U => {
                    min_x = min(min_x, p.0 - step);
                    p = (p.0 - step, p.1);
                }
                Direction::D => {
                    max_x = max(max_x, p.0 + step);
                    p = (p.0 + step, p.1);
                }
            }
        });

        let mut p = Point { x: 0, y: 0 }.offset(-min_x, -min_y).unwrap();
        plan.0.iter().for_each(|dig| {
            let step = dig.step as isize;
            let start = p;
            let end;
            let direction = dig.direction;
            match direction {
                Direction::L | Direction::R => {
                    let step = if direction == Direction::L {
                        -step
                    } else {
                        step
                    };
                    end = start.offset(0, step).unwrap();
                    h_lines.insert(
                        start.x,
                        Line {
                            start,
                            end,
                            direction,
                        },
                    );
                }
                Direction::U | Direction::D => {
                    let step = if direction == Direction::U {
                        -step
                    } else {
                        step
                    };
                    end = start.offset(step, 0).unwrap();
                    v_lines.insert(
                        start.y,
                        Line {
                            start,
                            end,
                            direction,
                        },
                    );
                }
            }
            p = end;
        });

        Self { v_lines, h_lines }
    }

    fn area(&self) -> usize {
        let mut inner_area = 0;
        let mut inner_edge_point = 0;
        let mut hline_axis: Vec<_> = self.h_lines.0.keys().collect();
        hline_axis.sort();
        hline_axis[0..hline_axis.len() - 1]
            .iter()
            .zip(hline_axis[1..].iter())
            .for_each(|(&&top_x, &&bottom_x)| {
                let mut vlines: Vec<_> = self.v_lines.all_lines_cross(top_x..bottom_x);
                vlines.sort_by(|l1, l2| l1.start.y.cmp(&l2.start.y));

                let mut width = 0;
                vlines[..vlines.len() - 1]
                    .iter()
                    .step_by(2)
                    .zip(vlines[1..vlines.len()].iter().step_by(2))
                    .for_each(|(l1, l2)| {
                        let inner_width = l2.start.y - l1.start.y - 1;
                        width += inner_width;

                        // Points that lies at the top/bottom lines that are between the two vlines, but not belong to the top/bottom edge, i.e. inner points.
                        let mut ep = inner_width;
                        self.h_lines.0[&top_x].iter().for_each(|hline| {
                            let h_line_left_y = min(hline.start.y, hline.end.y);
                            let h_line_right_y = max(hline.start.y, hline.end.y);
                            let mut margin_touch = 0;
                            if h_line_left_y >= l1.start.y && h_line_right_y <= l2.start.y {
                                if h_line_left_y == l1.start.y {
                                    margin_touch += 1;
                                }
                                if h_line_right_y == l2.start.y {
                                    margin_touch += 1;
                                }
                                ep -= h_line_right_y - h_line_left_y + 1 - margin_touch;
                            }
                        });
                        inner_edge_point += ep;

                        let mut ep = inner_width;
                        self.h_lines.0[&bottom_x].iter().for_each(|hline| {
                            let h_line_left_y = min(hline.start.y, hline.end.y);
                            let h_line_right_y = max(hline.start.y, hline.end.y);
                            let mut margin_touch = 0;
                            if h_line_left_y >= l1.start.y && h_line_right_y <= l2.start.y {
                                if h_line_left_y == l1.start.y {
                                    margin_touch += 1;
                                }
                                if h_line_right_y == l2.start.y {
                                    margin_touch += 1;
                                }
                                ep -= h_line_right_y - h_line_left_y + 1 - margin_touch;
                            }
                        });
                        inner_edge_point += ep;
                    });
                let inner = width * (bottom_x - top_x - 1);
                inner_area += inner;
            });

        let mut edge_area = 0;
        self.h_lines.0.iter().for_each(|(_, set)| {
            edge_area += set.iter().fold(0, |acc, line| {
                if line.start.y > line.end.y {
                    acc + line.start.y - line.end.y
                } else {
                    acc + line.end.y - line.start.y
                }
            });
        });
        self.v_lines.0.iter().for_each(|(_, set)| {
            edge_area += set.iter().fold(0, |acc, line| {
                if line.start.x > line.end.x {
                    acc + line.start.x - line.end.x
                } else {
                    acc + line.end.x - line.start.x
                }
            });
        });
        inner_area + inner_edge_point / 2 + edge_area
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let content = read_to_string(&args[1]).unwrap();

    let plan = Plan::new(&content);
    let edges = Edges::new(&plan);
    println!("{}", edges.area());

    let plan = Plan::new2(&content);
    let edges = Edges::new(&plan);
    println!("{}", edges.area());
}
