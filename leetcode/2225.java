/*
Question:
You are given an integer array matches where matches[i] = [winneri, loseri] indicates that the player winneri defeated player loseri in a match.

Return a list answer of size 2 where:
    answer[0] is a list of all players that have not lost any matches.
    answer[1] is a list of all players that have lost exactly one match.

The values in the two lists should be returned in increasing order.

Note:
    You should only consider the players that have played at least one match.
    The testcases will be generated such that no two matches will have the same outcome.
*/

// Approach:
// 1) Create a hashmap of freq of each player
// 2) Iterate over the matches and update the freq of each player
// 3) Iterate over the hashmap and find players with freq 0 and 1
// 4) Sort the players with freq 0 and 1
// 5) Return the result

class Solution {
  public List<List<Integer>> findWinners(int[][] matches) {
    HashMap<Integer, Integer> freq = new HashMap<>();
    for(int[] match: matches) {
      freq.put(match[1], freq.getOrDefault(match[1], 0) + 1);
      if(!freq.containsKey(match[0])) freq.put(match[0], 0);
    }

    ArrayList<Integer> zeros = new  ArrayList<>();
    ArrayList<Integer> ones = new  ArrayList<>();
    for(int k: freq.keySet()) {
      if(freq.get(k) == 0) zeros.add(k);
      else if(freq.get(k) == 1)ones.add(k);
    }

    List<List<Integer>> res = new  ArrayList<>();
    Collections.sort(zeros);
    Collections.sort(ones);
    res.add(zeros);
    res.add(ones);
    return res;
  }
}
