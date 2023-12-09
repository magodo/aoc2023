use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::hash::Hash;

#[derive(Debug)]
struct Graph<NId> {
    edges: HashMap<NId, Edge<NId>>,
}

impl<NId> Graph<NId>
where
    NId: Eq + Hash,
{
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    fn insert_edge(self: &mut Self, from: NId, edge: Edge<NId>) {
        self.edges.insert(from, edge);
    }

    fn walk<P>(self: &Self, from: NId, direction: &str, f: P) -> u64
    where
        P: Fn(&NId) -> bool,
    {
        let mut edge = self.edges.get(&from).unwrap();
        let mut step = 0;
        'outer: loop {
            for c in direction.chars() {
                match c {
                    'L' => {
                        step += 1;
                        let node = &edge.l;
                        if f(&node) {
                            break 'outer;
                        }
                        edge = self.edges.get(&node).unwrap();
                    }
                    'R' => {
                        step += 1;
                        let nid = &edge.r;
                        if f(&nid) {
                            break 'outer;
                        }
                        edge = self.edges.get(&nid).unwrap();
                    }
                    _ => panic!("unreachable"),
                };
            }
        }
        return step;
    }
}

#[derive(Debug)]
struct Edge<NId> {
    l: NId,
    r: NId,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }

    let content = read_to_string(&args[1]).unwrap();
    let mut lines = content.lines();
    let direction = lines.next().unwrap();

    lines.next();
    let mut graph = Graph::new();
    lines.for_each(|line| {
        let split: Vec<&str> = line.split("=").collect();
        let nid = split[0].trim();
        let split: Vec<&str> = split[1]
            .trim()
            .trim_matches(['(', ')'].as_slice())
            .split(",")
            .collect();
        let edge = Edge {
            l: split[0].trim(),
            r: split[1].trim(),
        };
        graph.insert_edge(nid, edge);
    });

    // part 1
    let step = graph.walk("AAA", direction, |nid| *nid == "ZZZ");
    println!("{}", step);

    // part2
    let nodes: Vec<&str> = graph
        .edges
        .keys()
        .filter(|nid| nid.ends_with("A"))
        .map(|e| *e)
        .collect();
    let step = nodes
        .iter()
        .map(|node| graph.walk(*node, direction, |nid| nid.ends_with("Z")))
        .fold(1, |acc, x| lcm(acc, x as usize));
    println!("{}", step);
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
