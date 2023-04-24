pub struct RunningSumof1dArraySolution1 {}
pub struct RunningSumof1dArraySolution2 {}
pub struct RunningSumof1dArraySolution3 {}

impl RunningSumof1dArraySolution1 {
    /// This function computes the running sum of an array.
    /// It takes a mutable reference to the input array `nums`
    /// and modifies it in-place.
    /// # Arguments
    /// * `nums` - A mutable reference to a vector of integers.
    ///
    /// # Returns
    /// (`Vec<i32>`): A mutable reference to the same array that was passed in, with each element
    /// replaced by the running sum of all preceding elements in the array (including
    /// the element itself).
    ///
    /// # Time Complexity
    ///
    /// O(n), where n is the length of the input array
    ///
    /// # Space Complexity
    ///
    /// O(1), Operations made in place
    ///
    /// # Examples
    /// ```
    /// use leet_rs::running_sum_of_1d_array::RunningSumof1dArraySolution1;
    /// let mut nums = vec![1, 2, 3, 4];
    /// nums = RunningSumof1dArraySolution1::running_sum(nums.clone());
    /// assert_eq!(nums, vec![1, 3, 6, 10]);
    /// ```
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        nums
    }
}

impl RunningSumof1dArraySolution2 {
    /// Computes the running sum of the input array by iterating over the array and adding
    /// each element to the sum of the previous elements. Uses an iterator to avoid manual indexing.
    ///
    /// # Arguments
    ///
    /// * `nums` - A vector of integers representing the input array
    ///
    /// # Returns
    /// (`Vec<i32>`): A vector of integers representing the running sum of the input array
    ///
    /// # Time Complexity
    ///
    /// O(n), where n is the length of the input array
    ///
    /// # Space Complexity
    ///
    /// O(n), where n is the length of the input array
    ///
    ///
    /// # Examples
    /// ```
    /// use leet_rs::running_sum_of_1d_array::RunningSumof1dArraySolution2;
    /// let nums = vec![1, 2, 3, 4];
    /// let result = RunningSumof1dArraySolution2::running_sum(nums.clone());
    /// assert_eq!(result, vec![1, 3, 6, 10]);
    /// ```
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        nums.iter()
            .map(|&x| {
                sum += x;
                sum
            })
            .collect()
    }
}

impl RunningSumof1dArraySolution3 {
    /// Computes the running sum of the input array by iterating over the array and adding
    /// each element to the sum of the previous elements. Uses a for loop to avoid manual indexing.
    ///
    /// # Arguments
    ///
    /// * `nums` - A vector of integers representing the input array
    ///
    /// # Returns
    /// (`Vec<i32>`): A vector of integers representing the running sum of the input array
    ///
    /// # Time Complexity
    ///
    /// O(n), where n is the length of the input array
    ///
    /// # Space Complexity
    ///
    /// O(n), where n is the length of the input array
    ///
    ///
    /// # Examples
    /// ```
    /// use leet_rs::running_sum_of_1d_array::RunningSumof1dArraySolution3;
    /// let nums = vec![1, 2, 3, 4];
    /// let result = RunningSumof1dArraySolution3::running_sum(nums.clone());
    /// assert_eq!(result, vec![1, 3, 6, 10]);
    /// ```
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let mut sum = 0;
        for (i, num) in nums.iter().enumerate() {
            sum += num;
            res[i] = sum;
        }
        res
    }
}
