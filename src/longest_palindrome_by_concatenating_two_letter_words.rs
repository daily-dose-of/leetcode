use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    /// Given a list of lowercase words, finds the length of the longest palindrome that can be
    /// created by concatenating any two words in the list. A palindrome is a word that is spelled
    /// the same way forwards and backwards.
    ///
    /// This solution uses a hashmap to keep track of the number of times a word appears in the input vector.
    /// For each word, it checks whether its reverse is present in the hashmap. If not, it adds the reverse of
    /// the word to the hashmap with a count of 1. If it is, it removes the reverse of the word from the hashmap
    /// and increments the length variable by 4 (since it has found a palindrome of length 4). Finally, it checks
    /// for the first set of double letters in the remaining unmatched words and adds 2 to the length variable if found.
    ///
    /// # Arguments
    /// * `words` - a vector of strings representing the words to be checked for palindromes
    ///
    /// # Returns
    /// The length of the longest palindrome that can be created by concatenating any two words
    /// in the input list.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use leet_rs::longest_palindrome_by_concatenating_two_letter_words::Solution;
    ///
    /// let words = vec!["lc".to_string(), "cl".to_string(), "gg".to_string()];
    /// assert_eq!(Solution::longest_palindrome(words), 6);
    /// ```
    ///
    /// # Complexity
    ///
    /// # Time complexity
    /// O(n^2), because the implementation loops through the list of input words twice.
    ///
    /// # Space complexity
    /// O(n), because the implementation uses a HashMap to store counts of unmatched words.
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut length = 0;
        let mut unmatched_words: HashMap<String, u32> = HashMap::new();
        // Iterate over each word in the input vector

        for word in words {
            // If the word has been encountered as an unmatched double letter, remove it from the hashmap
            // and add 4 to the length of the longest palindrome
            if let Some(count) = unmatched_words.get_mut(&word) {
                length += 4;
                if *count == 1 {
                    unmatched_words.remove(&word);
                } else { // If the word has not been encountered as an unmatched double letter, add it to the hashmap

                    *count -= 1;
                }
            } else {
                *unmatched_words.entry(word.chars().rev().collect()).or_insert(0) += 1;
            }
        }

        // Iterate over the remaining unmatched double letters
        for (word, _) in unmatched_words {
            let chars: Vec<char> = word.chars().collect();
                        // If the first and second characters of the word are the same, add 2 to the length of the longest palindrome

            if chars[0] == chars[1] {
                length += 2;
                break;
            }
        }

        length
    }
}
