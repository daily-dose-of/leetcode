pub struct MoveZeroesSolution {}

impl MoveZeroesSolution {
    /// Moves all zeroes in the input vector to the end while maintaining the relative order of non-zero elements.
    ///
    /// # Arguments
    ///
    /// * `nums` - The vector of integers to be modified.
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_rs::move_zeroes::MoveZeroesSolution;
    ///
    /// let mut nums = vec![0, 1, 0, 3, 12];
    ///
    /// MoveZeroesSolution::move_zeroes(&mut nums);
    ///
    /// assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    /// ```
    ///
    /// # Time complexity
    /// O(n), where n is the length of the input vector.
    ///
    /// # Space complexity
    /// O(1), as it operates on the input vector in-place.
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // initialize a variable to keep track of the index where non-zero elements will be placed
        let mut non_zero_idx = 0;
        // loop through each element in the array
        for current_idx in 0..nums.len() {
            // if the element is non-zero
            if nums[current_idx] != 0 {
                // place it at the current index
                nums[non_zero_idx] = nums[current_idx];
                // increment the index for the next non-zero element
                non_zero_idx += 1;
            }
        }
        // loop through the remaining indices
        while non_zero_idx < nums.len() {
            // set the value at the current index to zero
            nums[non_zero_idx] = 0;
            // increment the index for the next zero element
            non_zero_idx += 1;
        }
    }
}
