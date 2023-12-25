use aoc_runner_derive::aoc;
use indexmap::IndexSet;
use std::collections::HashMap;

#[aoc(day25, part1)]
fn solve_part1(input: &str) -> usize {
    let mut contraption = Contraption::from(input);

    let (left, right) = contraption.split();

    left * right
}

struct Contraption<'a> {
    nodes: HashMap<&'a str, usize>,
    edges: HashMap<&'a str, HashMap<&'a str, usize>>,
}

impl<'a> Contraption<'a> {
    fn split(&mut self) -> (usize, usize) {
        let mut min_cut_weight = usize::MAX;
        let mut left_size = 0;
        let mut right_size = 0;

        while self.nodes.len() > 1 {
            let (cut_weight, left, right) = self.minimum_cut_phase();

            if cut_weight < min_cut_weight {
                min_cut_weight = cut_weight;
                left_size = left;
                right_size = right;
            }
        }

        assert_eq!(3, min_cut_weight);

        (left_size, right_size)
    }

    fn minimum_cut_phase(&mut self) -> (usize, usize, usize) {
        let mut subset = IndexSet::new();
        subset.insert(*self.nodes.keys().next().unwrap());

        while let Some(next) = self.most_connected(&subset) {
            subset.insert(next);
        }

        let mut nodes = subset.into_iter().rev();
        let second = nodes.next().unwrap();
        let first = nodes.next().unwrap();

        let total_node_weight: usize = self.nodes.values().copied().sum();
        let right = *self.nodes.get(second).unwrap();
        let left = total_node_weight - right;
        let cut_weight = self.edges.get(second).unwrap().values().copied().sum();

        self.merge(first, second);

        (cut_weight, left, right)
    }

    fn most_connected(&self, subset: &IndexSet<&'a str>) -> Option<&'a str> {
        let mut max_weight = 0;
        let mut max_node = "";

        for node in self.nodes.keys().copied().filter(|n| !subset.contains(n)) {
            // Calculate the node's connection weight to the subset
            let weight = self
                .edges
                .get(node)
                .unwrap()
                .iter()
                .filter_map(|(other, weight)| subset.contains(*other).then_some(*weight))
                .sum();

            if weight > max_weight {
                max_weight = weight;
                max_node = node;
            }
        }

        if max_node.is_empty() {
            None
        } else {
            Some(max_node)
        }
    }

    fn merge(&mut self, first: &'a str, second: &'a str) {
        let mut edges: HashMap<&'a str, usize> = HashMap::new();

        // Calculate new edges & weights
        for (connection, weight) in self.edges.get(first).unwrap() {
            if *connection != second {
                edges.insert(*connection, *weight);
            }
        }

        // Calculate new edges & weights
        for (connection, weight) in self.edges.get(second).unwrap() {
            if *connection != first {
                *edges.entry(*connection).or_default() += *weight;
            }
        }

        for (node, connections) in self.edges.iter_mut() {
            connections.remove(second);
            if let Some(weight) = edges.get(node).copied() {
                connections.insert(first, weight);
            }
        }

        let weight = self.nodes.remove(second).unwrap();
        *self.nodes.get_mut(first).unwrap() += weight;

        self.edges.insert(first, edges);
    }
}

impl<'a> From<&'a str> for Contraption<'a> {
    fn from(value: &'a str) -> Self {
        let mut nodes = HashMap::new();
        let mut edges: HashMap<_, HashMap<_, _>> = HashMap::new();

        for line in value.lines() {
            let (node, connections) = line.split_once(": ").unwrap();

            nodes.insert(node, 1);

            for connection in connections.split(' ') {
                nodes.insert(connection, 1);
                edges.entry(node).or_default().insert(connection, 1);
                edges.entry(connection).or_default().insert(node, 1);
            }
        }

        Self { nodes, edges }
    }
}

#[test]
fn test_split() {
    let mut con = Contraption::from(
        "jqt: rhn xhk nvd
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
frs: qnr lhk lsr",
    );

    assert_eq!((9, 6), con.split());
}
