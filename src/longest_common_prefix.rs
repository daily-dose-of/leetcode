pub struct LongestCommonPrefixSolution {}

impl LongestCommonPrefixSolution {
    /// Finds the longest common prefix string amongst an array of strings.
    ///
    /// # Arguments
    ///
    /// * `strs` - A vector of strings to check for a common prefix.
    ///
    /// # Returns
    ///
    /// A `String` representing the longest common prefix amongst the strings in `strs`. If there is no common prefix,
    /// an empty string is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_rs::longest_common_prefix::LongestCommonPrefixSolution;
    ///
    /// let input1 = vec![String::from("flower"), String::from("flow"), String::from("flight")];
    /// assert_eq!(LongestCommonPrefixSolution::longest_common_prefix(input1), "fl".to_string());
    ///
    /// let input2 = vec![String::from("dog"), String::from("racecar"), String::from("car")];
    /// assert_eq!(LongestCommonPrefixSolution::longest_common_prefix(input2), "".to_string());
    ///
    /// let input3 = vec![String::from("apple"), String::from("application"), String::from("app")];
    /// assert_eq!(LongestCommonPrefixSolution::longest_common_prefix(input3), "app".to_string());
    /// ```
    ///
    /// # Time complexity
    /// O(m x n) where `n` is the number of strings in `strs` and `m` is the length of the shortest string.
    ///
    /// # Space complexity
    /// O(m) where `m` is the length of the shortest string.
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // If `strs` is empty, return an empty string
        if strs.is_empty() {
            return String::new();
        }
        // Initialize `prefix` as the first string in `strs`
        let mut prefix = strs[0].clone();
        // Loop through each string `s` in `strs`
        for s in strs {
            // While `s` does not start with `prefix`, remove the last character from `prefix`
            while !s.starts_with(&prefix) {
                prefix.pop();
                // If `prefix` becomes empty, return an empty string
                if prefix.is_empty() {
                    return String::new();
                }
            }
        }
        // Return the final `prefix` string
        prefix
    }
}
