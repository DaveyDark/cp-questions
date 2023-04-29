/*
Question:
An image is represented by an m x n integer grid image where image[i][j] represents the pixel value of the image.

You are also given three integers sr, sc, and color. You should perform a flood fill on the image starting from the pixel image[sr][sc].

To perform a flood fill, consider the starting pixel, plus any pixels connected 4-directionally to the starting pixel of the same color as the starting pixel, plus any pixels connected 4-directionally to those pixels (also with the same color), and so on. Replace the color of all of the aforementioned pixels with color.

Return the modified image after performing the flood fill.
*/

// Approach:
// 1) We first grab the color at the target position and store it in target
// 2) Then if the target is already colored we exit
// 3) Otherwise we fill the area by calling the fill function
//    The fill function recursively changes the color of the current element to the given one
//    And then calls intself on adjacent cells that have the target color

impl Solution {
    pub fn fill(image: &mut Vec<Vec<i32>>, sr: i32,sc: i32, color: i32, target: i32){
        image[sr as usize][sc as usize] = color;
        for i in -1..=1{
            for j in -1..=1 {
                if j != 0 && i != 0 {continue;}
                if sr+i < 0 || sr+i >= image.len() as i32 || sc+j < 0 || sc+j >= image[0].len() as i32 {continue;}
                if image[(sr+i) as usize][(sc+j) as usize] == target {Solution::fill(image,sr+i,sc+j,color, target);}
            }
        }
    }
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image.clone();
        let target = image[sr as usize][sc as usize];
        if target == color {
            return image;
        }
        Solution::fill(&mut image,sr,sc,color,target);
        image
    }
}
