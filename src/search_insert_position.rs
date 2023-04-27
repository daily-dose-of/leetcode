use std::cmp::Ordering;

pub struct SearchInsertPositionSolution {}

impl SearchInsertPositionSolution {
    /// Given a sorted array of distinct integers and a target value, returns the index if the target is found. If not, returns
    /// the index where it would be if it were inserted in order.
    /// This solution uses binary search to search for the target in the array. If the target is not found, the function returns
    /// the index of the first element greater than the target, which is where it should be inserted.
    ///
    /// # Arguments
    /// * nums - A sorted vector of integers to search for the target in
    /// * target - The target integer to search for in the vector
    ///
    /// # Returns
    /// Returns an integer representing the index of the target in the vector if found. If not, returns an integer representing
    /// the index where the target should be inserted.
    ///
    /// # Examples
    /// ```
    /// use leet_rs::search_insert_position::SearchInsertPositionSolution;
    /// assert_eq!(SearchInsertPositionSolution::search_insert(vec![1, 3, 5, 6], 5), 2);
    /// assert_eq!(SearchInsertPositionSolution::search_insert(vec![1, 3, 5, 6], 2), 1);
    /// assert_eq!(SearchInsertPositionSolution::search_insert(vec![1, 3, 5, 6], 7), 4);
    /// ```
    /// # Complexity
    ///
    /// # Time complexity
    ///  O(log n) - Binary search algorithm to search the array.
    ///
    /// # Space complexity
    ///  O(1) - Constant space.
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // Initialize the left pointer to 0
        let mut left = 0 as isize;
        // Initialize the right pointer to the last index in the array
        let mut right = nums.len() as isize - 1;

        // loop as long as left pointer is less than or equal to the right pointer
        while left <= right {
            // calculate the middle pointer
            let mid = left + (right - left) / 2;
            // compare the value at the middle pointer to the target
            match nums.get(mid as usize).unwrap_or(&i32::MAX).cmp(&target) {
                Ordering::Less => left = mid + 1,
                Ordering::Greater => right = mid - 1,
                Ordering::Equal => return mid as i32,
            }
        }
        // return the left pointer as the position to insert the target
        left as i32
    }
}
