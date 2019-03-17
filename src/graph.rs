pub struct Graph {
    size: usize,
    vertexes: Vec<u32>,
    edges: Vec<Box<Vec<usize>>>,

}

#[allow(dead_code)]
impl Graph {
    pub fn new(size: usize) -> Graph {
        Graph {
            size,
            vertexes: vec![0; size],
            edges: vec![Box::new(vec![]); size],
        }
    }


    pub fn add_edge(&mut self, node_a: usize, node_b: usize) {
        //Make sure the nodes are on the graph
        assert!(node_a <= self.size);
        assert!(node_b <= self.size);

        {
            let node_a_edges = self.edges.get_mut(node_a-1).unwrap();
            //prevent redundant edges.
            if node_a_edges.contains(&node_b) {
               return;
            }
            node_a_edges.push(node_b);
        }

        let node_b_edges = self.edges.get_mut(node_b-1).unwrap();
        node_b_edges.push(node_a);
    }

    pub fn remove_edge(&mut self, node_a: usize, node_b: usize) {
        //Make sure the nodes are on the graph
        assert!(node_a <= self.size);
        assert!(node_b <= self.size);

        {
            let node_a_edges = self.edges.get_mut(node_a - 1).unwrap();
            node_a_edges.iter()
                .position(|&n| n == node_b)
                .map(|e| node_a_edges.remove(e));
        }

        let node_b_edges = self.edges.get_mut(node_b-1).unwrap();
        node_b_edges.iter()
            .position(|&n| n==node_a)
            .map(|e| node_b_edges.remove(e));

    }

    pub fn get_vertex_value(&self, node: usize) -> u32 {
        assert!(node <= self.size);
        self.vertexes[node-1]
    }

    pub fn set_vertex_value(&mut self, node: usize, value: u32) {
        assert!(node <= self.size);
        self.vertexes[node-1] = value;
    }

    pub fn neighbors(&self, node: usize) -> Vec<usize> {
        assert!(node <= self.size);
        return *self.edges.get(node-1).unwrap().clone();
    }

    pub fn adjacent(&self, node_a: usize, node_b: usize) -> bool {
        assert!(node_a <= self.size);
        assert!(node_b <= self.size);
        self.neighbors(node_a).contains(&node_b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_add_edge_test() {
        let mut graph = Graph::new(4);
        graph.add_edge(5,1);
    }

    #[test]
    fn valid_add_edge_test() {
        let mut graph = Graph::new(9);
        graph.add_edge(1, 2);
        graph.add_edge(1, 4);
        graph.add_edge(2, 1);
        graph.add_edge(2, 3);
        graph.add_edge(2, 5);
        graph.add_edge(9, 1);

        assert_eq!(graph.neighbors(1), [2, 4, 9]);
        assert_eq!(graph.neighbors(2), [1, 3, 5]);
    }

    #[test]
    fn empty_edge_test() {
        let graph = Graph::new(4);

        assert_eq!(graph.neighbors(1), []);
        assert_eq!(graph.neighbors(4), []);
    }

    #[test]
    fn remove_edge_test() {
        let mut graph = Graph::new(9);
        graph.add_edge(1, 2);
        graph.add_edge(1, 4);
        graph.add_edge(2, 1);
        graph.add_edge(2, 3);
        graph.add_edge(2, 5);
        graph.add_edge(9, 1);
        assert_eq!(graph.neighbors(1), [2, 4, 9]);
        assert_eq!(graph.neighbors(2), [1, 3, 5]);

        graph.remove_edge(1, 2);

        assert_eq!(graph.neighbors(1), [4, 9]);
        assert_eq!(graph.neighbors(2), [3, 5]);
    }

    #[test]
    fn set_get_vertex_value_test() {
        let mut graph = Graph::new(9);
        graph.set_vertex_value(1, 5);
        graph.set_vertex_value(9, 3);

        assert_eq!(graph.get_vertex_value(1), 5);
        assert_eq!(graph.get_vertex_value(9), 3);
    }

    #[test]
    fn adjacency_test() {
        let mut graph = Graph::new(9);
        graph.add_edge(1, 2);
        graph.add_edge(1, 4);
        graph.add_edge(2, 1);
        graph.add_edge(2, 3);
        graph.add_edge(2, 5);
        graph.add_edge(9, 1);

        assert!(graph.adjacent(1,2));
        assert!(!graph.adjacent(1,3));

    }

    #[test]
    #[should_panic]
    fn invalid_adjacency_test() {
        let mut graph = Graph::new(9);
        graph.add_edge(1, 2);
        graph.adjacent(10,1);
    }

    #[test]
    #[should_panic]
    fn invalid_set_vertex_test() {
        let mut graph = Graph::new(9);
        graph.set_vertex_value(10,0);
    }

}