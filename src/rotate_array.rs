pub struct RotateArraySolution {}

impl RotateArraySolution {
    /// Rotate the elements of the given vector `nums` to the right by `k` steps.
    ///
    /// # Arguments
    ///
    /// * `nums` - A mutable reference to a vector of integers.
    /// * `k` - The number of steps to rotate the elements by.
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_rs::rotate_array::RotateArraySolution;
    /// let mut nums = vec![1, 2, 3, 4, 5];
    /// RotateArraySolution::rotate(&mut nums, 3);
    /// assert_eq!(nums, vec![3, 4, 5, 1, 2]);
    /// ```
    ///
    /// # Time Complexity
    ///
    /// The time complexity of this function is O(n), where n is the length of the vector `nums`.
    ///
    /// # Space Complexity
    ///
    /// The space complexity of this function is O(1), since it performs the rotation in place.
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        // Calculate the effective rotation amount based on the length of the vector.
        let k = k as usize % nums.len();

        // Initialize a counter to keep track of the number of elements that have been rotated.
        let mut count = 0;
        
        // Iterate over each index in the vector.
        for start in 0..nums.len() {
            // Initialize variables to keep track of the current element and the previous element in the rotation.
            let mut current = start;
            let mut prev = nums[start];
            
            // Continue rotating elements until all elements have been rotated.
            while count < nums.len() {
                // Calculate the index of the next element to be rotated.
                let next = (current + k) % nums.len();
                
                // Swap the current and next elements.
                let temp = nums[next];
                nums[next] = prev;
                prev = temp;
                
                // Update the current index and the number of elements that have been rotated.
                current = next;
                count += 1;
                
                // If we've completed a full rotation cycle, break out of the inner loop.
                if start == current {
                    break;
                }
            }
        }
    }
}