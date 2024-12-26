use std::{collections::VecDeque, path::Component};

use common::*;
use petgraph::{
    adj::NodeIndex,
    dot::{Config, Dot},
    visit::{depth_first_search, Control, DfsEvent, IntoNeighbors, IntoNodeIdentifiers, Time},
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

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut graph: UnGraphMap<&str, i32> = GraphMap::new();
    for line in input.lines() {
        let a = line.split_once('-').unwrap();
        graph.add_edge(a.0, a.1, 1);
    }
    let graph = graph.into_graph::<NodeIndex>();
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    let mut paths = vec![];
    for node in graph.node_identifiers() {
        let mut scratch = vec![];
        scratch.push(node);
        depth_first_search(&graph, Some(node), |event| {
            assert_eq!(scratch[0], node);
            match event {
                DfsEvent::TreeEdge(_, n) => {
                    if scratch.len() == 3 {
                        if graph.neighbors(n).contains(&node) {
                            paths.push(scratch.clone());
                        }
                        Control::<()>::Continue
                    } else {
                        scratch.push(n);
                        Control::Continue
                    }
                }
                DfsEvent::BackEdge(_, n) => {
                    if scratch.len() != 1 {
                        let _ = scratch.pop();
                    }
                    Control::Continue
                }
                _ => Control::Continue,
            }
        });
    }
    for mut path in paths {
        //path.sort();
        for node in path {
            print!("{},", graph[node]);
        }
        println!();
    }
    Err(AdventOfCodeError::UnimplementedError)?
}

fn part2(input: &str) -> anyhow::Result<usize> {
    Err(AdventOfCodeError::UnimplementedError)?
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 7);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 0);
    }
}
