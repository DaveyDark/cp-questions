/*
Question:
An image smoother is a filter of the size 3 x 3 that can be applied to each cell of an image by
rounding down the average of the cell and the eight surrounding cells (i.e.,
the average of the nine cells in the blue smoother).
If one or more of the surrounding cells of a cell is not present,
we do not consider it in the average (i.e., the average of the four cells in the red smoother).
*/

// Approach:
// 1) Create a new image of the same size as the original image.
// 2) Iterate over the cells of the new image and calculate the average of the cell and set it
// 3) Return the new image

impl Solution {
    fn average_cell(img: &Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
        let adj = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut sum = 0;
        let mut cnt = 0;
        for (x, y) in &adj {
            let (i, j) = (i + x, j + y);
            if i < 0 || j < 0 || i >= img.len() as i32 || j >= img[0].len() as i32 {
                continue;
            }
            sum += img[i as usize][j as usize];
            cnt += 1;
        }

        sum / cnt
    }
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut new_img = vec![vec![0; img[0].len()]; img.len()];
        for i in 0..img.len() {
            for j in 0..img[0].len() {
                new_img[i][j] = Self::average_cell(&img, i as i32, j as i32);
            }
        }
        new_img
    }
}
