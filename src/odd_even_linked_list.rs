use crate::remove_nth_node_from_end_of_list::ListNode;

pub struct Solution {}

impl Solution {
    /// This method uses a "two-pointer" approach, where two pointers are used to traverse a list or array.
    /// The idea is to use two pointers to keep track of the current nodes being processed in the linked list.
    /// One pointer starts at the head of the list and traverses every other node, while the other pointer starts
    /// at the second node and traverses every other node. By doing this, we can effectively split the list into
    /// two halves: one with the odd-indexed nodes and the other with the even-indexed nodes. Finally, we connect
    /// the last node of the odd-indexed list to the first node of the even-indexed list.
    ///
    /// # Arguments
    /// * `head` - A linked list represented by a list node that serves as the head of the list
    ///
    /// # Returns
    /// (`Option<Box<ListNode>>`) - A linked list with odd-indexed nodes appearing before even-indexed nodes
    ///
    /// # Examples
    /// 
    /// ```
    /// use leet_rs::remove_nth_node_from_end_of_list::ListNode;
    /// use leet_rs::odd_even_linked_list::Solution;
    /// // Example 1:
    /// // 1 -> 2 -> 3 -> 4 -> 5
    /// // Returns: 1 -> 3 -> 5 -> 2 -> 4
    /// let mut list_ref = Box::new(ListNode::new(1));
    /// let mut node_ref = &mut list_ref;
    /// node_ref.next = Some(Box::new(ListNode::new(3)));
    /// node_ref = node_ref.next.as_mut().unwrap();
    /// node_ref.next = Some(Box::new(ListNode::new(5)));
    /// node_ref = node_ref.next.as_mut().unwrap();
    /// node_ref.next = Some(Box::new(ListNode::new(2)));
    /// node_ref = node_ref.next.as_mut().unwrap();
    /// node_ref.next = Some(Box::new(ListNode::new(4)));
    ///
    /// let mut list = Some(Box::new(ListNode::new(1)));
    /// let mut current = list.as_mut().unwrap();
    /// current.next = Some(Box::new(ListNode::new(2)));
    /// current = current.next.as_mut().unwrap();
    /// current.next = Some(Box::new(ListNode::new(3)));
    /// current = current.next.as_mut().unwrap();
    /// current.next = Some(Box::new(ListNode::new(4)));
    /// current = current.next.as_mut().unwrap();
    /// current.next = Some(Box::new(ListNode::new(5)));
    /// assert_eq!(
    ///     Solution::odd_even_list(list),
    ///     Some(list_ref)
    /// );
    /// ```
    ///
    /// # Time complexity
    /// O(n), where n is the number of nodes in the linked list.
    ///
    /// # Space complexity
    /// O(1), since only a constant amount of extra space is required.
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        // Edge case
        if head.is_none() {
            return None;
        }

        // Create two empty linked lists for odd and even nodes
        let (mut odd, mut even) = (ListNode::new(0), ListNode::new(0));
        
        // Create mutable references for the last nodes in each list
        let (mut odd_ref, mut even_ref) = (&mut odd, &mut even);
        
        // Initialize a counter for odd/even nodes
        let mut count = 1;
        
        // Iterate over the input linked list
        let mut current = head;
        while let Some(mut node) = current {
            // Move ownership of the next node
            current = node.next.take();
            
            // Insert the node at the end of the appropriate list
            if count % 2 == 1 {
                odd_ref.next = Some(node);
                odd_ref = odd_ref.next.as_mut().unwrap();
            } else {
                even_ref.next = Some(node);
                even_ref = even_ref.next.as_mut().unwrap();
            }
            count += 1;
        }
        
        // Link the two lists together
        odd_ref.next = even.next.take();

        // Return the head of the merged list
        Some(odd.next.take().unwrap())
    }
}
