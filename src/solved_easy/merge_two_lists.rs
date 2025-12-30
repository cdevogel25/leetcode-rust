#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn _new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub struct Solution;

// do the same thing?
impl ListNode {
    pub fn to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut next = &list;
        let mut v = Vec::new();

        while next.is_some() {
            let value = next.as_ref().unwrap();
            v.push(value.val);

            next = &value.next;
        }
        v
    }

    pub fn to_list(mut v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        while !v.is_empty() {
            let i = v.pop().unwrap();
            head = Some(Box::new(ListNode { val: i, next: head }));
        }
        head
    }
}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = ListNode::to_vec(list1); l1.reverse();
        let mut l2 = ListNode::to_vec(list2); l2.reverse();
        let mut r = Vec::new();

        loop {
            match (!l1.is_empty(), !l2.is_empty()) {
                (true, true) => {
                    if l1.last().unwrap() <= l2.last().unwrap() {
                        r.push(l1.pop().unwrap());
                    } else {
                        r.push(l2.pop().unwrap());
                    }
                }
                (true, false) => {
                    r.push(l1.pop().unwrap());
                }
                (false, true) => {
                    r.push(l2.pop().unwrap());
                }
                (false, false) => break,
            }
        }
        ListNode::to_list(r)
    }
}

#[cfg(test)]
mod test {
    use super::{ ListNode, Solution };

    #[test]
    fn merge_two_lists() {
        let l1 = ListNode::to_list(vec![1,2,3]);
        let l2 = ListNode::to_list(vec![1,3,4]);

        assert_eq!(ListNode::to_vec(Solution::merge_two_lists(l1, l2)), vec![1,1,2,3,3,4]);
    }
}