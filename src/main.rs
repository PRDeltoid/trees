use graph::Graph;

mod binary_search_tree;
mod binary_heap;
mod graph;

fn main() {
    let mut graph = Graph::new(9);
    // 3x3 grid where all nodes are connected on their cardinal directions
    // 1 2 3
    // 4 5 6
    // 7 8 9
    graph.add_edge(1,2);
    graph.add_edge(1,4);
    graph.add_edge(2,3);
    graph.add_edge(2,5);
    graph.add_edge(3,6);
    graph.add_edge(4,5);
    graph.add_edge(4,7);
    graph.add_edge(5,6);
    graph.add_edge(5,8);
    graph.add_edge(6,9);
    graph.add_edge(7,8);
    graph.add_edge(8,9);

    let start: usize = 1;
    //distance from start to start is 0
    graph.set_vertex_value(start, 0);

    let open_set: Vec<usize> = vec![1];

    let came_from: Vec<usize> = vec![];

    while !open_set.is_empty() {

    }
    println!("{:?}", graph.neighbors(1));
}

