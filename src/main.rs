
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub value: i32,
    pub next:   Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(value: i32) -> Self {
        ListNode {
            value,
            next:  None
        }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        None
    }
}

fn main() {
    println!("Initial structure for solution is initialized");
}
