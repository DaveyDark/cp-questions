/*
Question:
You are given an array coordinates, coordinates[i] = [x, y], where [x, y] represents the coordinate of a point. Check if these points make a straight line in the XY plane.
*/

// Approach:
// Points in the same line will have the same slope
// Slope = Delta Y/ Delta X, where delta is change
// Therefore for Three points in a straight line, a, b, c, Slope(a,b) = Slope(b,c)
// (y(b) - y(a))/(x(b) - x(a)) = (y(c) - y(b))/(x(c) - x(b))
// But this can cause division by zero, so we convert this to
// (y(b) - y(a)) * (x(c) - x(b)) = (y(c) - y(b)) * (x(b) - x(a))
// 1) Calculate delta_x and delta_y for the first two points
// 2) Go through the remaining points and check that the slope is equal for each pair by using above formula
//    Return false if it isn't
// 3) At the end, return true

class Solution {
    public boolean checkStraightLine(int[][] coordinates) {
        int delta_x = coordinates[1][0] - coordinates[0][0];
        int delta_y = coordinates[1][1] - coordinates[0][1];
        for(int i=2; i<coordinates.length; i++) {
            if(delta_x*(coordinates[i][1] - coordinates[i-1][1]) != delta_y*(coordinates[i][0] - coordinates[i-1][0])){
                return false;
            }
        }
        return true;
    }
}
