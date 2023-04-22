pub struct FirstBadVersionSolution1 {}

impl FirstBadVersionSolution1 {
    /// Finds the first bad version given an API to check if a given version is bad.
    ///
    /// # Arguments
    /// * `n` - The number of versions.
    ///
    /// # Returns
    /// (`i32`): The first bad version.
    ///
    /// # Time complexity
    ///
    /// O(log n), where n is the number of versions.
    ///
    /// # Space complexity
    ///
    /// O(1).
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_rs::first_bad_version::FirstBadVersionSolution1;
    /// let n = 5;
    /// let bad = 4;
    /// let sol = FirstBadVersionSolution1{};
    /// let first_bad_version = sol.first_bad_version(n);
    /// assert_eq!(first_bad_version, 4);
    /// ```
    pub fn first_bad_version(&self, mut n: i32) -> i32 {
        // making `n` mutable allows us to modify its value inside the loop without having to create a new variable.
        let mut left = 1;
        while left < n {
            // the bitwise right shift operator `>>` is faster than the division operator `/`
            let mid = left + ((n - left) >> 1);
            match self.is_bad_version(mid) {
                true => n = mid,
                _ => left = mid + 1,
            }
        }
        left
    }
    /// A dummy function that determines whether a given version is the first bad version.
    ///
    /// # Arguments
    /// * `version` - An integer representing the version number to be checked.
    ///
    /// # Returns
    /// (`bool`):  Returns `true` if the version is the first bad version, and `false`
    pub fn is_bad_version(&self, version: i32) -> bool {
        version >= 4
    }
}
