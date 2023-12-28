use std::{collections::HashMap, io::Write};

use common::{
    petgraph::{algo::toposort, dot::Dot, Graph},
    petgraph::{data::DataMap, prelude::*},
    *,
};

fn main() {
    let input = read_input_file_for_project_as_string!();
    {
        let _timer = Timer::new("Part 1");
        println!("Part1: {}", part1(&input).unwrap());
    }
    {
        let _timer = Timer::new("Part 2");
        println!("Part2: {}", part2(&input).unwrap());
    }
}

struct Input {
    inner: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Input {
    fn new(input: &str) -> Self {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().chars().count();
        let inner = input.replace('\n', "").chars().collect();
        Self { inner, rows, cols }
    }

    #[inline]
    fn index(&self, row: isize, col: isize) -> Option<usize> {
        if row < 0 || col < 0 || row >= self.rows as isize || col >= self.cols as isize {
            None
        } else {
            let index = (row * self.cols as isize + col) as usize;
            debug_assert!(index < self.inner.len(), "{},r{},c{}", index, row, col);
            Some(index)
        }
    }

    #[inline]
    fn get_edge(&self, row: isize, col: isize) -> Option<char> {
        self.index(row, col).map(|index| self.inner[index])
    }
}
fn part1(input: &str) -> anyhow::Result<isize> {
    let map = Input::new(input);
    let mut graph = Graph::<(isize, isize), isize>::new();
    // Determine start position
    //  - Start at row0 col1
    let start = graph.add_node((0, 1));
    // Determine end position
    graph.add_node((map.rows as isize - 1, map.cols as isize - 2));
    // Build directed graph
    // - Find all nodes
    add_all_nodes(&mut graph, &map);
    // - Walk the output for all nodes to find each edge
    add_all_edges(&mut graph, &map);
    let dot = Dot::new(&graph);
    std::fs::File::create("day23pt1.dot")?.write_all(format!("{dot:?}").as_bytes())?;
    // Use modified Djisktras to find longest path
    let mut distances = HashMap::new();
    // https://www.geeksforgeeks.org/find-longest-path-directed-acyclic-graph/
    for b in graph.node_indices() {
        // But -negative inf for all nodes
        distances.entry(b).or_insert(isize::MIN);
    }
    // Set 0 for the start node
    distances.insert(start, 0);
    // Go in topographical order and adjust the distance from each node
    for b in toposort(&graph, None).unwrap() {
        for edge in graph.edges_directed(b, Direction::Outgoing) {
            if distances[&edge.target()] < distances[&edge.source()] + edge.weight() {
                distances.insert(edge.target(), distances[&edge.source()] + edge.weight());
            }
        }
    }
    // Max distance is the solution
    Ok(*distances.values().max().unwrap())
}

fn add_all_nodes(graph: &mut Graph<(isize, isize), isize>, map: &Input) {
    for r in 1..map.rows as isize - 1 {
        for c in 1..map.cols as isize - 1 {
            // Make sure current node is not a '#'
            match map.get_edge(r, c) {
                Some('#') | None => (),
                Some(_) => {
                    // If there are 3 or more paths (non '#' spaces) around this position then it is a node
                    let paths = [
                        map.get_edge(r - 1, c),
                        map.get_edge(r + 1, c),
                        map.get_edge(r, c - 1),
                        map.get_edge(r, c + 1),
                    ]
                    .into_iter()
                    .flatten()
                    .filter(|x| *x != '#')
                    .count();
                    if paths >= 3 {
                        graph.add_node((r, c));
                    }
                }
            }
        }
    }
}

fn add_all_edges(graph: &mut Graph<(isize, isize), isize>, map: &Input) {
    for source in graph.node_indices() {
        let (r, c) = graph.node_weight(source).unwrap();
        let mut edges = vec![];
        if let Some('>') = map.get_edge(*r, *c + 1) {
            let (distance, edge) = walk_until_node((*r, *c + 1), (*r, *c), 1, graph, map);
            let dest = graph
                .node_indices()
                .find(|x| graph.node_weight(*x).unwrap() == &edge)
                .unwrap();
            edges.push((source, dest, distance));
        }
        if let Some('v') | Some('.') = map.get_edge(*r + 1, *c) {
            let (distance, edge) = walk_until_node((*r + 1, *c), (*r, *c), 1, graph, map);
            let dest = graph
                .node_indices()
                .find(|x| graph.node_weight(*x).unwrap() == &edge)
                .unwrap();
            edges.push((source, dest, distance));
        }
        for (source, dest, distance) in edges {
            graph.update_edge(source, dest, distance);
        }
    }
}

fn add_all_edges2(graph: &mut Graph<(isize, isize), isize>, map: &Input) {
    for source in graph.node_indices() {
        let (r, c) = graph.node_weight(source).unwrap();
        let mut edges = vec![];
        for (r2, c2) in [(*r + 1, *c), (*r - 1, *c), (*r, *c + 1), (*r, *c - 1)] {
            if let Some('v') | Some('.') | Some('>') = map.get_edge(r2, c2) {
                let (distance, edge) = walk_until_node((r2, c2), (*r, *c), 1, graph, map);
                let dest = graph
                    .node_indices()
                    .find(|x| graph.node_weight(*x).unwrap() == &edge)
                    .unwrap();
                edges.push((source, dest, distance));
            }
        }
        for (source, dest, distance) in edges {
            graph.update_edge(source, dest, distance);
        }
    }
}

fn walk_until_node(
    current: (isize, isize),
    previous: (isize, isize),
    distance: isize,
    graph: &Graph<(isize, isize), isize>,
    map: &Input,
) -> (isize, (isize, isize)) {
    if graph.raw_nodes().iter().any(|x| x.weight == current) {
        (distance, current)
    } else {
        let (r, c) = current;
        let next = [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
            .into_iter()
            // Remove previous entry
            .filter(|x| *x != previous)
            // Get the next item that has is not '#'
            .filter(|(r, c)| match map.get_edge(*r, *c) {
                Some('#') | None => false,
                Some(_) => true,
            })
            // Should only be a single item left
            .at_most_one()
            .unwrap()
            .unwrap();
        walk_until_node(next, current, distance + 1, graph, map)
    }
}

fn part2(input: &str) -> anyhow::Result<isize> {
    let map = Input::new(input);
    let mut graph = Graph::<(isize, isize), isize>::new();
    // Determine start position
    //  - Start at row0 col1
    let start = graph.add_node((0, 1));
    // Determine end position
    let end = graph.add_node((map.rows as isize - 1, map.cols as isize - 2));
    // Build directed graph
    // - Find all nodes
    add_all_nodes(&mut graph, &map);
    // - Walk the output for all nodes to find each edge
    add_all_edges2(&mut graph, &map);
    let dot = Dot::new(&graph);
    std::fs::File::create("day23pt2.dot")?.write_all(format!("{dot:?}").as_bytes())?;
    let mut visited = HashMap::new();
    let mut dist = HashMap::new();
    for x in graph.node_indices() {
        visited.insert(x, false);
        dist.insert(x, 0);
    }
    longest_path(&graph, start, &mut visited, &mut dist, 0);
    Ok(dist[&end])
}

fn longest_path(
    graph: &Graph<(isize, isize), isize>,
    current: NodeIndex,
    visited: &mut HashMap<NodeIndex, bool>,
    dist: &mut HashMap<NodeIndex, isize>,
    current_dist: isize,
) {
    if visited[&current] {
        return;
    }
    visited.insert(current, true);
    if dist[&current] < current_dist {
        dist.insert(current, current_dist);
    }
    for next in graph.neighbors(current) {
        longest_path(
            graph,
            next,
            visited,
            dist,
            current_dist + graph.edge_weight(graph.find_edge(current, next).unwrap()).unwrap(),
        );
    }
    visited.insert(current, false);
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 94);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 154);
    }
}
