/*
Question:
There are some spherical balloons taped onto a flat wall that represents the XY-plane. 
The balloons are represented as a 2D integer array points where 
points[i] = [xstart, xend] denotes a balloon whose horizontal diameter stretches between xstart and xend. 
You do not know the exact y-coordinates of the balloons.

Arrows can be shot up directly vertically (in the positive y-direction) from different points along the x-axis. 
A balloon with xstart and xend is burst by an arrow shot at x if xstart <= x <= xend. 
There is no limit to the number of arrows that can be shot. A shot arrow keeps traveling up infinitely, bursting any balloons in its path.

Given the array points, return the minimum number of arrows that must be shot to burst all balloons.
*/

// Approach:
// Sort the points based on the start of the balloon
// 
// Then we iterate through the points and keep track of the end of the current balloon
// If the start of the next balloon is greater than the end of the current balloon, we need to shoot another arrow
// Otherwise, we update the end of the current balloon to be the minimum of the current end and the end of the next balloon
//
// After we iterate through all the points, we return the number of arrows

class Solution {
  public int findMinArrowShots(int[][] points) {
    Arrays.sort(points, new Comparator<int[]>() {
      @Override
      public int compare(int[] a, int[] b) {
        return Integer.compare(a[0], b[0]);
      }
    });

    int i = points[0][1];
    int arrows = 1;

    for(int[] p: points) {
      if(p[0] > i) {
        arrows++;
        i = p[1];
      } else {
        i = Math.min(i, p[1]);
      }
    }

    return arrows;
  }
}
