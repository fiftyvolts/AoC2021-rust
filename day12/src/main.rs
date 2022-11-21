use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    io::{stdin, Read},
};

fn input_txt() -> String {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).ok();
    buf
}

fn main() {
    let input = input_txt();

    part12(&input, 1);
    part12(&input, 2);
}
#[derive(Debug)]
struct Node {
    id: String,
    small: bool,
    termination: bool,
    visit_count: i32,
    peers: Vec<String>,
}

struct Graph(HashMap<String, Node>);

impl Graph {
    fn new() -> Graph {
        Graph(HashMap::new())
    }

    fn start(&self) -> &Node {
        self.0.get("start").unwrap()
    }

    fn end(&self) -> &Node {
        self.0.get("end").unwrap()
    }

    fn add_edge(&mut self, a: &str, b: &str) {
        {
            let na = self
                .0
                .entry(String::from(a))
                .or_insert(Node::new(String::from(a)));
            na.peers.push(String::from(b));
        }
        {
            let nb = self
                .0
                .entry(String::from(b))
                .or_insert(Node::new(String::from(b)));
            nb.peers.push(String::from(a));
        }
    }

    fn get(&self, id: &str) -> &Node {
        &self.0.get(id).unwrap()
    }

    fn get_mut(&mut self, id: &str) -> &mut Node {
        self.0.get_mut(id).unwrap()
    }
    fn max_small_visits(&self) -> i32 {
        self.0
            .iter()
            .filter(|e| e.1.small)
            .map(|e| e.1.visit_count)
            .max()
            .unwrap()
    }
}

impl Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines: Vec<String> = vec![];
        let mut stack: Vec<String> = vec![self.start().id.clone()];
        let mut added: HashSet<String> = HashSet::new();

        while stack.len() > 0 {
            let node = self.get(&stack.pop().unwrap());
            for p in node.peers.iter() {
                if !added.contains(p) {
                    if node.id == "start"
                        || (node.id != "end") && node.id.to_lowercase() < p.to_lowercase()
                    {
                        lines.push(format!("{}-{}", node.id, p));
                    } else {
                        lines.push(format!("{}-{}", p, node.id))
                    }
                    stack.push(p.clone());
                }
            }
            added.insert(node.id.clone());
        }

        lines.sort();
        f.write_fmt(format_args!("{}", lines.join("\n"))).ok();
        Ok(())
    }
}

impl From<String> for Graph {
    fn from(input: String) -> Graph {
        let mut graph = Graph::new();
        for line in input.lines() {
            let pair: Vec<&str> = line.split("-").collect();
            graph.add_edge(pair[0], pair[1]);
        }
        graph
    }
}

impl Node {
    fn new(id: String) -> Node {
        let small = id.to_lowercase() == id;
        let termination : bool = id.eq("start") || id.eq("end");
        Node {
            id,
            small,
            termination,
            visit_count: 0,
            peers: vec![],
        }
    }
}

fn part12(input: &str, small_visits: i32) {
    let path: &mut Vec<String> = &mut vec![];
    let paths: &mut Vec<Vec<String>> = &mut vec![];
    let graph = &mut Graph::from(String::from(input));
    visit(path, paths, graph, graph.start().id.clone(), small_visits);
    println!("{}", paths.len());
}

fn visit(
    path: &mut Vec<String>,
    paths: &mut Vec<Vec<String>>,
    graph: &mut Graph,
    id: String,
    small_visits: i32,
) {
    path.push(id.clone());
    graph.get_mut(&id).visit_count += 1;

    if id == graph.end().id {
        paths.push(path.clone());
    } else {
        let peers = graph.get(&id).peers.clone();
        for peer in peers {
            let pn = graph.get(&peer);
            
            if !pn.small || pn.visit_count < 1 || (!pn.termination && graph.max_small_visits() < small_visits) {
                visit(path, paths, graph, peer, small_visits);
            }
        }
    }

    graph.get_mut(&id).visit_count -= 1;
    path.pop();
}
