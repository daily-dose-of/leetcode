pub struct SpiralMatrixSolution {}

impl SpiralMatrixSolution {
    /// Given a matrix of m x n elements (m rows, n columns), return all elements of the matrix in spiral order.
    ///
    /// # Arguments
    /// * `matrix` - A vector of vectors representing a matrix of integers.
    ///
    /// # Returns
    /// (`Vec<i32>`) A vector of integers representing the elements of the matrix in spiral order.
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_rs::spiral_matrix::SpiralMatrixSolution;
    ///
    /// let matrix = vec![
    ///     vec![1, 2, 3],
    ///     vec![4, 5, 6],
    ///     vec![7, 8, 9]
    /// ];
    /// let result = SpiralMatrixSolution::spiral_order(matrix);
    /// assert_eq!(result, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    /// ```
    ///
    /// # Time complexity
    /// O(m x n) where m is the number of rows and n is the number of columns in the matrix.
    ///
    /// # Space complexity
    /// O(m x n) where m is the number of rows and n is the number of columns in the matrix.
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return vec![];
        }

        let mut result = Vec::new();
        let mut top = 0;
        let mut bottom = matrix.len() - 1;
        let mut left = 0;
        let mut right = matrix[0].len() - 1;

        // Handle single row
        if bottom == 0 {
            return matrix[0].clone();
        }

        // Handle single column
        if right == 0 {
            for i in 0..=bottom {
                result.push(matrix[i][0]);
            }
            return result;
        }

        while top <= bottom && left <= right {
            // Traverse Right
            for i in left..=right {
                result.push(matrix[top][i]);
            }
            top += 1;

            // Traverse Down
            for i in top..=bottom {
                result.push(matrix[i][right]);
            }
            right -= 1;

            // Traverse Left
            if top <= bottom {
                for i in (left..=right).rev() {
                    result.push(matrix[bottom][i]);
                }
                bottom -= 1;
            }

            // Traverse Up
            if left <= right {
                for i in (top..=bottom).rev() {
                    result.push(matrix[i][left]);
                }
                left += 1;
            }
        }
        result
    }
}
