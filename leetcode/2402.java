/*
Question:
You are given an integer n. There are n rooms numbered from 0 to n - 1.

You are given a 2D integer array meetings where meetings[i] = [starti, endi] 
means that a meeting will be held during the half-closed time interval [starti, endi). 
All the values of starti are unique.

Meetings are allocated to rooms in the following manner:
    Each meeting will take place in the unused room with the lowest number.
    If there are no available rooms, the meeting will be delayed until a room becomes free. 
    The delayed meeting should have the same duration as the original meeting.
    When a room becomes unused, meetings that have an earlier original start time should be given the room.

Return the number of the room that held the most meetings. If there are multiple rooms, return the room with the lowest number.

A half-closed interval [a, b) is the interval between a and b including a and not including b.
*/

// Approach:
// 1) Create an array of size n to store the time when each room will be free
//    By default, all rooms are free at time 0
// 2) Create an array of size n to store the number of meetings conducted in each room
// 3) Sort the meetings by start time
// 4) Iterate through each meeting
//     Iterate through each room
//      If a room is free, i.e. rooms[i] < meet[0], then conduct the meeting in this room
//      Otherwise, keep track of the room which will be free earliest
//      If no free room was found, then conduct the meeting in the room which will be free earliest
// 5) Return the index of the room which conducted the most meetings

class Solution {
  int maxIndex(int[] arr) {
    int max = arr[0];
    int maxI = 0;
    for(int i = 0; i < arr.length; i++) {
      if(arr[i] > max) {
        max = arr[i];
        maxI = i;
      }
    }
    return maxI;
  }
  public int mostBooked(int n, int[][] meetings) {
    long[] rooms = new long[n]; // rooms[i] is the time ith room will be free
    int[] meets = new int[n]; // number of meets conducted in each room

    // sort meetings by start time
    Arrays.sort(meetings, new Comparator<int[]>() {
      @Override
      public int compare(int[] o1, int[] o2) {
        return Integer.compare(o1[0], o2[0]);
      }
    });

    // conduct each meeting
    for(int[] meet: meetings) {
      int time = meet[0]; // start time of current meet
      long wait = Long.MAX_VALUE; // amout of time spent waiting
      int waitRoom = 0; // the room we will wait for to get free
      for(int i = 0; i < n; i++) {
        if(rooms[i] <= meet[0]) {
          // we can use this room without waiting
          rooms[i] = meet[1];
          wait = 0;
          meets[i]++;
          break;
        } else if(wait > rooms[i]) {
          // keep track if the room which will get freed earliest in case no free room is found
          wait = rooms[i];
          waitRoom = i;
        }
      }
      if(wait != 0) {
        // no room was free, meeting is delayed
        rooms[waitRoom] = wait + (meet[1] - meet[0]);
        meets[waitRoom]++;
      }
    }

    return maxIndex(meets);
  }
}
