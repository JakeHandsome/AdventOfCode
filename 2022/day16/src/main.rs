use common::*;
use itertools::*;
use petgraph::{
    algo::dijkstra,
    data::DataMap,
    dot::{Config, Dot},
    graph::{self, Edge},
    stable_graph::NodeIndex,
    visit::{EdgeIndexable, GraphProp, IntoNodeIdentifiers},
    Graph,
};
use rayon::{current_num_threads, prelude::*};
use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    io::Write,
    time::SystemTime,
};
use thousands::Separable;

struct State {
    minute: usize,
    total_flow: u64,
    pressure: u64,
    log: bool,
}

impl State {
    fn new(log: bool) -> Self {
        State {
            minute: 1,
            total_flow: 0,
            pressure: 0,
            log,
        }
    }
    fn increment_time(&mut self) {
        if self.log {
            println!(
                "Minute: {}, pressure: {}, flow:{}",
                self.minute, self.pressure, self.total_flow
            );
        }
        self.total_flow += self.pressure;
        self.minute += 1;
    }
    fn increase_flow(&mut self, flow: u64) {
        self.pressure += flow;
    }

    fn open_valve(&mut self, graph: &Graph<&Valve, usize>, i: NodeIndex) {
        let node = graph.node_weight(i).unwrap();
        let flow_rate = node.flow_rate;
        if flow_rate > 0 {
            // Turn on this node
            self.increment_time();
            self.increase_flow(flow_rate);
            if self.log {
                println!("Opened valve {} at {}", node.name, self.minute);
            }
        }
    }
    fn step(
        &mut self,
        graph: &Graph<&Valve, usize>,
        path: &Vec<&NodeIndex>,
        distances: &HashMap<NodeIndex, HashMap<NodeIndex, usize>>,
    ) {
        let mut process_last = true;
        let iter = path.windows(2);
        let mut j = 0;
        for (i, a) in iter.enumerate() {
            let current = *a[0];
            let current_node = graph.node_weight(current).unwrap();
            let next = *a[1];
            let next_node = graph.node_weight(next).unwrap();
            self.open_valve(graph, current);
            if self.minute <= 30 {
                let next_distance = distances[&current][&next];
                if self.log {
                    println!(
                        "Moving from {} to {} over {}",
                        current_node.name, next_node.name, next_distance
                    );
                }
                let next_stop = self.minute + next_distance;
                if next_stop <= 30 {
                    while self.minute != next_stop {
                        self.increment_time();
                    }
                } else {
                    process_last = false;
                    break;
                }
                j = i + 1;
            } else {
                break;
            }
        }
        if self.minute <= 30 && process_last {
            // process last step
            let last_node = path[j];
            self.open_valve(graph, *last_node);
        }
        while self.minute <= 30 {
            self.increment_time();
        }
    }
}

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

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u64,
}

trait GraphExtension {
    fn remove_valves_with_no_flow(&mut self);
    fn create_graph(valves: &Vec<Valve>, valve_connections: HashMap<String, Vec<String>>) -> Graph<&Valve, usize>;
}

impl GraphExtension for Graph<&Valve, usize> {
    fn remove_valves_with_no_flow(&mut self) {
        // Reduce the graph by removing all 0 flow nodes and increasing distances
        let mut index = 0;
        // When a node is removed, `node_count()` will decrease, otherwise index will increase
        while index < self.node_count() {
            let node_index = self.node_indices().skip(index).next().unwrap();
            let valve = self.node_weight(node_index).unwrap();
            let name = valve.name.clone();
            if name == "AA" {
                index += 1;
                continue;
            }
            let flow_rate = valve.flow_rate;
            // We want to remove this node
            if flow_rate == 0 {
                let incoming = self
                    .neighbors_directed(node_index, petgraph::Direction::Incoming)
                    .collect::<Vec<_>>();
                let outgoing = self
                    .neighbors_directed(node_index, petgraph::Direction::Outgoing)
                    .collect::<Vec<_>>();
                for i in incoming {
                    let incoming_weight = *self.edge_weight(self.find_edge(i, node_index).unwrap()).unwrap();
                    for o in &outgoing {
                        let o = o.to_owned();
                        if o != i {
                            let out_going_weight = *self.edge_weight(self.find_edge(o, node_index).unwrap()).unwrap();
                            self.add_edge(i, o, incoming_weight + out_going_weight);
                        }
                    }
                }
                self.remove_node(node_index);
            } else {
                index += 1;
            }
        }
    }

    fn create_graph(valves: &Vec<Valve>, valve_connections: HashMap<String, Vec<String>>) -> Graph<&Valve, usize> {
        let mut graph = Graph::<&Valve, usize>::new();
        let mut indexes = HashMap::new();
        for valve in valves.iter() {
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

fn part1(input: &str) -> R<u64> {
    let mut valves = vec![];
    let mut valve_connections = HashMap::new();
    for line in input.lines() {
        let name = line.split(' ').nth(1).unwrap().to_string();
        let flow_rate: u64 = line.split(';').next().unwrap().split('=').last().unwrap().parse()?;
        let edges = line
            .split(" ")
            .skip(9)
            .map(|x| x[0..2].to_string())
            .collect::<Vec<String>>();

        valve_connections.insert(name.clone(), edges);
        valves.push(Valve { name, flow_rate });
    }

    let mut graph = Graph::create_graph(&valves, valve_connections);

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

    // Remove all 0 flow_rates from the graph and increase edge distance for each removed
    graph.remove_valves_with_no_flow();
    #[cfg(not)]
    {
        #[cfg(test)]
        let file_name = "day16_sample_min.dot";
        #[cfg(not(test))]
        let file_name = "day16_real_min.dot";

        let mut file = std::fs::File::create(file_name)?;
        file.write_fmt(format_args!(
            "{:?}",
            Dot::with_config(&graph, &[Config::_Incomplete(())])
        ))?;
    }
    let mut distances = HashMap::new();
    for a in graph.node_indices() {
        let distance = dijkstra(&graph, a, None, |x| *x.weight());
        distances.insert(a, distance);
    }

    // Print distances chart
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

    let min_distance_between_nodes = distance_vals.into_iter().filter(|x| **x > 0).min().unwrap();
    println!("Min_distance_between_nodes : {:}", min_distance_between_nodes);
    // The max nodes that can be visited is if we have 30 minutes, and it takes min_distance +1 minutes to visit a node,
    // we can limit permutations by only generating permutations of certain length
    let max_visited_nodes = 30 / (min_distance_between_nodes + 1);

    let mut largest_flow = 0;
    let mut fastest_path = vec![];
    let start = graph
        .node_indices()
        .find(|x| graph.node_weight(x.to_owned()).unwrap().name == "AA")
        .unwrap();
    let all_paths = distances
        .keys()
        .filter(|x| graph.node_weight(**x).unwrap().name != "AA");

    let all_paths = all_paths.permutations((max_visited_nodes - 1).min(distances.keys().len() - 1));
    let len = all_paths.clone().count();

    let mut diff = SystemTime::now();
    for (i, path) in all_paths.enumerate() {
        #[cfg(not(test))]
        {
            if largest_flow >= 2070 {
                break;
            }
        }
        let mut path = VecDeque::from(path);
        path.push_front(&start);
        let path = Vec::from(path);
        let mut state = State::new(false);
        state.step(&graph, &path, &distances);
        if largest_flow < state.total_flow {
            println!(
                "{:}, ({}/{})",
                state.total_flow,
                i.separate_with_commas(),
                len.separate_with_commas()
            );
            largest_flow = state.total_flow;
            fastest_path = path.clone();
        }
        if i % 1_000_000 == 0 {
            println!(
                "({}/{}) {}ms",
                i.separate_with_commas(),
                len.separate_with_commas(),
                diff.elapsed().unwrap().as_millis()
            );
            diff = SystemTime::now();
        }
    }
    for a in &fastest_path {
        let a = a.to_owned();
        print!("{} ", graph.node_weight(*a).unwrap().name);
    }
    println!();
    let mut state = State::new(true);
    state.step(&graph, &fastest_path, &distances);
    Ok(largest_flow)
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
        assert_eq!(part2(SAMPLE1).unwrap(), 0);
    }
}
