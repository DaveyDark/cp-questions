/*
Question:
You are given a 0-indexed integer array nums, and you are allowed to traverse between its indices. 
You can traverse between index i and index j, i != j, if and only if gcd(nums[i], nums[j]) > 1, where gcd is the greatest common divisor.

Your task is to determine if for every pair of indices i and j in nums, where i < j, 
there exists a sequence of traversals that can take us from i to j.

Return true if it is possible to traverse between all such pairs of indices, or false otherwise.

*/

// Approach:
// We can check the given condition by checking if the prime factors of all numbers are connected in a disjoint set.
// For this, we create a DisjointSet class to represent the disjoint set data structure.
// Then we calculate the prime factors of all numbers and insert them into the disjoint set.
// Then we union the prime factors of each number.
// Finally, we check if there is only one set in the disjoint set and return the result.

class DisjointSet {
  // Class to represent a disjoint set data structure
  // parents stores the parent of each node
  HashMap<Integer, Integer> parents;
  // ranks stores the number of nodes in each set
  HashMap<Integer, Integer> ranks;

  DisjointSet() {
    // Constructor for the disjoint set, initializes the parents and ranks
    this.parents = new HashMap<>();
    this.ranks = new HashMap<>();
  }

  DisjointSet(Set<Integer> initialElements) {
    // Constructor for the disjoint set, initializes the parents and ranks
    this();
    // Go through each element in the initialElements set and insert it into the disjoint set
    for (int element : initialElements) {
      this.insert(element);
    }
  }

  // Method to insert a new element into the disjoint set
  void insert(int key) {
    // Insert the element into the parents and ranks hashmaps
    this.parents.put(key, key);
    this.ranks.put(key, 1);
  }

  // Method to count the number of sets in the disjoint set
  int countSets() {
    // Create a new hashset to store the roots of each set
    HashSet<Integer> roots = new HashSet<>();
    for (int key : this.parents.keySet()) {
      // Add the root of the current element to the hashset
      roots.add(find(key));
    }
    // Return the size of the set which represents the number of unique roots
    return roots.size();
  }

  void insertIfNew(int k) {
    // Inserts into the set if it is a new element
    if(!this.parents.containsKey(k)) 
      this.insert(k);
  }

  int find(int key) {
    // Finds the root of the set the key belongs to and returns it
    if(this.parents.get(key) != key) {
      // Shortens the path by setting the parent of the current element to the root of the set
      this.parents.put(key, this.find(this.parents.get(key)));
    }
    return this.parents.get(key);
  }

  void union(int k1, int k2) {
    int root1 = this.find(k1);
    int root2 = this.find(k2);

    // If the roots are the same, the elements are already in the same set
    if(root1 != root2) {
      int rank1 = this.ranks.get(root1);
      int rank2 = this.ranks.get(root2);
      if(rank1 < rank2) {
        // If the rank of the first set is less than the rank of the second set, 
        // make the second set the parent of the first set
        this.parents.put(root1, root2);
      } else if (rank2 < rank1) {
        // If the rank of the second set is less than the rank of the first set, 
        // make the first set the parent of the second set
        this.parents.put(root2, root1);
      } else {
        // Otherwise, make one set the parent of the other and increase the rank of the new parent
        this.parents.put(root2, root1);
        this.ranks.put(root1, rank2+1);
      }
    }
  }
}

class Solution {
  List<Integer> primeFactorization(int n) {
    // Calculates the prime factors of a number
    List<Integer> factors = new ArrayList<>();
    for(int i = 2; i*i <= n; i++) {
      if(n%i == 0) {
        factors.add(i);
        while(n%i == 0) n/= i;
      }
    }
    // If no prime factors are found, the number is prime
    if(n > 1) factors.add(n);
    return factors;
  }
  public boolean canTraverseAllPairs(int[] nums) {
    // Edge case: if there is only one element, it is always possible to traverse all pairs
    if(nums.length == 1) return true;

    // Stores the unique prime factors of the numbers in the input array
    Set<Integer> primeFactors = new HashSet<>();
    for(int n: nums) {
      if(n == 1) return false;
      List<Integer> factors = primeFactorization(n);
      primeFactors.addAll(factors);
    }

    // Create a disjoint set with the unique prime factors
    DisjointSet dj = new DisjointSet(primeFactors);

    for(int n: nums) {
      // For each number, union the prime factors of the number
      List<Integer> factors = primeFactorization(n);
      for(int i = 0; i < factors.size()-1; i++) {
        dj.union(factors.get(i), factors.get(i+1));
      }
    }

    // If there is only one set, it is possible to traverse all pairs
    return dj.countSets() == 1;
  }
}
