pub struct MultiplyStringsSolution {}

impl MultiplyStringsSolution {
    /// Adds the product of two digits to a slice of digits representing a larger number.
    ///
    /// # Arguments
    /// * `res` -  A mutable reference to a slice of digits representing a larger number.
    /// * `index` -  The index in `res` where the product should be added.
    /// * `product` -  The product of two digits.
    ///
    /// # Returns
    /// None
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_rs::multiply_strings::MultiplyStringsSolution;
    /// let mut res = vec![0, 0, 0];
    /// MultiplyStringsSolution::add_product(&mut res, 1, 6);
    /// assert_eq!(res, vec![0, 6, 0]);
    /// ```
    ///
    /// # Complexity
    ///
    /// # Time complexity
    /// O(1)
    ///
    /// # Space complexity
    /// O(1)
    pub fn add_product(res: &mut [u8], index: usize, product: u8) {
        let sum = res[index] + product;
        // Update the current digit and carry the overflow to the next digit
        res[index] = sum % 10;
        let carry = sum / 10;
        if carry > 0 {
            // Carry the overflow to the next digit
            res[index - 1] += carry;
        }
    }

    /// Multiplies two strings representing non-negative integers and returns their product as a string.
    ///
    /// # Arguments
    /// * `num1` -  The first number to multiply.
    /// * `num2` -  The second number to multiply.
    ///
    /// # Returns
    /// A string representing the product of `num1` and `num2`.
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_rs::multiply_strings::MultiplyStringsSolution;
    /// let num1 = "123".to_string();
    /// let num2 = "456".to_string();
    /// let product = MultiplyStringsSolution::multiply(num1, num2);
    /// assert_eq!(product, "56088".to_string());
    /// ```
    ///
    /// # Complexity
    ///
    /// # Time complexity
    /// O(mn), where m and n are the lengths of `num1` and `num2`, respectively.
    ///
    /// # Space complexity
    /// O(m + n)
    pub fn multiply(num1: String, num2: String) -> String {
         // Convert input strings to byte arrays for faster access
        let n1 = num1.as_bytes();
        let n2 = num2.as_bytes();

        // Get lengths of input byte arrays
        let len1 = n1.len();
        let len2 = n2.len();

        // Initialize result vector with 0s
        let mut res = vec![0; len1 + len2];

        // Iterate over each digit of the first number from right to left
        for i in (0..len1).rev() {
            // Get the current digit of the first number
            let a = n1[i] - b'0';

            // Iterate over each digit of the second number from right to left
            for j in (0..len2).rev() {
                // Get the current digit of the second number
                let b = n2[j] - b'0';

                // Calculate the product of the two digits
                let product = a * b;

                // Add the product to the current digit of the result vector
                Self::add_product(&mut res, i + j + 1, product);
            }
        }

        // Remove any leading zeroes from the result vector
        while res.len() > 1 && res[0] == 0 {
            res.remove(0);
        }

        // Convert the result vector back to a string and return it
        String::from_utf8(res.iter().map(|&x| x + b'0').collect()).unwrap()
    }
}