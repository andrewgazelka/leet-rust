#![feature(test)]
#![feature(int_error_matching)]

mod problem1;
mod problem2;
mod problem3;
mod problem7;
mod problem6;
mod problem8;
mod problem9;
mod problem10;
mod problem23;
mod problem773;
mod problem5;
mod problem41;
mod problem42;
mod problem60;
mod problem128;
mod problem329;
mod problem191;


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
