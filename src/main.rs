
pub type NodePtr = Option<Box<ListNode>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub value: i32,
    pub next:  NodePtr,
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
        node_one: NodePtr,
        node_two: NodePtr,
    ) -> NodePtr {
        None
    }
}

fn main() {
    println!("Initial structure for solution is initialized");
}
