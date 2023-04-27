pub struct SquaresSortedArraySolution {}

impl SquaresSortedArraySolution {
    /// This function takes a vector of integers and returns the squares of each element in the vector in non-decreasing order.
    ///
    /// # Arguments
    /// * `nums` - A vector of integers.
    ///
    /// # Returns
    /// (`Vec<i32>`) A new vector containing the squares of the input vector elements in non-decreasing order.
    ///
    /// # Examples
    /// ```
    /// use leet_rs::squares_of_a_sorted_array::SquaresSortedArraySolution;
    /// let nums = vec![-4,-1,0,3,10];
    /// let result = SquaresSortedArraySolution::sorted_squares(nums);
    /// assert_eq!(result, vec![0,1,9,16,100]);
    /// ```
    ///
    /// # Time complexity
    /// O(n), where n is the length of the input vector, as it iterates over the vector once.
    ///
    /// # Space complexity
    /// O(n), where n is the length of the input vector, as it creates a new vector of the same length as the input vector.
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        // Get the length of the input vector
        let n = nums.len();
        // Initialize a result vector with the same length as the input vector
        let mut result = vec![0; n];
        // Initialize two pointers to the start and end of the vector
        let mut left = 0;
        let mut right = n as i32 - 1;
        // Initialize an index for the result vector
        let mut index = n as i32 - 1;

        // Loop until the left and right pointers meet or cross
        while left <= right {
            // Get the absolute values of the numbers pointed to by the left and right pointers
            let left_abs = nums[left as usize].abs();
            let right_abs = nums[right as usize].abs();

            // If the absolute value of the number pointed to by the left pointer is greater
            if left_abs > right_abs {
                // Square the number pointed to by the left pointer and assign it to the result vector
                result[index as usize] = left_abs * left_abs;
                // Move the left pointer to the right
                left += 1;
            } else {
                // Square the number pointed to by the right pointer and assign it to the result vector
                result[index as usize] = right_abs * right_abs;
                // Move the right pointer to the left
                right -= 1;
            }

            // Move the index of the result vector to the left
            index -= 1;
        }

        // Return the result vector
        result
    }
}
