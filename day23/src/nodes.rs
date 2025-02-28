use std::{collections::HashMap, ops::Index};

use intersect_sorted_iterators::IntersectionIterator;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Node {
    name: [u8; 2],
    index: usize,
    t_adjacent: bool,
}

const fn node_name2index(name: [u8; 2]) -> usize {
    (name[0] - b'a') as usize * 26 + (name[1] - b'a') as usize
}

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(input: &str) -> Self {
        let mut nodes = Vec::with_capacity(26 * 26);
        let mut edges = vec![Vec::new(); 26 * 26];
        for letter1 in b'a'..=b'z' {
            for letter2 in b'a'..=b'z' {
                nodes.push(Node {
                    name: [letter1, letter2],
                    index: node_name2index([letter1, letter2]),
                    t_adjacent: false,
                })
            }
        }
        for line in input.lines() {
            let mut bytes = line.bytes();
            let node_a_name = [bytes.next().unwrap(), bytes.next().unwrap()];
            let node_a_index = node_name2index(node_a_name);
            bytes.next();
            let node_b_name = [bytes.next().unwrap(), bytes.next().unwrap()];
            let node_b_index = node_name2index(node_b_name);
            if node_a_index < node_b_index {
                edges[node_a_index].push(node_b_index);
            } else {
                edges[node_b_index].push(node_a_index)
            }
            if node_a_name[0] == b't' || node_b_name[0] == b't' {
                nodes[node_a_index].t_adjacent = true;
                nodes[node_b_index].t_adjacent = true;
            }
        }
        for node_edges in edges.iter_mut() {
            node_edges.sort();
        }

        let mut hm = HashMap::new();
        for node_edges in edges.clone() {
            let entry = hm.entry(node_edges.len()).or_insert(0);
            *entry += 1;
        }
        let mut v: Vec<(usize, i32)> = hm.iter().map(|(k, v)| (*k, *v)).collect();
        v.sort();
        for (edges, count) in v {
            println!("edges : {edges:5}    count : {count:5}");
        }

        Self { nodes, edges }
    }
    pub fn count_triangles(&self) -> usize {
        // const T_RANGE: std::ops::Range<usize> =
        //     node_name2index([b't', b'a'])..node_name2index([b's', b'a']);
        let mut count = 0;
        let mut triangles = Vec::new();

        for node_a in &self.nodes {
            if node_a.t_adjacent {
                let node_a_edges = &self.edges[node_a.index];
                for node_b_index_index in 0..node_a_edges.len() {
                    let node_b_index = node_a_edges[node_b_index_index];
                    let mut node_b_edges = self.edges[node_b_index].iter();
                    let mut node_a_edges = node_a_edges[node_b_index_index + 1..].iter();
                    let intersection =
                        IntersectionIterator::new(&mut node_a_edges, &mut node_b_edges);
                    for &node_c_index in intersection {
                        if node_a.name[0] == b't'
                            || self.nodes[node_b_index].name[0] == b't'
                            || self.nodes[node_c_index].name[0] == b't'
                        {
                            triangles.push([
                                node_a.name,
                                self.nodes[node_b_index].name,
                                self.nodes[node_c_index].name,
                            ]);
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    fn max_complete_graph(&self, graph_nodes: &[usize], hm: &mut HashMap<Vec<usize>, Cgr>) -> Cgr {
        if graph_nodes.is_empty() {
            unreachable!();
        }
        if hm.contains_key(graph_nodes) {
            hm.get(graph_nodes).unwrap().clone()
        } else {
            let mut result;
            if graph_nodes.len() == 1 {
                result = Cgr::new(graph_nodes[0]);
            } else {
                let rest = self.max_complete_graph(&graph_nodes[1..], hm);
                let rest_len = rest.len();

                let mut iter1 = self.edges[graph_nodes[0]].iter();
                let mut iter2 = graph_nodes[1..].iter();
                let v: Vec<usize> =
                    intersect_sorted_iterators::IntersectionIterator::new(&mut iter1, &mut iter2)
                        .copied()
                        .collect();
                if v.is_empty() {
                    result = rest;
                } else {
                    let minus_first_node = self.max_complete_graph(v.as_ref(), hm);
                    let minus_first_node_len = minus_first_node.len();
                    if minus_first_node_len < rest_len {
                        result = rest;
                    } else {
                        result = minus_first_node;
                        result.push(graph_nodes[0]);
                    }
                }
            }
            hm.insert(graph_nodes.to_vec(), result.clone());
            result
        }
    }

    const ALL_VALUES: [usize; 26 * 26] = {
        let mut a: [usize; 26 * 26] = [0; 26 * 26];
        let mut i = 0;
        while i < 26 * 26 {
            a[i] = i;
            i += 1;
        }
        a
    };

    pub fn largest_complete_graph_size(&self) -> Cgr {
        let mut hm = HashMap::new();
        self.max_complete_graph(&Graph::ALL_VALUES[..], &mut hm)
    }
}

#[derive(Eq, Clone)]
pub struct Cgr {
    nodes: Vec<usize>,
}

impl std::fmt::Debug for Cgr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cgr")
            .field("nodes", &self.as_string())
            .finish()
    }
}

const CGR_EMPTY: Cgr = Cgr { nodes: Vec::new() };
impl Cgr {
    pub fn new(node: usize) -> Self {
        Self {
            // size,
            nodes: vec![node],
        }
    }
    fn len(&self) -> usize {
        self.nodes.len()
    }
    fn as_string(&self) -> String {
        let mut nodes = self.nodes.clone();
        nodes.reverse();
        nodes.iter().fold(String::new(), |mut acc, &index| {
            if !acc.is_empty() {
                acc.push(',');
            }
            acc.push(((index / 26) as u8 + b'a') as char);
            acc.push(((index % 26) as u8 + b'a') as char);
            acc
        })
    }
    fn push(&mut self, node_index: usize) {
        self.nodes.push(node_index);
    }
    // pub fn append(&mut self, other: &mut Self) {
    //     // self.size += other.size;
    //     self.nodes.append(&mut other.nodes);
    // }
}

impl PartialEq for Cgr {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len()
    }
}

impl Ord for Cgr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.len().cmp(&other.len())
    }
}

impl PartialOrd for Cgr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub mod intersect_sorted_iterators;

#[cfg(test)]
mod tests {
    use crate::{INPUT, TESTINPUT};

    use super::{Cgr, Graph};

    #[test]
    fn test_new() {
        Graph::new(TESTINPUT);
    }
    #[test]
    fn test_new_real_input() {
        Graph::new(INPUT);
    }
    #[test]
    fn test_count_triangles() {
        let graph = Graph::new(TESTINPUT);
        let count = graph.count_triangles();
        assert_eq!(count, 7);
    }

    #[test]
    fn test_complete() {
        let graph = Graph::new(TESTINPUT);
        let largest = graph.largest_complete_graph_size();
        println!("{:?}", largest);
        assert_eq!(largest.len(), 4);
    }

    #[test]
    fn test_cgr() {
        let mut cgr = Cgr::new(0);
        cgr.push(1);
        cgr.push(2);
        println!("{cgr:?}, {}, {}", cgr.nodes.len(), cgr.as_string());
    }
}
