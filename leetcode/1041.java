/*
On an infinite plane, a robot initially stands at (0, 0) and faces north. Note that:
    The north direction is the positive direction of the y-axis.
    The south direction is the negative direction of the y-axis.
    The east direction is the positive direction of the x-axis.
    The west direction is the negative direction of the x-axis.
The robot can receive one of three instructions:
    "G": go straight 1 unit.
    "L": turn 90 degrees to the left (i.e., anti-clockwise direction).
    "R": turn 90 degrees to the right (i.e., clockwise direction).
The robot performs the instructions given in order, and repeats them forever.
Return true if and only if there exists a circle in the plane such that the robot never leaves the circle.
*/

// Approach:
// 1) We track the current position of the robot using an array to store the x/y coordinates and the direction using an int
// 2) We go over the instructions char by char
//    If the command is go then we update the position according to the current direction
//    If the command is L or R we upadate the direction
// 3) At the end we check if the robot has rotated or if it has returned to the start, if either is true then it is in a cycle
//    Otherwise we return false at then end

class Solution {
    public boolean isRobotBounded(String instructions) {
        int pos[] = {0,0};
        int dir=0;
        // 0 is north
        // 1 is east
        // 2 is south
        // 3 is west
        for(char ch: instructions.toCharArray()) {
            switch (ch) {
                case 'G':
                    if(dir == 0)pos[1]++;
                    else if(dir == 1)pos[0]++;
                    else if(dir == 2)pos[1]--;
                    else if(dir == 3)pos[0]--;
                    break;
                case 'L':
                    dir--;
                    if(dir<0)dir=3;
                    break;
                case 'R':
                    dir++;
                    if(dir>3)dir=0;
                    break;
            }
        }
        if(dir != 0 ||( pos[0] == 0 && pos[1] == 0))return true;
        return false;
    }
}
