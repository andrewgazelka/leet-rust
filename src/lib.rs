// #![feature(stmt_expr_attributes)]

mod utils;
mod problems0100;
mod problems0200;
mod problems0300;
mod problems0400;
mod problems0500;
mod problems0600;
mod problems0700;


pub struct Solution {}


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }

    fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {

        let mut head = None;

        for val in vec.into_iter().rev() {
            head = Some(Box::new(ListNode {
                next: head,
                val,
            }));
        }
        head
    }
}
