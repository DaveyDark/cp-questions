/*
Question:
You are given an array routes representing bus routes where routes[i] is a bus route that the ith bus repeats forever.
    For example, if routes[0] = [1, 5, 7], this means that the 0th bus travels in the sequence 1 -> 5 -> 7 -> 1 -> 5 -> 7 -> 1 -> ... forever.

You will start at the bus stop source (You are not on any bus initially), and you want to go to the bus stop target. 
You can travel between bus stops by buses only.

Return the least number of buses you must take to travel from source to target. Return -1 if it is not possible.
*/

// Approach:
// 1) If the source is also the target just return 0
// 2) Make a 2d vector to store the graph in adjacency list form and a HashMap to group the
//    stations into routes
// 3) Loop over the given routes and put stations into the stations hashmap. The key is the station
//    and the value is a HashSet containing all the routes the station is a part of
// 4) Loop over all stations in the stations map and for each station we interconnect all the
//    routes it is in in the graph
// 5) Loop over the routes and filter them to find all the routes the source is in
// 6) Loop over the routes and filter them to find all the routes the target is in
// 7) Make a variable to store the shortest path and init it to max
// 8) Perform BFS on the graph using a queue. Enqueue all teh source nodes with a path of 1 and
//    then each iteration dequeue a node and enqueue all it's unexplored neightbors with a path
//    distance of path + 1. If a the current node is one of the targets we return the path
//    otherwise we return i32::MAX at the end
// 9) If a path is found we return it otherwise if the path is still i32::MAX we return -1

use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {return 0}
        let mut graph: Vec<Vec<i32>> = vec!(vec!(); routes.len());
        let mut stations = HashMap::new();
        for i in 0..routes.len() {
            for station in &routes[i] {
                stations.entry(station).or_insert(HashSet::new()).insert(i as i32);
            }
        }
        for (_, routes) in stations.into_iter() {
            let routes: Vec<i32> = routes.into_iter().collect();
            for i in 0..routes.len() {
                for j in i+1..routes.len() {
                    graph[routes[i] as usize].push(routes[j]);
                    graph[routes[j] as usize].push(routes[i]);
                }
            }
        }
        let mut sources: Vec<i32> = routes
            .iter()
            .enumerate()
            .filter(|(_,r)| r.contains(&source))
            .map(|(i,r)| i as i32)
            .collect();
        let mut targets: HashSet<i32> = routes
            .iter()
            .enumerate()
            .filter(|(_,r)| r.contains(&target))
            .map(|(i,r)| i as i32)
            .collect();
        let mut path = i32::MAX;
        path = path.min(Self::bfs(&graph, &sources, &targets));
        if path == i32::MAX {-1} else {path}
    }
    fn bfs(graph: &Vec<Vec<i32>>, sources: &Vec<i32>, targets: &HashSet<i32>) -> i32 {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        for source in sources{
            queue.push_back((*source,1));
            visited.insert(*source);
        }
        while !queue.is_empty() {
            let (curr, path) = queue.pop_front().unwrap();
            if(targets.contains(&curr)) {return path}
            for node in &graph[curr as usize] {
                if visited.insert(*node) {
                    queue.push_back((*node,path+1));
                }
            }
        }
        i32::MAX
    }
}
