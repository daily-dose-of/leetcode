use crate::remove_nth_node_from_end_of_list::ListNode;

pub struct PalindromeLinkedListSolution {}

impl PalindromeLinkedListSolution {
    /// Determines whether a linked list is a palindrome.
    ///
    /// # Arguments
    /// * `head` -  The head node of the linked list.
    ///
    /// # Returns
    /// (`bool`):  `true` if the linked list is a palindrome, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// // Example 1:
    /// // Input: head = [1,2,2,1]
    /// // Output: true
    /// use leet_rs::palindrome_linked_list::PalindromeLinkedListSolution;
    /// use leet_rs::remove_nth_node_from_end_of_list::ListNode;
    /// let mut node1 = ListNode::new(1);
    /// let mut node2 = ListNode::new(2);
    /// let mut node3 = ListNode::new(2);
    /// let mut node4 = ListNode::new(1);
    /// node3.next = Some(Box::new(node4));
    /// node2.next = Some(Box::new(node3));
    /// node1.next = Some(Box::new(node2));
    /// let head = Some(Box::new(node1));
    /// println!("{:?}", head); // Verify that the linked list is constructed correctly
    /// // TODO: assert_eq!(PalindromeLinkedListSolution::is_palindrome(head), true);
    ///
    /// // Example 2:
    /// // Input: head = [1,2]
    /// // Output: false
    /// let mut node1 = ListNode::new(1);
    /// let mut node2 = ListNode::new(2);
    /// node1.next = Some(Box::new(node2));
    /// let head = Some(Box::new(node1));
    /// // TODO: assert_eq!(PalindromeLinkedListSolution::is_palindrome(head), false);
    /// ```
    ///
    /// # Time Complexity
    ///
    /// * O(n) - Time taken to find the middle of the linked list, reverse the second half of the linked list,
    /// and compare the two halves of the linked list.
    ///
    /// # Space Complexity
    ///
    /// * O(1) - Space used for pointers and variables.
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        // Initialize two pointers: slow and fast
        let mut slow = &head;
        let mut fast = &head;

        // Find the middle node of the linked list using the two pointers technique
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        // Reverse the second half of the linked list
        let mut reversed_half = None;
        Self::reverse_list(slow, &mut reversed_half);

        // Compare the first half of the linked list with the reversed second half
        let mut p1 = &head;
        let mut p2 = &reversed_half;
        while p2.is_some() {
            if p1.as_ref().unwrap().val != p2.as_ref().unwrap().val {
                return false;
            }
            p1 = &p1.as_ref().unwrap().next;
            p2 = &p2.as_ref().unwrap().next;
        }

        true
    }

    /// Reverses a given linked list.
    ///
    /// # Arguments
    ///
    /// * `head` - A linked list represented by an `Option<Box<ListNode>>`.
    /// * `reversed_half` - A mutable reference to an `Option<Box<ListNode>>>` representing
    ///   the reversed linked list.
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_rs::palindrome_linked_list::PalindromeLinkedListSolution;
    /// use leet_rs::remove_nth_node_from_end_of_list::ListNode;
    /// let mut node1 = ListNode::new(1);
    /// let mut node2 = ListNode::new(2);
    /// let mut node3 = ListNode::new(3);
    /// node2.next = Some(Box::new(node3));
    /// node1.next = Some(Box::new(node2));
    /// let head = Some(Box::new(node1));
    /// let mut reversed_half = None;
    /// PalindromeLinkedListSolution::reverse_list(&head, &mut reversed_half);
    /// // TODO: assert_eq!(reversed_half.unwrap().val, 3);
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(n), where n is the length of the linked list.
    ///
    /// # Space Complexity
    ///
    /// O(n), since the function uses recursion
    pub fn reverse_list(head: &Option<Box<ListNode>>, reversed_half: &mut Option<Box<ListNode>>) {
        if let Some(node) = head {
            // Recursively reverse the rest of the linked list
            Self::reverse_list(&node.next, reversed_half);

            // Create a new node with the same value as the current node
            let mut new_node = ListNode::new(node.val);

            // Append the new node to the beginning of the reversed half of the linked list
            new_node.next = std::mem::take(reversed_half);
            *reversed_half = Some(Box::new(new_node));
        }
    }
}
