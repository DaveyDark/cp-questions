/*
Question:
There is a directed weighted graph that consists of n nodes numbered from 0 to n - 1. The edges of the graph are initially represented by the given array edges where edges[i] = [fromi, toi, edgeCosti] meaning that there is an edge from fromi to toi with the cost edgeCosti.

Implement the Graph class:
    Graph(int n, int[][] edges) initializes the object with n nodes and the given edges.
    addEdge(int[] edge) adds an edge to the list of edges where edge = [from, to, edgeCost]. It is guaranteed that there is no edge between the two nodes before adding this one.
    int shortestPath(int node1, int node2) returns the minimum cost of a path from node1 to node2. If no path exists, return -1. The cost of a path is the sum of the costs of the edges in the path.
*/

// Approach:
// Graph:
// The struct contains a 2d vector of (i32,i32) tuples called graph which is an adjacency list for
// the graph where the ith element is the list of nodes and their weights connected to the ith node
//  new():
//      Create an empty adjacency list of n nodes and then populate it with the given edges, put
//      it in a Graph object and return it.
//  add_edge():
//      Updates the adjacency list with the given edge
//  shortest_path():
//      Uses Dijkstra's Algorithm to compute the shortest path from node1 to node2
//      We create a heap to store the nodes to explore and a costs vector to store the calculated
//      cost of each node
//      Loop until the heap is empty:
//          If the current node is the destination we return the computed distance
//          Each iteration we pop a node from the heap and then loop over it's neighbors which we
//          grab from the graph adjacency list. If the current distance is shorter than the store
//          distance for that neighbor then we update it and push the neighbor into the heap
//      After the loop if the node isn't found we return -1

use std::collections::BinaryHeap;

struct Graph {
    graph: Vec<Vec<(i32, i32)>>,
}

impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut vec = vec!(vec!(); n as usize);
        for edge in &edges {
            vec[edge[0] as usize].push((edge[1], edge[2]));
        }
        Graph {
            graph: vec,
        }
    }
    
    fn add_edge(&mut self, edge: Vec<i32>) {
        self.graph[edge[0] as usize].push((edge[1], edge[2]));
    }
    
    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let mut costs = vec!(i32::MAX; self.graph.len() + 1);
        let mut heap = BinaryHeap::new();
        heap.push((0, node1));
        costs[node1 as usize] = 0; // cost of source is 0
        while !heap.is_empty() {
            let (currCost, currNode) = heap.pop().unwrap();
            if currNode == node2 {return -currCost}
            // loop over neighbors of current node
            for edge in &self.graph[currNode as usize] {
                //update existing cost if it is cheaper
                if edge.1 - currCost < costs[edge.0 as usize] {
                    costs[edge.0 as usize] = edge.1 - currCost;
                    heap.push((-costs[edge.0 as usize], edge.0));
                }
            }
        }
        -1
    }
}
