/*
Question:
In a town, there are n people labeled from 1 to n. There is a rumor that one of these people is secretly the town judge.

If the town judge exists, then:
    The town judge trusts nobody.
    Everybody (except for the town judge) trusts the town judge.
    There is exactly one person that satisfies properties 1 and 2.

You are given an array trust where trust[i] = [ai, bi] representing that the person labeled ai trusts the person labeled bi. 
If a trust relationship does not exist in trust array, then such a trust relationship does not exist.

Return the label of the town judge if the town judge exists and can be identified, or return -1 otherwise.
*/ 

// Approach:
// 1) Make arrays to count the number of trusts and trusted people for each person
// 2) Iterate through the arrays and store the counts in the arrays
// 3) Iterate from 0 to n and find the person who is trusted by everyone else and trusts no one
//     Return the person if found
// 4) Otherwise return -1

class Solution {
  public int findJudge(int n, int[][] trust) {
    int trusts[] = new int[n];
    int trusted[] = new int[n];
    for(int[] t: trust) {
      trusts[t[0]-1]++;
      trusted[t[1]-1]++;
    }
    for(int i = 0; i < n; i++) {
      if(trusts[i] == 0 && trusted[i] == n-1) return i+1;
    }
    return -1;
  }
}
