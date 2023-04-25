pub struct ReverseWordsStringIIISolution {}

impl ReverseWordsStringIIISolution {
    /// Reverses the order of characters in each word within a sentence while preserving whitespace and initial word order.
    ///
    /// # Arguments
    ///
    /// * `s` - A string of printable ASCII characters that does not contain any leading or trailing spaces and has at least one word. All the words in the string are separated by a single space.
    ///
    /// # Returns
    ///
    /// A string with reversed order of characters in each word, while preserving whitespace and initial word order.
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_rs::reverse_words_in_a_string_iii::ReverseWordsStringIIISolution;
    /// let result = ReverseWordsStringIIISolution::reverse_words("Let's take LeetCode contest".to_string());
    /// assert_eq!(result, "s'teL ekat edoCteeL tsetnoc");
    ///
    /// let result = ReverseWordsStringIIISolution::reverse_words("God Ding".to_string());
    /// assert_eq!(result, "doG gniD");
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(n), where n is the length of the input string `s`.
    ///
    /// # Space Complexity
    ///
    /// O(n), where n is the length of the input string `s`.
    pub fn reverse_words(s: String) -> String {
        let mut chars = s.chars().collect::<Vec<_>>();
        let mut left = 0;
        let mut right = 0;
        
        while right < chars.len() {
            if chars[right] == ' ' {
                // Reverse the characters in the current word
                chars[left..right].reverse();
                left = right + 1;
            }
            right += 1;
        }
        
        // Reverse the characters in the last word
        chars[left..right].reverse();
        
        // Convert the characters back to a string
        chars.into_iter().collect()
    }
}
