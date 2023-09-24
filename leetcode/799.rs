/*
Question:
We stack glasses in a pyramid, where the first row has 1 glass, 
the second row has 2 glasses, and so on until the 100th row.  
Each glass holds one cup of champagne.

Then, some champagne is poured into the first glass at the top.  
When the topmost glass is full, any excess liquid poured will fall equally to the glass 
immediately to the left and right of it.  When those glasses become full, 
any excess champagne will fall equally to the left and right of those glasses, and so on.  
(A glass at the bottom row has its excess champagne fall on the floor.)

For example, after one cup of champagne is poured, the top most glass is full.  
After two cups of champagne are poured, the two glasses on the second row are half full.  
After three cups of champagne are poured, those two cups become full - there are 3 full glasses total now.  
After four cups of champagne are poured, the third row has the middle glass half full, 
and the two outside glasses are a quarter full, as pictured below.
*/

// Approach:
// 1) Make a vector with two rows to track the relevant 2 rows of the tower, and a counter to track
//    the index of the row we are on
// 2) Loop query_row times and do the following:
//      Swap the two rows, post 2 more 0s to the second row and fill it with 0s
//      Loop through the first row and calculate the amount spilled which will be the liquid
//      contained-1, or 0 it is < 1.
//      Add half of the spilled amount to the ith and (i+1)th element of the second row.
// 3) Return the query_glass element of the last row or 1 if it is > 1

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut tower = vec!(vec!(),vec!(poured as f64));
        for _ in 0..query_row {
            tower.swap(0,1);
            tower[1].push(0.0);
            tower[1].push(0.0);
            tower[1].fill(0.0);
            for i in 0..tower[0].len() {
                let spilled = (tower[0][i] - 1.0).max(0.0);
                tower[1][i] += spilled/2.0;
                tower[1][i+1] += spilled/2.0;
            }
        }
        tower[1][query_glass as usize].min(1.0)
    }
}
