mod problem0001;
mod problem0002;
mod problem0003;
mod problem0005;
mod problem0006;
mod problem0007;
mod problem0008;
mod problem0009;
mod problem0010;
mod problem0023;
mod problem0041;
mod problem0042;
mod problem0060;
mod problem0089;
mod problem0128;
mod problem0190;
mod problem0191;
mod problem0329;
mod problem0338;
mod problem0659;
mod problem0717;
mod problem0773;
mod utils;

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
