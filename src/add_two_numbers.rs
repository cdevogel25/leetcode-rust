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
    // instead, simply...
    pub fn add_two_numbers(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = ListNode::to_vec(list1); l1.reverse();
        let mut l2 = ListNode::to_vec(list2); l2.reverse();

        let mut c = 0;
        let mut r = Vec::new();

        while !l1.is_empty() || !l2.is_empty() {
            let mut s = c;

            if !l1.is_empty() {
                s += l1.pop().unwrap();
            }

            if !l2.is_empty() {
                s += l2.pop().unwrap();
            }

            if s > 9 {
                c = 1;
                s -= 10;
            } else {
                c = 0;
            }

            r.push(s);
        }

        if c != 0 {
            r.push(c);
        }

        ListNode::to_list(r)
    }

    // slow and bad
    pub fn _add_two_numbers_old(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut s = 0; let mut c = 0;
        let mut l = Some(Box::new(ListNode::new(0)));
        let mut head = l.as_mut();
        let (mut l1, mut l2) = (list1.as_ref(), list2.as_ref());

        loop {
            match (l1.is_some(), l2.is_some()) {
                (true, true) => {
                    s += l1.unwrap().val + l2.unwrap().val;
                    l1 = l1.unwrap().next.as_ref();
                    l2 = l2.unwrap().next.as_ref();
                }
                (true, false) => {
                    s += l1.unwrap().val;
                    l1 = l1.unwrap().next.as_ref();
                }
                (false, true) => {
                    s += l2.unwrap().val;
                    l2 = l2.unwrap().next.as_ref();
                }
                (false, false) => break,
            }
            s += c; c = s/10; s %= 10;
            head.as_mut().unwrap().next = Some(Box::new(ListNode::new(s)));
            head = head.unwrap().next.as_mut();
            s = 0;
        }

        if c > 0 {
            head.as_mut().unwrap().next = Some(Box::new(ListNode::new(c)));
        }

        l.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::{ ListNode, Solution };

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

        assert_eq!(_to_vec(Solution::add_two_numbers(_l1, _l2)), vec![7,0,8]);
    }
}