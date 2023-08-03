/*
Question:
You are given an m x n matrix maze (0-indexed) with empty cells (represented as '.') and walls (represented as '+'). 
You are also given the entrance of the maze, where entrance = [entrancerow, entrancecol] denotes the row and column of the cell you are initially standing at.

In one step, you can move one cell up, down, left, or right. You cannot step into a cell with a wall, and you cannot step outside the maze. 
Your goal is to find the nearest exit from the entrance. An exit is defined as an empty cell that is at the border of the maze. 
The entrance does not count as an exit.

Return the number of steps in the shortest path from the entrance to the nearest exit, or -1 if no such path exists.
*/

// Approach:
// 1) Clone maze and make it mutable
// 2) Make a queue and init it with entrance, a counter to measure distance and an array to store adjacent coordinates that we can traverse
// 3) While there are items in the queue, make another empty vec and process each element of the queue
//    If the element is at boundry and it's not the entrance,return the currect distance
//    Otherwise we traverse the adjacent tiles of the element
//      If any adjacent tile is out of bounds, ignore it
//      Otherwise if it's an empty space, add it to the next vector
//    After that, transfer the next to queue and increment distance
// 4) If we reach outside the loop, we return -1 since no path is possible

impl Solution {
    fn is_boundry(bounds: &Vec<i32>, point: &Vec<i32>) -> bool {
        point[0] == 0 || point[0] == bounds[0] - 1 || point[1] == 0 || point[1] == bounds[1] - 1
    }
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let mut maze = maze.clone();
        let adjacent = [(-1,0),(0,-1),(1,0),(0,1)];
        let mut queue = vec!(entrance.clone());
        let mut dist = 0;
        while !queue.is_empty() {
            let mut next = vec!();
            for node in queue {
                if Self::is_boundry(&vec!(maze.len() as i32,maze[0].len() as i32),&node) && node != entrance {
                    return dist;
                }
                for pair in &adjacent {
                    if node[0] + pair.0 < 0 || node[0] + pair.0 == maze.len() as i32
                        || node[1] + pair.1 < 0 || node[1] + pair.1 == maze[0].len() as i32 {continue}
                    if maze[(node[0] + pair.0) as usize][(node[1] + pair.1) as usize] == '.' {
                        next.push(vec!((node[0] + pair.0),(node[1] + pair.1)));
                        maze[(node[0] + pair.0) as usize][(node[1] + pair.1) as usize] = '+';
                    }
                }
            }
            queue = next;
            dist += 1;
        }
        -1
    }
}
