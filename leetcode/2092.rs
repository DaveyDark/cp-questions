/*
Question:
You are given an integer n indicating there are n people numbered from 0 to n - 1
. You are also given a 0-indexed 2D integer array meetings where
meetings[i] = [xi, yi, timei] indicates that person xi and person yi have a meeting at timei.
A person may attend multiple meetings at the same time. Finally, you are given an integer firstPerson.

Person 0 has a secret and initially shares the secret with a person firstPerson at time 0.
This secret is then shared every time a meeting takes place with a person that has the secret.
More formally, for every meeting, if a person xi has the secret at timei,
then they will share the secret with person yi, and vice versa.

The secrets are shared instantaneously.
That is, a person may receive the secret and share it with people in other meetings within the same time frame.

Return a list of all the people that have the secret after all the meetings have taken place.
You may return the answer in any order.
*/

// Approach:
// We can treat the input as a graph, where each person is a node and each meeting is an edge.
// The weight of the edge is the time of the meeting.
// We can use a BFS/Djikstra like algorithm to traverse the graph and find all the people that have the secret.
//
// We use a array to track the time we visited each node, and a queue to keep track of the nodes we have visited.
// Then we use the queue to start the traversal from 0 and first_person.
//
// For each node, we check the neighbors and if the edge weight >= the edge weight to the current
// node, and we haven't visited that neighbor earlier(times[nb.0] > edge weight), we can visit the neighbor.
//
// Repeat the process until the queue is empty and return the people we have visited.

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        // Transform the input into a graph represented by an adjacency list
        let mut graph = vec![vec![]; n as usize];
        for meet in meetings {
            graph[meet[0] as usize].push((meet[1], meet[2]));
            graph[meet[1] as usize].push((meet[0], meet[2]));
        }

        // Track the nodes we have vistied and at what time we visited
        let mut times = vec![i32::MAX; n as usize];
        times[0] = 0;
        times[first_person as usize] = 0;

        let mut people = HashSet::new();

        // Queue for BFS traversal, starting at 0 and first_person
        let mut queue = Vec::new();
        queue.push((0, 0));
        queue.push((first_person, 0));

        while let Some((node, time)) = queue.pop() {
            people.insert(node);
            // Explore Neighbors of current node
            for nb in graph[node as usize].iter() {
                if nb.1 >= time && times[nb.0 as usize] > nb.1 {
                    // If the time/weight of the edge going to the neighbor is
                    // >= the time/weight to get to the current node,
                    // And we haven't visited the node at that time,
                    // We can go to the neighbor
                    times[nb.0 as usize] = nb.1;
                    queue.push((nb.0, nb.1));
                }
            }
        }

        // Return the contents of visited as a Vec
        people.into_iter().collect()
    }
}
