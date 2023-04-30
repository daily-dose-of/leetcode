pub struct Solution {}

impl Solution {
    /// Given a list of tasks represented by characters, this function calculates the least number
    /// of units of time required to finish all the tasks, where each task takes one unit of time.
    /// There must be a cooldown period of `n` units of time between two same tasks.
    ///
    /// # Arguments
    ///
    /// * `tasks` - A vector of characters representing the tasks.
    /// * `n` - A non-negative integer representing the cooldown period between two same tasks.
    ///
    /// # Returns
    ///
    /// The least number of units of time that the CPU will take to finish all the given tasks.
    ///
    /// # Examples
    ///
    /// ```
    /// # use leet_rs::task_scheduler::Solution;
    /// let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
    /// let n = 2;
    /// let result = Solution::least_interval(tasks, n);
    /// assert_eq!(result, 8);
    /// ```
    ///
    /// # Time complexity:
    ///
    /// O(n log n), where n is the length of the tasks vector.
    ///
    /// # Space complexity:
    ///
    /// O(1), since we use a fixed-size array of length 26.
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        // Initialize a fixed-size array with 26 elements to store the frequency of each task.
        let mut d: [i32; 26] = [0; 26];
        
        // Count the frequency of each task and store it in the array.
        for &task in tasks.iter() {
            d[(task as u8 - b'A') as usize] += 1;
        }

        // Sort the array in ascending order to find the task with maximum frequency.
        d.sort_unstable();

        // Calculate the maximum value of idle slots.
        let max_val = d[25] - 1;

        // Calculate the total number of idle slots.
        let mut idle_slots = max_val * n as i32;

        // Subtract the number of idle slots required for each task from the total number of idle slots.
        for i in 0..25 {
            idle_slots -= std::cmp::min(d[i], max_val);
        }

        // Return the maximum of the number of idle slots and the length of the tasks vector.
        std::cmp::max(idle_slots + tasks.len() as i32, tasks.len() as i32)
    }
}
