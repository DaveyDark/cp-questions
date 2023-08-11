/*
Question:
There are n cities numbered from 0 to n - 1 and n - 1 roads such that there is only one way to travel between two different cities (this network form a tree). 
Last year, The ministry of transport decided to orient the roads in one direction because they are too narrow.

Roads are represented by connections where connections[i] = [ai, bi] represents a road from city ai to city bi.

This year, there will be a big event in the capital (city 0), and many people want to travel to this city.

Your task consists of reorienting some roads such that each city can visit the city 0. Return the minimum number of edges changed.

It's guaranteed that each city can reach city 0 after reorder.
*/

// Approach:
// 1) Make a graph out of the given connections which will be a 2d vector, where the first index is the node id
//    The inner vector contains all nodes it is connected to along with a bool to represent if it's the source of the connection
// 2) Populate the graph using the given connections
// 3) Make a function to use DFS to traverse the graph called check()
//    check() will go through all unvisited connections of a given node and if it needs to be inverted, add 1 and call check() on the connection 
//    otherwise it will jstu call check() on the connection without adding 1, and return the count of inverted connections at the end
// 4) Back in min_reorder, we can call check on the root of the graph and return the count returned from it

impl Solution {
    fn check(map: &Vec<Vec<(i32,bool)>>, visited: i32, source: i32) -> i32 {
        let mut cnt = 0;
        for connection in &map[source as usize] {
            if connection.0 == visited {continue}
            if connection.1 {
                cnt += 1 + Self::check(map, source, connection.0);
            } else {
                cnt += Self::check(map, source, connection.0);
            }
        }
        return cnt;
    }
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut map = vec!(vec!(); n as usize);
        for conn in connections {
            map[conn[0] as usize].push((conn[1],true));
            map[conn[1] as usize].push((conn[0],false));
        }
        Self::check(&map, -1, 0)
    }
}
