// #![allow(dead_code, unused)]

use stephen_morris_utils::timer::time;
const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TESTINPUT: &str = "kh-tc
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
td-yn";

mod nodes;
use nodes::Graph;

fn main() {
    let graph = time(|| Graph::new(INPUT), "graph");
    let triangles = time(|| graph.count_triangles(), "triangles");

    graph.print_duration();
    triangles.print_all();

    let max_complete_graph = time(|| graph.largest_complete_graph_size(), "complete graph");
    max_complete_graph.print_all();
}
