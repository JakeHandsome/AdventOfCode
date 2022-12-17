use common::*;
use itertools::*;
use petgraph::{algo::dijkstra, stable_graph::NodeIndex, Graph};
use std::{collections::HashMap, fmt::Debug};

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

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: u64,
}

trait GraphExtension {
    fn create_graph(valves: Vec<Valve>, valve_connections: HashMap<String, Vec<String>>) -> Graph<Valve, usize>;
}

impl GraphExtension for Graph<Valve, usize> {
    fn create_graph(valves: Vec<Valve>, valve_connections: HashMap<String, Vec<String>>) -> Graph<Valve, usize> {
        let mut graph = Graph::<Valve, usize>::new();
        let mut indexes = HashMap::new();
        for valve in valves.into_iter() {
            let name = valve.name.clone();
            let index = graph.add_node(valve);
            indexes.insert(name, index);
        }
        for (valve_name, edges) in valve_connections {
            for edge in edges {
                graph.add_edge(
                    indexes.get(&valve_name).unwrap().to_owned(),
                    indexes.get(&edge).unwrap().to_owned(),
                    1,
                );
            }
        }
        graph
    }
}

fn get_one_and_slice(vec: &Vec<NodeIndex>, index: usize) -> (NodeIndex, Vec<NodeIndex>) {
    let mut vec = vec.to_owned();
    let a = vec.remove(index);
    (a, vec)
}

struct Solver {
    solutions: HashMap<Vec<NodeIndex>, u64>,
    graph: Graph<Valve, usize>,
    distances: HashMap<NodeIndex, HashMap<NodeIndex, usize>>,
}
impl Solver {
    fn new(graph: Graph<Valve, usize>, distances: HashMap<NodeIndex, HashMap<NodeIndex, usize>>) -> Self {
        Solver {
            solutions: HashMap::new(),
            graph,
            distances,
        }
    }
    fn solve(
        &mut self,
        current: NodeIndex,
        remaining: Vec<NodeIndex>,
        steps: isize,
        total_flow: isize,
        visited: Vec<NodeIndex>,
    ) {
        let current_node = self.graph.node_weight(current).unwrap();
        let mut steps = steps;
        let mut total_flow = total_flow;
        let mut visited = visited;
        if steps <= 0 {
            self.solutions.insert(visited.clone(), total_flow as u64);
        }
        visited.push(current);
        if current_node.flow_rate > 0 && steps > 0 {
            steps -= 1;
            total_flow += steps * current_node.flow_rate as isize;
        }
        if steps <= 0 {
            self.solutions.insert(visited.clone(), total_flow as u64);
        }
        if steps > 0 && !remaining.is_empty() {
            for i in 0..remaining.len() {
                let (next, next_v) = get_one_and_slice(&remaining, i);
                let _next_node = self.graph.node_weight(next).unwrap();
                let next_distance = self.distances[&current][&next];
                self.solve(
                    next,
                    next_v,
                    steps - next_distance as isize,
                    total_flow,
                    visited.clone(),
                )
            }
        } else {
            self.solutions.insert(visited, total_flow as u64);
        }
    }
}

fn part1(input: &str) -> R<u64> {
    let mut valves = vec![];
    let mut valve_connections = HashMap::new();
    for line in input.lines() {
        let name = line.split(' ').nth(1).unwrap().to_string();
        let flow_rate: u64 = line.split(';').next().unwrap().split('=').last().unwrap().parse()?;
        let edges = line
            .split(' ')
            .skip(9)
            .map(|x| x[0..2].to_string())
            .collect::<Vec<String>>();

        valve_connections.insert(name.clone(), edges);
        valves.push(Valve { name, flow_rate });
    }

    let graph = Graph::create_graph(valves, valve_connections);

    #[cfg(not)]
    {
        #[cfg(test)]
        let file_name = "day16_sample.dot";
        #[cfg(not(test))]
        let file_name = "day16_real.dot";

        let mut file = std::fs::File::create(file_name)?;
        file.write_fmt(format_args!(
            "{:?}",
            Dot::with_config(&graph, &[Config::_Incomplete(())])
        ))?;
    }

    let mut distances = HashMap::new();
    for a in graph.node_indices().filter(|x| {
        let w = graph.node_weight(*x).unwrap();
        w.flow_rate > 0 || w.name == "AA"
    }) {
        let distance = dijkstra(&graph, a, None, |x| *x.weight())
            .into_iter()
            .filter(|(x, _size)| {
                let w = graph.node_weight(*x).unwrap();
                w.flow_rate > 0 || w.name == "AA"
            })
            .collect::<HashMap<_, _>>();
        distances.insert(a, distance);
    }

    // Print distances chart
    #[cfg(not)]
    {
        print!("----- ");
        for key in distances.keys().sorted() {
            print!("{:3}, ", graph.node_weight(*key).unwrap().name);
        }
        println!();
        for col in distances.keys().sorted() {
            print!("{:4}: ", graph.node_weight(*col).unwrap().name);
            for row in distances[col].keys().sorted() {
                print!("{:3}, ", distances[col][row]);
            }
            println!()
        }
    }

    let mut distance_vals = vec![];
    for values in distances.values() {
        for values2 in values.values() {
            distance_vals.push(values2);
        }
    }

    let start = graph
        .node_indices()
        .find(|x| graph.node_weight(x.to_owned()).unwrap().name == "AA")
        .unwrap();
    let all_paths = distances
        .keys()
        .into_iter()
        .filter(|x| graph.node_weight(**x).unwrap().name != "AA")
        .map(|x| *x)
        .collect_vec();

    let graph2 = graph.clone();
    let mut solver = Solver::new(graph, distances);
    solver.solve(start, all_paths, 30, 0, vec![]);

    let max = *solver.solutions.values().max().unwrap();
    let (path, _) = solver.solutions.iter().find(|(_, v)| **v == max).unwrap();
    for node in path {
        print!("{:} ,", graph2.node_weight(*node).unwrap().name);
    }

    Ok(max)
}

fn part2(_input: &str) -> R<u64> {
    Err(Box::new(AdventOfCodeError::new("Not implemented")))
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 1651);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 1707);
    }
}
