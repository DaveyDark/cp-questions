/*
Question:
You are given an array of non-overlapping intervals intervals 
where intervals[i] = [starti, endi] represent the start and the end of the ith interval 
and intervals is sorted in ascending order by starti. 
You are also given an interval newInterval = [start, end] that represents the start and end of another interval.

Insert newInterval into intervals such that intervals is still sorted in ascending order by starti and 
intervals still does not have any overlapping intervals (merge overlapping intervals if necessary).

Return intervals after the insertion.

Note that you don't need to modify intervals in-place. You can make a new array and return it.
*/

// Approach:
// W create an answer list to store the merged intervals.
// We traverse the intervals one by one and check the following conditions:
//
// - If the current interval lies before the new interval, we add it to the answer.
// - If the current interval lies after the new interval,
//     We add the new interval if it isn't added yet and then add the current interval.
// - Otherwise, there is an overlap
//     So we update the new interval to contain the overlapping intervals.
//
// If the new interval isn't added yet after the traversal, we add it to the answer.
//
// Finally, we return the answer as a 2D array.

class Solution {
  public int[][] insert(int[][] intervals, int[] newInterval) {
    List<int[]> merged = new ArrayList<>();
    boolean added = false;

    for(int[] interval: intervals) {
      if(interval[1] < newInterval[0])
        merged.add(interval);
      else if(interval[0] > newInterval[1]) {
        if(!added) {
          added = true;
          merged.add(newInterval);
        }
        merged.add(interval);
      } else {
        newInterval[0] = Math.min(newInterval[0], interval[0]);
        newInterval[1] = Math.max(newInterval[1], interval[1]);
      }
    }

    if(!added)
      merged.add(newInterval);

    return merged.toArray(new int[0][2]);
  }
}
