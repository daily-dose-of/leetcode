pub struct ReverseStringSolution1 {}
pub struct ReverseStringSolution2 {}
pub struct ReverseStringSolution3 {}

impl ReverseStringSolution1 {
    /// Reverses the order of characters in a mutable vector `s`.
    ///
    /// # Arguments
    ///
    /// * `s` - A mutable vector of characters.
    ///
    /// # Returns
    ///
    /// This function does not return a value, but modifies the input vector in place.
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_rs::reverse_string::ReverseStringSolution1;
    /// let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    /// ReverseStringSolution1::reverse_string(&mut s);
    /// assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(n), where n is the length of the input vector.
    ///
    /// # Space Complexity
    ///
    /// O(1), since it only uses constant extra space.
    pub fn reverse_string(s: &mut Vec<char>) {
        fn recursive_swap(i: usize, s: &mut [char]) {
            // Base case: if i is greater than or equal to s.len() / 2, return
            if i >= s.len() / 2 {
                return;
            }
            // Swap the characters at indices i and s.len() - i - 1
            s.swap(i, s.len() - i - 1);
            // Recursively call the function with i + 1 and the updated s
            recursive_swap(i + 1, s);
        }
        recursive_swap(0, s);
    }
}

impl ReverseStringSolution2 {
    /// Reverses the order of characters in a string in-place using the two pointers approach.
    ///
    /// # Arguments
    ///
    /// * `s` - A mutable reference to the string slice to reverse.
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_rs::reverse_string::ReverseStringSolution2;
    /// let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    /// ReverseStringSolution2::reverse_string(&mut s);
    /// assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(n), where n is the length of the input string.
    ///
    /// # Space Complexity
    ///
    /// O(1), since the algorithm operates in-place.
    pub fn reverse_string(s: &mut Vec<char>) {
        let (mut left, mut right) = (0, s.len() - 1);
        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

impl ReverseStringSolution3 {
    /// Reverses the order of characters in a string in-place using the two pointers approach.
    ///
    /// # Arguments
    ///
    /// * `s` - A mutable reference to the string slice to reverse.
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_rs::reverse_string::ReverseStringSolution3;
    /// let mut s1 = vec!['h', 'e', 'l', 'l', 'o'];
    /// ReverseStringSolution3::reverse_string(&mut s1);
    /// assert_eq!(s1, vec!['o', 'l', 'l', 'e', 'h']);
    ///
    /// let mut s2 = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    /// ReverseStringSolution3::reverse_string(&mut s2);
    /// assert_eq!(s2, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(n), where n is the length of the input string.
    ///
    /// # Space Complexity
    ///
    /// O(1), since the algorithm operates in-place.
    pub fn reverse_string(s: &mut Vec<char>) {
        // Initialize two pointers left_idx and right_idx
        let mut left_idx = 0;
        let mut right_idx = s.len() as isize - 1;
        // Loop until left_idx pointer crosses right_idx pointer
        while left_idx < right_idx as usize {
            // Swap characters at left_idx and right_idx pointers using unsafe code
            // to avoid borrow checker issues
            unsafe {
                // Get a raw pointer to the left index in the slice
                let left_ptr = s.as_mut_ptr().add(left_idx);
                // Get a raw pointer to the right index in the slice
                // Add an offset to the pointer to access other elements in the vector.
                let right_ptr = s.as_mut_ptr().add(right_idx as usize);
                // Swap the characters at the left and right indices using the pointers
                left_ptr.swap(right_ptr);
            }
            // Move left_idx pointer forward and right_idx pointer backward
            left_idx += 1;
            right_idx -= 1;
        }
    }
}
