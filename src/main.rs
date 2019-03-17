use graph::Graph;

mod binary_search_tree;
mod binary_heap;
mod graph;

fn main() {
    let mut graph = Graph::new(4);
    graph.add_edge(1,2);
    graph.add_edge(1,3);
    println!("{:?}", graph.neighbors(1));
}

