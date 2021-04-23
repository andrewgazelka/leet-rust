// #![feature(test)]
// #![feature(int_error_matching)]
//
mod problem001;
mod problem002;
mod problem003;
mod problem007;
mod problem006;
mod problem008;
mod problem009;
mod problem010;
mod problem023;
mod problem773;
mod problem005;
mod problem041;
mod problem042;
mod problem060;
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
