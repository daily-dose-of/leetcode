pub struct FindPivotIndexSolution1 {}

impl FindPivotIndexSolution1 {
    /// Find the pivot index of an array, return -1 if not found.
    /// The pivot index is the index where the sum of the numbers to the left of the index
    /// is equal to the sum of the numbers to the right of the index.
    ///
    /// # Arguments
    /// * `nums` -  A vector of integers.
    ///
    /// # Returns
    /// (`i32`): An integer representing the pivot index of the input array.
    /// If no pivot index exists, return -1.
    ///
    /// # Time Complexity
    /// O(n): where n is the length of the input array.
    ///
    /// # Space Complexity
    /// O(1): as it only uses a constant amount of additional space.
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_rs::find_pivot_index::FindPivotIndexSolution1;
    /// assert_eq!(FindPivotIndexSolution1::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    /// assert_eq!(FindPivotIndexSolution1::pivot_index(vec![1, 2, 3]), -1);
    /// assert_eq!(FindPivotIndexSolution1::pivot_index(vec![2, 1, -1]), 0);
    /// ```
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut total_sum = 0;
        let mut left_sum = 0;

        for i in 0..nums.len() {
            total_sum += nums[i];
        }
        // calculate the total_sum and left_sum in the same loop
        for i in 0..nums.len() {
            // This reduces the number of operations and eliminates the need to store
            // the right_sum values in a separate array variable.
            // right_sum = total_sum - left_sum - nums[i]
            if left_sum == total_sum - left_sum - nums[i] {
                return i as i32;
            }
            left_sum += nums[i];
        }

        -1
    }
}
