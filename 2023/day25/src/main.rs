use std::collections::{BTreeMap, HashMap};

use common::{
    petgraph::{
        algo::{astar::astar, tarjan_scc},
        visit::IntoNodeIdentifiers,
        Graph, Undirected,
    },
    winnow::{
        ascii::{alpha1, space1},
        combinator::separated,
        prelude::*,
        seq,
    },
    *,
};
use rand::{seq::SliceRandom, thread_rng};

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
struct LineOfInput<'a> {
    key: &'a str,
    values: Vec<&'a str>,
}

fn line_of_input<'a>(input: &mut &'a str) -> PResult<LineOfInput<'a>> {
    seq!(LineOfInput {
        key:alpha1,
        _:": ",
        values: separated(1..,alpha1,space1)
    })
    .parse_next(input)
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut graph = Graph::<&str, (), Undirected>::new_undirected();
    let mut nodes = HashMap::new();
    // Build the graph
    for line in input.lines().map(|mut line| line_of_input(&mut line).unwrap()) {
        let key_node = *nodes.entry(line.key).or_insert_with(|| graph.add_node(line.key));
        for value in line.values {
            let value_node = *nodes.entry(value).or_insert_with(|| graph.add_node(value));
            graph.update_edge(key_node, value_node, ());
        }
    }
    let mut rng = thread_rng();
    let mut edges_count = BTreeMap::new();
    let nodeids = graph.node_identifiers().collect_vec();
    // For 1000 iterations, randomly pick 2 nodes and find the shortest path between them. There is
    // a 50% chance that one of the boundary edges is passed through. Save each edge that is
    // passed. After 1000 iterations the 3 most common should be the edges to be removed.
    for _ in 0..1000 {
        let mut rand_iter = nodeids.choose_multiple(&mut rng, 2);
        let a = *rand_iter.next().unwrap();
        let b = *rand_iter.next().unwrap();
        let (_dist, nodes_visited) = astar(&graph, a, |x| x == b, |_| 1, |_| 0).unwrap();
        for nodes in nodes_visited.windows(2) {
            let edge = graph.find_edge(nodes[0], nodes[1]).unwrap();
            edges_count.entry(edge).and_modify(|x| *x += 1).or_insert(1);
        }
    }
    for (k, _) in edges_count.into_iter().sorted_by(|(_, v1), (_, v2)| v2.cmp(v1)).take(3) {
        graph.remove_edge(k);
    }

    #[cfg(not)]
    {
        use common::petgraph::dot::Dot;
        use std::io::Write;
        let dot = Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel]);
        std::fs::File::create("day25pt1.dot")?.write_all(format!("{dot:?}").as_bytes())?;
    }
    // Hack: I found this function when working on another problem and it just seperates the graph
    // into groups. I can reuse it here to get the 2 groups since they are separated
    let groups = tarjan_scc(&graph);
    Ok(groups[0].len() * groups[1].len())
}

fn part2(input: &str) -> anyhow::Result<usize> {
    Err(AdventOfCodeError::UnimplementedError)?
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 54);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 0);
    }
}
