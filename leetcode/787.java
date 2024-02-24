/*
Question:
There are n cities connected by some number of flights. 
You are given an array flights where flights[i] = [fromi, toi, pricei] indicates that 
there is a flight from city fromi to city toi with cost pricei.

You are also given three integers src, dst, and k, 
return the cheapest price from src to dst with at most k stops. 
If there is no such route, return -1.
*/

// Approach:
// We treat the given flights as a directed graph where each node represents a city 
// and each edge represents a flight from one city to another.
// Then we can apply Dijkstra's algorithm to find the minimum cost to reach the destination from the source.
//
// There are slight variations in the implementation of Dijkstra's algorithm for this problem.
// We also track the number of stops to reach the destination from the source and stop the traversal if the number of stops exceeds k.
// 
// And we store the shortest distance per number of stops in the 2D array costs.

class Solution {
  List<Pair<Integer, Integer>>[] constructGraph(int[][] edges, int n) {
    List<Pair<Integer, Integer>> adjList[] = new List[n];
    for (int i = 0; i < n; i++) {
      // Initialize the adjacency list with empty lists.
      adjList[i] = new ArrayList<>();
    }
    for (int[] e : edges) {
      // Traverse through the edges and add the edge to the adjacency list.
      adjList[e[0]].add(new Pair(e[1], e[2]));
    }
    return adjList;
  }

  public int findCheapestPrice(int n, int[][] flights, int src, int dst, int k) {
    // Create an adjacency list from the given edges.
    List<Pair<Integer, Integer>> adjList[] = constructGraph(flights, n);

    // Create a 2D array to store the minimum cost to reach a node from the source in a given number of stops.
    // costs[i][j] will store the minimum cost to reach node i from the source using j stops.
    // Initialize the costs array with infinity.
    int costs[][] = new int[n][k+2];
    for (int i = 0; i < n; i++) {
      for (int j = 0; j <= k+1; j++) 
        costs[i][j] = Integer.MAX_VALUE;
    }
    // Set the minimum cost to reach the source from the source as 0.
    costs[src][0] = 0;

    // Create a priority queue to store the nodes to be visited.
    // It will store the node, the cost to reach the node, and the number of stops to reach the node.
    // The priority queue will be sorted based on the cost to reach the node.
    Queue<int[]> queue = new PriorityQueue<>(new Comparator<int[]>() {
      @Override
      public int compare(int[] o1, int[] o2) {
        return Integer.compare(o1[0], o2[0]);
      }
    });

    // Insert the src with 0 cost and 0 stops to the priority queue.
    queue.offer(new int[] { 0, src, 0 });

    // Traverse through the priority queue until it is empty.
    while (!queue.isEmpty()) {
      // Take the node with the minimum cost from the priority queue.
      int[] p = queue.poll();

      // If the number of stops to reach the node is greater than k, continue.
      if (p[2] > k)
        continue;

      // Explore the neighbors of the node.
      for (Pair<Integer, Integer> nb : adjList[p[1]]) {
        int des = nb.getKey();
        int c = p[0] + nb.getValue();
        // Check if we can reach the neighbor with a lower cost using the current node.
        if (costs[des][p[2]+1] > c) {
          // Update the cost to reach the neighbor and add it to the priority queue.
          queue.offer(new int[] { c, des, p[2] + 1 });
          costs[des][p[2]+1] = c;
        }
      }
    }

    // Find the minimum cost to reach the destination from the source in any number of stops.
    int minCost = Integer.MAX_VALUE;
    for(int c: costs[dst]) {
      minCost = Math.min(minCost, c);
    }

    // If the minimum cost is still infinity, return -1, otherwise return the minimum cost.
    return minCost == Integer.MAX_VALUE ? -1 : minCost;
  }
}
