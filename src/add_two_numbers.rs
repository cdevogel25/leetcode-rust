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
    fn to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = vec![];
        let mut l = list.as_ref();

        while l.unwrap().next.is_some() {
            vec.push(l.unwrap().val);
            l = l.unwrap().next.as_ref();
        }
        vec.push(l.unwrap().val);
        vec
    }
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    }
}

#[cfg(test)]
mod test {
    use super::{Solution, ListNode};

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

    fn to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
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
        let l1 = to_list(vec![2, 4, 3]);
        
        // List 2: 5 -> 6 -> 4
        let l2 = to_list(vec![5, 6, 4]);

        // println!("{:?}", to_vec(l1));
        assert_eq!(to_vec(l1), vec![2,4,3]);
        
        // let result = Solution::add_two_numbers(l1, l2);
        
        // You can print the result to verify (requires implementing Debug or a helper to print)
        // println!("{:?}", result);
    }
}