use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

use crate::{ListNode, Solution};

struct HeapNode(i32, Box<ListNode>);

impl Eq for HeapNode {}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.0).cmp(&self.0)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // reverse
        Some(self.cmp(other))
    }
}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Solution {
    /**
    We will want a min heapify to extract the smallest elements
   */
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut init_data = Vec::with_capacity(lists.len());

        lists.into_iter()
            .filter_map(|list| list)
            .for_each(|list| init_data.push(HeapNode(list.val, list)));

        let mut heap = BinaryHeap::from(init_data);

        let mut dummy_head = ListNode::new(0);
        let mut on = &mut dummy_head;

        while let Some(HeapNode(val, references)) = heap.pop() {
            let next = Box::new(ListNode::new(val));
            on.next = Some(next);
            on = on.next.as_mut().unwrap();
            if let Some(next_reference) = references.next {
                heap.push(HeapNode(next_reference.val, next_reference))
            }
        }

        dummy_head.next
    }
}
