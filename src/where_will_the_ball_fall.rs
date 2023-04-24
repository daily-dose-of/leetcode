pub struct WhereWillTheBallFallSolution {}

impl WhereWillTheBallFallSolution {
    /// Finds the final position of the ball in the given grid.
    ///
    /// The ball starts at the top of the grid and moves according to the following rules:
    /// * If the ball lands on a hole, it stops moving and is stuck in the hole.
    /// * If the ball hits a wall, it bounces back in the opposite direction.
    /// * If the ball hits a block diagonally, it bounces off the block and continues moving in the same direction.
    /// * If the ball hits a block horizontally or vertically, it bounces off the block and changes direction
    /// 90 degrees to the right or left, depending on the direction of the block.
    ///
    /// # Arguments
    ///
    /// * grid - The grid representing the path of the ball, where 1 represents a block,
    /// -1 represents a hole, and 0 represents an empty space.
    ///
    /// # Returns
    ///
    /// A Vec<i32> representing the final position of the ball in each column of the grid.
    /// If the ball is stuck in a hole in a column, the corresponding value in the result vector will be -1.
    ///
    /// # Examples
    /// ```
    /// use leet_rs::where_will_the_ball_fall::WhereWillTheBallFallSolution;
    /// let grid = vec![ vec![1,1,1,-1,-1], vec![1,1,1,-1,-1], vec![-1,-1,-1,1,1], vec![1,1,1,1,-1], vec![-1,-1,-1,-1,-1] ];
    /// let expected = vec![1, -1, -1, -1, -1];
    /// assert_eq!(WhereWillTheBallFallSolution::find_ball(grid), expected);
    /// ```
    ///
    /// # Time complexity
    /// O(m x n), where m and n are the dimensions of the grid,
    ///
    /// # Space complexity
    /// O(n), since we only need to store the result vector of length n.
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        // Get the number of rows and columns in the grid.
        let m = grid.len();
        let n = grid[0].len();

        // Initialize the result vector with -1 for each column.
        let mut res = vec![-1; n];

        // Loop through each column.
        for col in 0..n {
            // Initialize the starting row and column for the ball.
            let mut row = 0;
            let mut cur = col;

            // Loop through each row until the ball falls out of the grid.
            while row < m {
                // Calculate the next column based on the current position and direction of the ball.
                let next = (cur as i32 + grid[row][cur]) as usize;
                // Check if the next column is out of bounds or has a different value from the current column.
                if next >= n || grid[row][next] != grid[row][cur] {
                    break;
                }
                // Update the current position of the ball.
                cur = next;
                row += 1;
            }

            // If the ball falls out of the last row, update the result vector with the final column position.
            if row == m {
                res[col] = cur as i32;
            }
        }
        // Return the result vector.
        res
    }
}
