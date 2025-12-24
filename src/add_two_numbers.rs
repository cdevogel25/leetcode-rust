pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

impl Solution {
    pub fn add_two_numbers(_l1: Option<Box<ListNode>>, _l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // dummy head
        let l3 = Some(Box::new(ListNode::new(0)));
        // let mut l3_ref = l3.as_ref();
        // let mut carry = 0;
        // let mut sum = 0;

        // let mut l1_ref = l1.as_ref();
        // let mut l2_ref = l2.as_ref();

        // loop {
        //     match (l1_ref.unwrap().next.is_some(), l2_ref.unwrap().next.is_some()) {
        //         (true, true) => {
        //             sum = l1_ref.unwrap().val + l2_ref.unwrap().val + carry;
        //             carry = if sum > 9 { 1 } else { 0 };
        //         }
        //         (true, false) => {
        //             sum = l1_ref.unwrap().val + carry;
        //             carry = if sum > 9 { 1 } else { 0 };
        //         }
        //         (false, true) => {
        //             sum = l2_ref.unwrap().val + carry;
        //             carry = if sum > 9 { 1 } else { 0 };
        //         }
        //         (false, false) => break
            // }
            // l3_ref.unwrap().next = Some(Box::new(ListNode::new(if sum > 9 { sum - 9 } else { sum })));
        // }
        l3
    }
}

#[cfg(test)]
mod test {
    use super::{ ListNode };

    // Helper function to create a linked list from a vector
    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &v in vec.iter().rev() {
            let mut node = ListNode::new(v);
            node.next = current;
            current = Some(Box::new(node));
        }
        current
    }

    fn _to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = vec![];
        let mut l = list.as_ref();
        while l.unwrap().next.is_some() {
            vec.push(l.unwrap().val);
            l = l.unwrap().next.as_ref();
        }
        vec.push(l.unwrap().val);
        vec
    }

    #[test]
    fn test_add_two_numbers() {
        // Example: 342 + 465 = 807
        // List 1: 2 -> 4 -> 3
        let _l1 = to_list(vec![2, 4, 3]);
        
        // List 2: 5 -> 6 -> 4
        let _l2 = to_list(vec![5, 6, 4]);

        // println!("{:?}", to_vec(l1));
        // assert_eq!(to_vec(l1), vec![2,4,3]);
        // assert_eq!(Solution::add_two_numbers(l1, l2), vec![7,10,7]);
        
        // let result = Solution::add_two_numbers(l1, l2);
        
        // You can print the result to verify (requires implementing Debug or a helper to print)
        // println!("{:?}", result);
    }
}