use std::cmp;

pub type NodePtr = Option<Box<ListNode>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next:  NodePtr,
}

impl ListNode {
    const MAX_LINK_DEPTH: usize = 100;


    #[inline]
    pub fn new(value: i32) -> Self {
        ListNode {
            val: value,
            next:  None
        }
    }

    fn iter(&self) -> ListNodeIter {
        ListNodeIter {
            current_node: self.clone(),
            next_value:   None
        }
    }

    pub fn new_from_list(mut values: Vec<i32>) -> ListNode {
        if values.len() == 0 {
            return ListNode::new(0);
        }

        // Define the new ListNode
        let mut base_list_node = ListNode::new(values.pop().unwrap());

        let mut current_list_node = &mut base_list_node;

        for value_ref in values.iter().rev().take(Self::MAX_LINK_DEPTH) {
            // Add new link
            current_list_node.next = Some(Box::new(ListNode::new(*value_ref)));

            // Move to the next link
            current_list_node = current_list_node.next
                .as_mut()
                .unwrap();
        }
        
        base_list_node
    }
    pub fn to_vec(&self) -> Vec<i32> {
        match self.next {
            Some(_) => {
                self.iter().collect()
            }
            None => vec![self.val]
        }
    }


    pub fn print(&self) {
        for (index, value) in self.iter().enumerate() {
            println!("[{}] = {}", index, value);
        }
    }
}

type ListNodeValueType = i32;
struct ListNodeIter {
    current_node: ListNode,
    next_value:   Option<ListNodeValueType>
}


/// Implement an iterator for our ListNode linked list struc
impl Iterator for ListNodeIter {
    type Item = ListNodeValueType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_node.next.is_some() {
            let value = self.current_node.val;

            self.current_node = *self.current_node
                .next
                .as_ref()
                .unwrap()
                .clone();

            self.next_value = Some(self.current_node.val);
            return Some(value);
        }

        self.next_value
            .take()

    }
}



struct Solution;

impl Solution {

    pub fn sum_two_vectors(
        vec_one: &Vec<ListNodeValueType>,
        vec_two: &Vec<ListNodeValueType>
    ) -> Vec<ListNodeValueType> {

        // Declare the target vector for the sum of the two vectors,
        // and declare a carry-over variable
        let mut sum_vec: Vec<ListNodeValueType> = vec![];
        let mut carry = 0;

        // Determine the max length of the two vectors
        let vec_one_len = vec_one.len();
        let vec_two_len = vec_two.len();
        let max_len = cmp::max(vec_one_len, vec_two_len);

        // Iterate over the max length of the two vectors,
        // and read the values of both vectors with the index.
        // `unwrap_or` takes care of the shorter vector's out-of-bounds
        // cases - the method declares a default value of 0 when the index
        // points to an out-of-bounds value.
        for index in 0..max_len {
            let value_one = vec_one.get(index).unwrap_or(&0);
            let value_two = vec_two.get(index).unwrap_or(&0);

            // Get the sm of the two values
            let mut value_sum = value_one + value_two + carry;

            println!(
                "[{}] (a){} + (b){} + (carry){} = (sum){}",
                index,
                value_one,
                value_two,
                carry,
                value_sum
            );


            // If the sum is greater than 10, we need to carry over
            if value_sum > 9 {

                print!("The sum {} is greater than 9, carrying over...", value_sum);

                carry      = value_sum / 10;
                value_sum %= 10;

                println!(" new carry = {}, carried sum {}", carry, value_sum);
            } else {

                if carry > 0 {
                    println!("The carry was consumed, its value was = {}, setting to zero", carry);
                    carry = 0;
                }

            }


            sum_vec.push(value_sum % 10);
        }

        // If there is a carry-over, add it to the end of the vector
        if carry > 0 {
            println!("There is a carry over {}, we need to add it to the end of the vector", carry);
            sum_vec.push(carry);
        }

        sum_vec
    }

    pub fn assert_that_vec_contains_only_decimal_digits(vec: &Vec<i32>) {
        for value in vec.iter() {
            assert!(*value >= 0 && *value <= 9);
        }
    }

    pub fn create_list_node_from_vec(values: &mut Vec<ListNodeValueType>) -> NodePtr {
        // Define the new ListNode
        let mut base_list_node = ListNode::new(*values.first().unwrap_or(&0));

        let mut current_list_node = &mut base_list_node;

        for value_ref in values.iter().skip(1).take(100) {
            // Add new link
            current_list_node.next = Some(Box::new(ListNode::new(*value_ref)));

            // Move to the next link
            current_list_node = current_list_node.next
                .as_mut()
                .unwrap();
        }

        Some(Box::new(base_list_node))

    }

    pub fn add_two_numbers(
        node_one: NodePtr,
        node_two: NodePtr,
    ) -> NodePtr {

        // Get the vector representations of the two linked lists
        let vec_one = node_one.unwrap().as_ref().to_vec();
        let vec_two = node_two.unwrap().as_ref().to_vec();

        Self::assert_that_vec_contains_only_decimal_digits(&vec_one);
        Self::assert_that_vec_contains_only_decimal_digits(&vec_two);

        let mut sum_vec = Self::sum_two_vectors(&vec_one, &vec_two);
        let node_sum = Solution::create_list_node_from_vec(&mut sum_vec);

        node_sum
    }
}

fn main() {
    let listnode_one = ListNode::new_from_list(vec![
        1, 2, 3, 4, 5, 6, 7
    ]);

    listnode_one.print();
    //println!("Initial structure for solution is initialized");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_vec_test() {
        let listnode_one = ListNode::new_from_list(vec![
            1, 2, 3, 4, 5, 6, 7
        ]);

        let vec = listnode_one.to_vec();
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn add_two_numbers_example_1_test() {

        let listnode_one = ListNode::new_from_list(vec![
            2, 4, 3
        ]);

        let listnode_two = ListNode::new_from_list(vec![
            5, 6, 4
        ]);

        let sum_node = Solution::add_two_numbers(
            Some(Box::new(listnode_one)),
            Some(Box::new(listnode_two))
        );

        sum_node.as_ref().unwrap().print();

        assert_eq!(sum_node, Some(Box::new(ListNode::new_from_list(vec![7, 0, 8]))));

    }

    #[test]
    fn add_two_numbers_example_2_test() {

        //let listnode_one = ListNode::new_from_list(vec![0]);
        let listnode_one = ListNode::new_from_list(vec![0]);
        let listnode_two = ListNode::new_from_list(vec![0]);

        let sum_node = Solution::add_two_numbers(
            Some(Box::new(listnode_one)),
            Some(Box::new(listnode_two))
        );

        sum_node.as_ref().unwrap().print();

        assert_eq!(sum_node, Some(Box::new(ListNode::new_from_list(vec![0]))));

    }

    #[test]
    fn add_two_numbers_exampl_3_test() {

        let listnode_one = ListNode::new_from_list(vec![
            9, 9, 9, 9, 9, 9, 9
        ]);

        let listnode_two = ListNode::new_from_list(vec![
            9, 9, 9, 9
        ]);

        let sum_node = Solution::add_two_numbers(
            Some(Box::new(listnode_one)),
            Some(Box::new(listnode_two))
        );

        sum_node.as_ref().unwrap().print();

        assert_eq!(sum_node, Some(Box::new(ListNode::new_from_list(vec![
            8, 9, 9, 9, 0, 0, 0, 1
        ]))));

    }

    #[test]
    fn add_two_numbers_example_1548_test() {

        //let listnode_one = ListNode::new_from_list(vec![0]);
        let listnode_one = ListNode::new(0);
        let listnode_two = ListNode::new(1);

        let sum_node = Solution::add_two_numbers(
            Some(Box::new(listnode_one)),
            Some(Box::new(listnode_two))
        );

        sum_node.as_ref().unwrap().print();

        assert_eq!(sum_node, Some(Box::new(ListNode::new_from_list(vec![1]))));

    }

}