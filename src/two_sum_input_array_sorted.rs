pub struct TwoSumIIInputArrayIsSortedSolution {}

impl TwoSumIIInputArrayIsSortedSolution {
    /// Given a sorted array of integers `numbers`, finds two numbers such that they add up to a specific target number.
    /// Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.len().
    ///
    /// # Arguments
    ///
    /// * `numbers` - A sorted vector of integers.
    /// * `target` - The target sum.
    ///
    /// # Returns
    /// A vector containing the indices of the two numbers that add up to the target, in the form [index1, index2].
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_rs::two_sum_input_array_sorted::TwoSumIIInputArrayIsSortedSolution;
    ///
    /// let numbers = vec![2, 7, 11, 15];
    /// let target = 9;
    /// assert_eq!(TwoSumIIInputArrayIsSortedSolution::two_sum(numbers, target), vec![1, 2]);
    ///
    /// let numbers = vec![2, 3, 4];
    /// let target = 6;
    /// assert_eq!(TwoSumIIInputArrayIsSortedSolution::two_sum(numbers, target), vec![1, 3]);
    ///
    /// let numbers = vec![-1, 0];
    /// let target = -1;
    /// assert_eq!(TwoSumIIInputArrayIsSortedSolution::two_sum(numbers, target), vec![1, 2]);
    /// ```
    ///
    /// # Time complexity
    /// O(n) - Since we are iterating the array once in a single while loop.
    ///
    /// # Space complexity
    /// O(1) - Since we are only using constant extra space for storing two pointers.
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        // Initialize left pointer to first element
        let mut left = 0;
        // Initialize right pointer to last element
        let mut right = numbers.len() - 1;
        // Continue until left pointer is greater than or equal to right pointer
        while left < right {
            // Compute the sum of the left and right elements
            let sum = numbers[left] + numbers[right];
            // If the sum is equal to the target, return the indices
            if sum == target {
                return vec![left as i32 + 1, right as i32 + 1];
            } else if sum < target {
                // If the sum is less than the target, move the left pointer to the right
                left += 1;
            } else {
                // If the sum is greater than the target, move the right pointer to the left
                right -= 1;
            }
        }
        // If no indices are found, return an empty vector
        vec![]
    }
}
