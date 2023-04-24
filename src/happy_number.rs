pub struct HappyNumberSolution {}

impl HappyNumberSolution {
    /// Determines whether a number is happy.
    ///
    /// A happy number is a number defined by the following process:
    /// Starting with any positive integer, replace the number by the sum of the squares of its digits.
    /// Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a
    /// cycle which does not include 1.
    /// Those numbers for which this process ends in 1 are happy.
    ///
    /// # Arguments
    /// * `n` - A positive integer to check for happiness
    ///
    /// # Returns
    /// (`bool`): `true` if the input number is happy, `false` otherwise
    ///
    /// # Time complexity
    ///
    /// O(log n) - each iteration reduces the size of the input number by a factor of 10
    ///
    /// # Space complexity
    /// O(log n) - the size of the hash set used for cycle detection
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_rs::happy_number::HappyNumberSolution;
    ///
    /// let is_happy = HappyNumberSolution::is_happy(19);
    /// assert_eq!(is_happy, true);
    ///
    /// let is_happy = HappyNumberSolution::is_happy(2);
    /// assert_eq!(is_happy, false);
    /// ```
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        let mut set = std::collections::HashSet::new();
        while n != 1 && !set.contains(&n) {
            set.insert(n);
            n = Self::square_sum(n);
        }
        n == 1
    }

    /// Computes the sum of the squares of the digits of a given number.
    ///
    /// # Arguments
    ///
    /// * `n` - A positive integer whose digits should be squared and summed
    ///
    /// # Returns
    /// (`i32`) - The sum of the squares of the digits of `n`
    ///
    /// # Time complexity
    ///
    /// O(log n) - each iteration reduces the size of the input number by a factor of 10
    ///
    /// # Space complexity
    /// O(1) - constant amount of space used for the sum and digit variables
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_rs::happy_number::HappyNumberSolution;
    ///
    /// let square_num = HappyNumberSolution::square_sum(19);
    /// assert_eq!(square_num, 82);
    /// ```
    pub fn square_sum(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            let digit = n % 10;
            sum += digit * digit;
            n /= 10;
        }
        sum
    }
}
