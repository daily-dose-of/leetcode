pub struct RemoveNthNodeFromEndListSolution {}
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl RemoveNthNodeFromEndListSolution {
    /// Removes the nth node from the end of a singly linked list.
    ///
    /// # Arguments
    ///
    /// * `head` - A singly linked list of type `Option<Box<ListNode>>`.
    /// * `n` - The index of the node to be removed, counting from the end of the list.
    ///
    /// # Returns
    ///
    /// Returns the head of the modified linked list, of type `Option<Box<ListNode>>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_rs::remove_nth_node_from_end_of_list::RemoveNthNodeFromEndListSolution;
    /// use leet_rs::remove_nth_node_from_end_of_list::ListNode;
    /// let mut node1 = ListNode::new(1);
    /// let mut node2 = ListNode::new(2);
    /// let mut node3 = ListNode::new(3);
    /// let mut node4 = ListNode::new(4);
    /// let mut node5 = ListNode::new(5);
    /// node4.next = Some(Box::new(node5));
    /// node3.next = Some(Box::new(node4));
    /// node2.next = Some(Box::new(node3));
    /// node1.next = Some(Box::new(node2));
    /// let head = Some(Box::new(node1));
    /// assert_eq!(
    ///     RemoveNthNodeFromEndListSolution::remove_nth_from_end(head, 2),
    ///     Some(Box::new(ListNode {
    ///         val: 1,
    ///         next: Some(Box::new(ListNode {
    ///             val: 2,
    ///             next: Some(Box::new(ListNode {
    ///                 val: 3,
    ///                 next: Some(Box::new(ListNode {
    ///                     val: 5,
    ///                     next: None,
    ///                 })),
    ///             })),
    ///         })),
    ///     }))
    /// );
    /// ```
    ///
    /// # Time Complexity
    ///
    /// O(n), where n is the length of the linked list.
    ///
    /// # Space Complexity
    ///
    /// O(1), since it only uses a constant amount of additional space.
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // Initialize a variable to keep track of the length of the linked list
        let mut len = 0;
        // Create a mutable reference to the head of the linked list
        let mut curr = &head;
        // Traverse the linked list and count the number of nodes
        while let Some(node) = curr {
            len += 1;
            curr = &node.next;
        }

        // Create a dummy node and set its next pointer to the head of the linked list
        let mut dummy = ListNode { val: 0, next: head };
        // Create a mutable reference to the dummy node
        let mut curr = &mut dummy;
        // Traverse the linked list to find the node that comes before the node to be removed
        for _ in 0..(len - n) {
            curr = curr.next.as_mut().unwrap();
        }
        // Remove the nth node from the end of the linked list
        let removed_node = curr.next.take();
        curr.next = removed_node.unwrap().next;
        // Return the head of the modified linked list
        dummy.next
    }
}
