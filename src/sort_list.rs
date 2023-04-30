use crate::remove_nth_node_from_end_of_list::ListNode;

pub struct Solution {}

impl Solution {
    /// Sorts a linked list in ascending order.
    ///
    /// # Arguments
    ///
    /// * `head` - A linked list of type `Option<Box<ListNode>>`.
    ///
    /// # Returns
    ///
    /// A sorted linked list of type `Option<Box<ListNode>>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use leet_rs::sort_list::Solution;
    /// use leet_rs::remove_nth_node_from_end_of_list::ListNode;
    ///
    /// let mut node1 = ListNode::new(3);
    /// let mut node2 = ListNode::new(2);
    /// let mut node3 = ListNode::new(1);
    /// node2.next = Some(Box::new(node1));
    /// node3.next = Some(Box::new(node2));
    /// let list = Some(Box::new(node3));
    /// let result = Solution::sort_list(list);
    /// assert_eq!(result.clone().unwrap().val, 1);
    /// assert_eq!(result.clone().unwrap().next.unwrap().val, 2);
    /// assert_eq!(result.clone().unwrap().next.unwrap().next.unwrap().val, 3);
    /// ```
    ///
    /// # Time complexity
    ///
    /// O(nlogn)
    ///
    /// # Space complexity
    ///
    /// O(logn)
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // If the list is empty or only contains one node, it is already sorted
        if let Some(ref node) = head {
            if node.next.is_none() {
                return head;
            }
        } else {
            return None;
        }

        // Split the list into two halves, with even nodes in the left half and odd nodes in the right half
        let mut even_list = ListNode::new(-1);
        let mut even_back = &mut even_list;
        let mut odd_list = ListNode::new(-1);
        let mut odd_back = &mut odd_list;
        let mut is_even = true;
        while let Some(mut curr) = head.take() {
            head = curr.next.take();
            if is_even {
                even_back = even_back.next.insert(curr);
                is_even = false;
            } else {
                odd_back = odd_back.next.insert(curr);
                is_even = true;
            }
        }

        // Recursively sort the left and right halves and then merge them
        Self::merge(
            Self::sort_list(even_list.next.take()),
            Self::sort_list(odd_list.next.take()),
        )
    }

    /// Merges two sorted linked lists into one sorted linked list.
    ///
    /// # Arguments
    /// * `left` -  The head of the left linked list.
    /// * `right` - The head of the right linked list.
    ///
    /// # Returns
    /// (`Option<Box<ListNode>>`) - The head of the merged linked list.
    ///
    /// # Time complexity
    ///
    /// O(n), where n is the total number of nodes in the two input linked lists.
    ///
    /// # Space complexity
    ///
    /// O(n), where n is the total number of nodes in the two input linked lists.
    fn merge(
        mut left: Option<Box<ListNode>>,
        mut right: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // Create a new dummy node to act as the head of the sorted list
        let mut sorted_list = ListNode::new(-1);
        // Create a mutable reference to the back of the sorted list
        let mut sorted_back = &mut sorted_list;

        // While both lists are not empty
        while let (Some(ref mut left_node), Some(ref mut right_node)) = (&mut left, &mut right) {
            // If left node value is less than right node value
            if left_node.val < right_node.val {
                // Remove the next node from the left list and append it to the sorted list
                let next = left_node.next.take();
                sorted_back = sorted_back.next.insert(left.take().unwrap());
                // Move to the next node in the left list
                left = next;
            } else {
                // Remove the next node from the right list and append it to the sorted list
                let next = right_node.next.take();
                sorted_back = sorted_back.next.insert(right.take().unwrap());
                // Move to the next node in the right list
                right = next;
            }
        }

        // Add any remaining nodes from the left list to the sorted list
        while let Some(mut left_node) = left.take() {
            left = left_node.next.take();
            sorted_back = sorted_back.next.insert(left_node);
        }

        // Add any remaining nodes from the right list to the sorted list
        while let Some(mut right_node) = right.take() {
            right = right_node.next.take();
            sorted_back = sorted_back.next.insert(right_node);
        }

        // Return the sorted list without the dummy node
        sorted_list.next.take()
    }
}
