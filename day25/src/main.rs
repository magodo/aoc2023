use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
struct Graph {
    edges: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new(content: &str) -> Self {
        let mut edges = HashMap::new();
        content.lines().for_each(|line| {
            let [name, children]: [&str; 2] = line
                .split(":")
                .map(|e| e.trim())
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();
            children.split_whitespace().map(|e| e.trim()).for_each(|c| {
                edges
                    .entry(name.to_string())
                    .or_insert(HashSet::new())
                    .insert(c.to_string());

                edges
                    .entry(c.to_string())
                    .or_insert(HashSet::new())
                    .insert(name.to_string());
            });
        });
        Self { edges }
    }

    fn union_node(&self, n: &String, set: &mut HashSet<String>) {
        if set.contains(n) {
            return;
        }
        set.insert(n.to_string());
        if let Some(edge) = self.edges.get(n) {
            edge.iter().for_each(|n| {
                self.union_node(n, set);
            });
        }
    }

    fn partitions(&self) -> Vec<HashSet<String>> {
        let mut p: Vec<HashSet<String>> = Vec::new();

        self.edges.iter().for_each(|(n, _)| {
            if p.iter().any(|set| set.contains(n)) {
                return;
            }
            let mut set = HashSet::new();
            self.union_node(n, &mut set);
            p.push(set);
        });

        p
    }
}

fn dot(content: &str) -> String {
    let mut diag = "digraph G {\n".to_string();
    let mut m = HashSet::new();
    content.lines().for_each(|line| {
        let [name, children]: [&str; 2] = line
            .split(":")
            .map(|e| e.trim())
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        children.split_whitespace().map(|e| e.trim()).for_each(|c| {
            if m.contains(&format!("{}-{}", name, c).to_string())
                || m.contains(&format!("{}-{}", c, name).to_string())
            {
                return;
            }
            diag += format!("  {} -> {} [label=\"{}-{}\"]\n", name, c, name, c).as_str();
            m.insert(format!("{}-{}", name, c).to_string());
        });
    });
    diag += "}";
    diag
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("./exe <file>");
    }
    let content = read_to_string(&args[1]).unwrap();
    let mut graph = Graph::new(&content);
    dbg!((graph.edges.iter().fold(0, |acc, preds| acc + preds.0.len())) / 2);
    //println!("{}", dot(&content));
    // Using dot I can clearly see the 3 lines:
    // gbc-hxr
    // xkz-mvv
    // tmt-pnz
    // for (n1, n2) in [("cmg", "bvb"), ("jqt", "nvd"), ("hfx", "pzl")] {
    for (n1, n2) in [("gbc", "hxr"), ("xkz", "mvv"), ("tmt", "pnz")] {
        if let Some(set) = graph.edges.get_mut(n1) {
            set.remove(n2);
        }
        if let Some(set) = graph.edges.get_mut(n2) {
            set.remove(n1);
        }
    }
    let ps = graph.partitions();
    assert_eq!(ps.len(), 2);
    println!("{}", ps[0].len() * ps[1].len());
}
