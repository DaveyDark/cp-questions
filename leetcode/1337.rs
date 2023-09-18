/*
Question:
You are given an m x n binary matrix mat of 1's (representing soldiers) and 0's (representing civilians). The soldiers are positioned in front of the civilians. That is, all the 1's will appear to the left of all the 0's in each row.

A row i is weaker than a row j if one of the following is true:
    The number of soldiers in row i is less than the number of soldiers in row j.
    Both rows have the same number of soldiers and i < j.

Return the indices of the k weakest rows in the matrix ordered from weakest to strongest.
*/

// Approach:
// 1) We convert the vec to an iterator and enumerate and collect it to convert to to a vector over
//    the idex and the values
// 2) Sort the vector by the number of ones in each row
// 3) Iterate over the vec and take the first k elements and make a vector of only their indices

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut mat = mat
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize,Vec<i32>)>>();
        mat.sort_by_key(|row| row.1.iter().sum::<i32>());
        mat
        .iter()
        .take(k as usize)
        .map(|x| x.0 as i32)
        .collect()
    }
}
