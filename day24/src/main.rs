use std::env;
use std::fs::read_to_string;
use std::str::FromStr;

use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(Debug)]
struct Track {
    x: Decimal,
    y: Decimal,
    z: Decimal,
    dx: Decimal,
    dy: Decimal,
    dz: Decimal,

    xy_k: Option<Decimal>,
    xy_b: Option<Decimal>,
}

impl Track {
    fn new(content: &str) -> Self {
        let [p, v]: [&str; 2] = content.split("@").collect::<Vec<_>>().try_into().unwrap();
        let [x, y, z] = p
            .split(",")
            .map(|e| Decimal::from_str(e.trim()).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let [dx, dy, dz] = v
            .split(",")
            .map(|e| Decimal::from_str(e.trim()).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let xy_k = if dx == dec!(0) { None } else { Some(dy / dx) };
        let xy_b = if let Some(xy_k) = xy_k {
            Some(y - xy_k * x)
        } else {
            None
        };
        Self {
            x,
            y,
            z,
            dx,
            dy,
            dz,
            xy_k,
            xy_b,
        }
    }

    fn xy_joint_in_area(
        &self,
        other: &Self,
        least_pos: Decimal,
        most_pos: Decimal,
    ) -> Option<(Decimal, Decimal)> {
        if self.xy_k == other.xy_k {
            assert!(self.x != other.x);
            return None;
        }
        if self.xy_k.is_none() {
            return other.xy_line_join_vline_in_area(self, least_pos, most_pos);
        }
        if other.xy_k.is_none() {
            return self.xy_line_join_vline_in_area(other, least_pos, most_pos);
        }
        let jx =
            (other.xy_b.unwrap() - self.xy_b.unwrap()) / (self.xy_k.unwrap() - other.xy_k.unwrap());
        let jy = self.xy_k.unwrap() * jx + self.xy_b.unwrap();

        if self.xy_joint_in_now_or_future(jx, jy) && other.xy_joint_in_now_or_future(jx, jy) {
            if Self::xy_point_in_area(jx, jy, least_pos, most_pos) {
                return Some((jx, jy));
            }
        }
        return None;
    }

    fn xy_line_join_vline_in_area(
        &self,
        vline: &Self,
        least_pos: Decimal,
        most_pos: Decimal,
    ) -> Option<(Decimal, Decimal)> {
        assert!(self.xy_k.is_some());
        let jx = vline.x;
        let jy = self.xy_k.unwrap() * jx + self.xy_b.unwrap();
        if self.xy_joint_in_now_or_future(jx, jy) && vline.xy_joint_in_now_or_future(jx, jy) {
            if Self::xy_point_in_area(jx, jy, least_pos, most_pos) {
                return Some((jx, jy));
            }
        }
        return None;
    }

    fn xy_joint_in_now_or_future(&self, jx: Decimal, jy: Decimal) -> bool {
        (jx - self.x) * self.dx >= dec!(0) && (jy - self.y) * self.dy >= dec!(0)
    }

    fn xy_point_in_area(x: Decimal, y: Decimal, least_pos: Decimal, most_pos: Decimal) -> bool {
        least_pos.le(&x) && least_pos.le(&y) && most_pos.ge(&x) && most_pos.ge(&y)
    }
}

#[derive(Debug)]
struct Tracks(Vec<Track>);

impl Tracks {
    fn new(content: &str) -> Self {
        let mut v = Vec::new();
        content.lines().for_each(|line| {
            v.push(Track::new(line));
        });
        Self(v)
    }

    fn xy_joints_in_area(&self, least_pos: Decimal, most_pos: Decimal) -> Vec<(Decimal, Decimal)> {
        let mut jp = Vec::new();
        for i in 0..self.0.len() - 1 {
            for j in i + 1..self.0.len() {
                let t1 = &self.0[i];
                let t2 = &self.0[j];
                if let Some((jx, jy)) = t1.xy_joint_in_area(&t2, least_pos, most_pos) {
                    jp.push((jx, jy));
                }
            }
        }
        jp
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let content = read_to_string(&args[1]).unwrap();
    let tracks = Tracks::new(&content);
    //let joints = tracks.xy_joints_in_area(dec!(7), dec!(27));
    let joints = tracks.xy_joints_in_area(dec!(200000000000000), dec!(400000000000000));
    println!("{}", joints.len());

    // (dy'-dy) X + (dx-dx') Y + (y-y') DX + (x'-x) DY = x' dy' - y' dx' - x dy + y dx
    // Calculate the known parts with the first 4 pairs, then use online linear system of equation
    // solver (e.g. https://www.wolframalpha.com/calculators/system-equation-calculator)
    tracks
        .0
        .iter()
        .take(4)
        .zip(tracks.0.iter().skip(1).take(4))
        .enumerate()
        .for_each(|(i, (t1, t2))| {
            let c1 = t2.dy - t1.dy;
            let c2 = t1.dx - t2.dx;
            let c3 = t1.y - t2.y;
            let c4 = t2.x - t1.x;
            let c5 = -(t2.x * t2.dy - t2.y * t2.dx - t1.x * t1.dy + t1.y * t1.dx);
            println!("{}x + ({})y + ({})a + ({})b + ({}) = 0", c1, c2, c3, c4, c5);
        });

    tracks
        .0
        .iter()
        .take(4)
        .zip(tracks.0.iter().skip(1).take(4))
        .for_each(|(t1, t2)| {
            let c1 = t2.dz - t1.dz;
            let c2 = t1.dx - t2.dx;
            let c3 = t1.z - t2.z;
            let c4 = t2.x - t1.x;
            let c5 = -(t2.x * t2.dz - t2.z * t2.dx - t1.x * t1.dz + t1.z * t1.dx);
            println!("{}x + ({})y + ({})a + ({})b + ({}) = 0", c1, c2, c3, c4, c5);
        });
}
