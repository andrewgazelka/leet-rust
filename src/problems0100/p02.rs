use crate::{Solution, ListNode};

/// Progresses to the next node and returns the value on the current node
#[inline]
fn progress(given: &mut Option<&ListNode>) -> i32 {
    let (val, next) = match given {
        None => (0,None),
        Some(x) => (x.val, x.next.as_deref())
    };
    *given = next;
    val
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode {
            val: 0,
            next: None,
        };

        let mut current = &mut dummy_head;

        let mut carry = 0;

        let mut on1 = l1.as_deref();
        let mut on2 = l2.as_deref();

        while on1 != None || on2 != None {

            let x = progress(&mut on1);
            let y = progress(&mut on2);

            let sum = x + y + carry;

            current.next = Some(Box::new(ListNode::new(sum % 10)));
            carry = sum / 10;
            current = current.next.as_mut().unwrap();
        }
        if carry > 0 {
            current.next = Some(Box::new(ListNode::new(carry)));
        }
        dummy_head.next
    }
}

#[cfg(test)]
mod tests {
    use crate::{Solution, ListNode};

    #[test]
    fn it_works() {
        let a = ListNode::from_vec(vec![1, 2, 3]);
        let b = a.clone();
        let added = Solution::add_two_numbers(a, b);
        println!("{:?}", added);
    }
}
