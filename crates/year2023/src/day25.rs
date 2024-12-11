use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    vec,
};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Edge<'a>([&'a str; 2]);

impl<'a> Edge<'a> {
    fn new(mut nodes: [&'a str; 2]) -> Self {
        assert_ne!(nodes[0], nodes[1]);
        nodes.sort();
        Self(nodes)
    }
}

#[derive(Debug, Clone)]
struct Graph<'a> {
    nodes: BTreeMap<&'a str, BTreeSet<&'a str>>,
}
impl<'a> Graph<'a> {
    fn remove_edge(&mut self, edge: Edge<'_>) {
        self.nodes.get_mut(edge.0[0]).unwrap().remove(edge.0[1]);
        self.nodes.get_mut(edge.0[1]).unwrap().remove(edge.0[0]);
    }

    fn add_edge(&mut self, edge: Edge<'a>) {
        self.nodes.get_mut(edge.0[0]).unwrap().insert(edge.0[1]);
        self.nodes.get_mut(edge.0[1]).unwrap().insert(edge.0[0]);
    }

    fn dijkstra(&self, start: &'a str, target: &str) -> Option<Vec<Edge<'a>>> {
        let mut visited = HashSet::from([start]);
        let mut paths: BTreeMap<&str, Vec<Edge<'_>>> = BTreeMap::from([(start, vec![])]);
        let mut todos = vec![start];
        while !todos.is_empty() {
            for node in std::mem::take(&mut todos) {
                let path = paths.remove(node).unwrap().clone();
                for &neighbor in &self.nodes[node] {
                    if visited.insert(neighbor) {
                        todos.push(neighbor);
                        let mut path = path.clone();
                        path.push(Edge::new([node, neighbor]));
                        if neighbor == target {
                            return Some(path);
                        }
                        paths.insert(neighbor, path);
                    }
                }
            }
        }
        None
    }
}

fn parse_input(input: &str) -> (Graph<'_>, Vec<Edge<'_>>) {
    let edges: Vec<Edge<'_>> = input
        .lines()
        .flat_map(|line| {
            let (src, dests) = line.split_once(": ").unwrap();
            dests.split(' ').map(|dest| Edge::new([src, dest]))
        })
        .collect();

    let mut nodes: BTreeMap<&str, BTreeSet<_>> = BTreeMap::new();

    for edge in &edges {
        nodes.entry(edge.0[0]).or_default().insert(edge.0[1]);
        nodes.entry(edge.0[1]).or_default().insert(edge.0[0]);
    }
    (Graph { nodes }, edges)
}

pub fn part1(input: &str) -> usize {
    let (mut graph, edges) = parse_input(input);

    println!("{}: ", edges.len());
    for i1 in 0..edges.len() {
        let e1 = edges[i1];
        graph.remove_edge(e1);
        // find a shortest path between the nodes we just cut the edge between,
        // the other two cuts need to be on that path to partition the graph
        if let Some(path) = graph.dijkstra(e1.0[0], e1.0[1]) {
            for e2 in path {
                graph.remove_edge(e2);
                // find another shortest path with the same reasoning
                if let Some(path) = graph.dijkstra(e1.0[0], e1.0[1]) {
                    for e3 in path {
                        graph.remove_edge(e3);
                        if let Some(count) = bi_components(&graph, [e1, e2, e3]) {
                            return count;
                        }
                        graph.add_edge(e3);
                    }
                }
                graph.add_edge(e2);
            }
        }
        graph.add_edge(e1);
    }

    unreachable!()
}

fn bi_components(graph: &Graph<'_>, [e1, e2, e3]: [Edge<'_>; 3]) -> Option<usize> {
    let Some(c1) = collect_component(&graph, e1.0[0], e1.0[1]) else {
        return None;
    };

    if c1.contains(e2.0[0]) == c1.contains(e2.0[1]) || c1.contains(e3.0[0]) == c1.contains(e3.0[1])
    {
        println!("Containment");
        return None;
    }

    let c2 = collect_component(&graph, e1.0[1], e1.0[0]).unwrap();

    if c1.len() + c2.len() == graph.nodes.len() {
        return Some(c1.len() * c2.len());
    }
    println!("Count");
    None
}

fn collect_component<'a>(
    graph: &Graph<'a>,
    start_node: &'a str,
    target_node: &str,
) -> Option<BTreeSet<&'a str>> {
    let mut visited = BTreeSet::from([start_node]);
    let mut todo = vec![start_node];

    while let Some(node) = todo.pop() {
        for neighbor in graph.nodes[node].iter().copied() {
            if neighbor == target_node {
                return None;
            }
            if visited.insert(neighbor) {
                todo.push(neighbor);
            }
        }
    }
    Some(visited)
}

#[test]
fn part1_example() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/example/year2023/day25.example.txt"
    ));
    assert_eq!(part1(input), 9 * 6);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/personal/year2023/day25.txt"
    ));
    assert_eq!(part1(input), 520380);
}
