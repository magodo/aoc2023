use geo::coord;
use geo::Intersects;
use geo::Line;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Brick {
    xy: Line<i64>,
    z: RangeInclusive<i64>,
    supporter: Vec<usize>,
}

#[derive(Debug)]
struct Bricks(Vec<Brick>);

impl Bricks {
    fn new(content: &str) -> Self {
        let mut bricks = Vec::new();
        content.lines().for_each(|line| {
            let coordinates: Vec<_> = line.split("~").collect();
            let coord1: Vec<i64> = coordinates[0]
                .split(",")
                .map(|e| e.parse().unwrap())
                .collect();
            let coord2: Vec<i64> = coordinates[1]
                .split(",")
                .map(|e| e.parse().unwrap())
                .collect();
            let z_start = min(coord1[2], coord2[2]);
            let z_end = max(coord1[2], coord2[2]);
            let b = Brick {
                xy: Line::new(
                    coord! {x: coord1[0], y: coord1[1]},
                    coord! {x: coord2[0], y: coord2[1]},
                ),
                z: z_start..=z_end,
                supporter: Vec::new(),
            };
            // Ensure all the lines that have different start Z and end Z, they are vertical lines.
            if b.z.start() != b.z.end() {
                assert!(b.xy.start == b.xy.end);
            }
            bricks.push(b);
        });
        bricks.sort_by(|b1, b2| b1.z.start().cmp(b2.z.start()));
        Bricks(bricks)
    }

    fn fall(&mut self) {
        // z.start -> brick indexes
        let mut m: HashMap<i64, Vec<usize>> = HashMap::new();
        for i in 0..self.0.len() {
            let mut hit = false;
            for j in (0..*self.0[i].z.start()).rev() {
                if hit {
                    break;
                }
                // hit ground
                if j == 0 {
                    // update brick
                    let brick = self.0.get_mut(i).unwrap();
                    brick.z = 1..=(brick.z.end() - (brick.z.start() - 1));
                    break;
                }
                if let Some(brick_indexes) = m.get(&j) {
                    for idx in brick_indexes {
                        let candidate_supporter = &self.0[*idx];
                        let brick = &self.0[i];
                        let z_offset = *brick.z.start() - 1 - j;

                        if !candidate_supporter.xy.intersects(&brick.xy) {
                            continue;
                        }

                        // update brick
                        let brick = self.0.get_mut(i).unwrap();
                        brick.z = (brick.z.start() - z_offset)..=(brick.z.end() - z_offset);
                        brick.supporter.push(*idx);
                        hit = true;
                    }
                }
            }
            let brick = &self.0[i];
            m.entry(*brick.z.end()).or_insert(Vec::new()).push(i);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let content = read_to_string(&args[1]).unwrap();
    let mut bricks = Bricks::new(&content);
    bricks.fall();

    let critical_supporters = bricks
        .0
        .iter()
        .filter(|e| e.supporter.len() == 1)
        .map(|e| e.supporter[0])
        .collect::<HashSet<_>>();
    let non_critical_supporters: Vec<_> = (0..bricks.0.len())
        .filter(|idx| !critical_supporters.contains(idx))
        .collect();
    let p1 = non_critical_supporters.len();
    println!("{}", p1);

    let mut p2 = 0;
    bricks.0.iter().enumerate().for_each(|(idx, _)| {
        let mut set: HashSet<usize> = HashSet::from_iter([idx]);
        loop {
            let old_set = set.clone();
            bricks
                .0
                .iter()
                .enumerate()
                .filter(|(_, b)| {
                    b.supporter.len() != 0 && b.supporter.iter().all(|sup| old_set.contains(sup))
                })
                .for_each(|(idx, _)| {
                    set.insert(idx);
                });

            if old_set.len() == set.len() {
                break;
            }
        }
        p2 += set.len() - 1;
    });
    println!("{}", p2);
}
