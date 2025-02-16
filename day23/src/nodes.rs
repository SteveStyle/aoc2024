use std::collections::HashMap;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Node {
    name: [u8; 2],
    index: usize,
    edges: Vec<usize>,
}

const fn node_name2index(name: [u8; 2]) -> usize {
    (name[0] - b'a') as usize * 26 + (name[1] - b'a') as usize
}
impl Node {
    fn start_with_t(&self) -> bool {
        self.name[0] == b't'
    }
    fn index(&self) -> usize {
        node_name2index(self.name)
    }
    fn can_ignore(&self) -> bool {
        self.edges.len() < 2
    }
}

struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new(input: &str) -> Self {
        let mut nodes = Vec::with_capacity(26 * 26);
        for letter1 in b'a'..=b'z' {
            for letter2 in b'a'..=b'z' {
                nodes.push(Node {
                    name: [letter1, letter2],
                    index: node_name2index([letter1, letter2]),
                    edges: Vec::new(),
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
                nodes[node_a_index].edges.push(node_b_index);
            } else {
                nodes[node_b_index].edges.push(node_a_index)
            }
        }
        for node in nodes.iter_mut() {
            node.edges.sort();
        }

        let mut hm = HashMap::new();
        for node in nodes.clone() {
            let entry = hm.entry(node.edges.len()).or_insert(0);
            *entry += 1;
        }
        let mut v: Vec<(usize, i32)> = hm.iter().map(|(k, v)| (*k, *v)).collect();
        v.sort();
        for (edges, count) in v {
            println!("edges : {edges:5}    count : {count:5}");
        }

        Self { nodes }
    }
    pub fn count_triangles(&self) -> usize {
        const T_RANGE_LOWER: usize = node_name2index([b't', b'a']);
        const T_RANGE_UPPER: usize = node_name2index([b's', b'a']);
        let count = 0;
        for node_a in &self.nodes {
            for node_a_edges_index_b in 0..node_a.edges.len() {
                let node_b_edges = &self.nodes[node_a_edges_index_b].edges;
                for node_b_index in IntersectionIterator::new(&node_b_edges, )
            }
        }
        count
    }
}

struct IntersectionIterator<'a> {
    vector1: &'a [usize],
    vector1_index: usize,
    vector2: &'a [usize],
    vector2_index: usize,
}

impl<'a> Iterator for IntersectionIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.vector1[self.vector1_index] != self.vector2[self.vector2_index] {
            if self.vector1[self.vector1_index] < self.vector2[self.vector2_index] {
                self.vector1_index += 1;
                if self.vector1_index == self.vector1.len() {
                    return None;
                }
            } else {
                self.vector2_index += 1;
                if self.vector2_index == self.vector2.len() {
                    return None;
                }
            }
        }
        let value = self.vector1[self.vector1_index];
        while self.vector1[self.vector1_index] == value {
            self.vector1_index += 1;
        }
        while self.vector2[self.vector2_index] == value {
            self.vector2_index += 1;
        }
        Some(value)
    }
}

impl<'a> IntersectionIterator<'a> {
    fn new(vector1: &'a [usize], vector2: &'a [usize]) -> Self {
        Self {
            vector1,
            vector2,
            vector1_index: 0,
            vector2_index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{INPUT, TESTINPUT};

    use super::Graph;

    #[test]
    fn test_new() {
        Graph::new(TESTINPUT);
    }
    #[test]
    fn test_new_real_input() {
        Graph::new(INPUT);
    }
}
