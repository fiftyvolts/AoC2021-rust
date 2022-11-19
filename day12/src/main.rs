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
    part12(&input);
}

struct Node {
    id: String,
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

    fn get<'a>(&'a self, id: &str) -> &'a Node {
        &self.0.get(id).unwrap()
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
        Node {
            id: id,
            peers: Vec::new(),
        }
    }
}

fn part12(input: &str) {

    let path: &mut Vec<String> = &mut vec![];
    let mut paths: &mut Vec<Vec<String>> = &mut vec![];
    let graph = &mut Graph::from(String::from(input));

    (_, paths, _) = visit(path, paths, graph, graph.start().id.clone());
    println!("{}",paths.len());
}

fn visit<'a>(
    mut path: &'a mut Vec<String>,
    mut paths: &'a mut Vec<Vec<String>>,
    mut graph: &'a mut Graph,
    id: String,
) -> (&'a mut Vec<String>, &'a mut Vec<Vec<String>>, &'a mut Graph) {
    path.push(id.clone());

    if id == graph.end().id {
        paths.push(path.clone());
    } else {
        let peers = graph.get(&id).peers.clone();
        for peer in peers {
            if !(peer.to_lowercase() == *peer && path.contains(&peer)) {
                (path, paths, graph) = visit(path, paths, graph, peer);
            }
        }
    }

    path.pop();

    (path, paths, graph)
}
