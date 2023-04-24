use std::cmp::Ordering;

pub struct BinarySearchSolution1 {}

impl BinarySearchSolution1 {
    /// This function searches for target in a sorted array of integers using binary search algorithm
    /// # Arguments
    /// * `nums` -  a vector of integers that is sorted in ascending order
    /// * `target` - the integer to search for
    ///
    /// # Returns
    /// (`i32`): the index of the target if it exists in the array, otherwise -1
    ///
    /// # Time Complexity
    ///
    /// O(log n) - the function performs binary search on the input vector
    ///
    /// # Space Complexity
    ///
    /// O(1) - the function uses constant amount of extra space
    ///
    /// # Examples
    /// ```
    /// use leet_rs::binary_search::BinarySearchSolution1;
    ///  assert_eq!(BinarySearchSolution1::search(vec![-1,0,3,5,9,12], 9), 4);
    /// ```
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;

        // Inclusive range left <= right to handle the case when nums has only one element.
        while left <= right {
            let mid = left + (right - left) / 2;

            // `cmp` method of the i32 type to compare the middle element with the target value,
            // and the match expression to handle the different cases
            match nums[mid as usize].cmp(&target) {
                Ordering::Less => left = mid + 1,
                Ordering::Greater => right = mid - 1,
                Ordering::Equal => return mid,
            }
        }

        -1
    }
}
