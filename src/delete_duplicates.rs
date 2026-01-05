// Definition for singly-linked list.
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

impl Solution {
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
            head = Some(Box::new(ListNode { val: i, next: head} ));
        }
        head
    }

    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut v = Self::to_vec(head);
        v.dedup();
        Self::to_list(v)
    }
}

// [1,1,2] -> [1,2]
// [1,1,2,3,3] -> [1,2,3]

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn delete_duplicates() {
        let v1 = vec![1,1,2];
        let v2 = vec![1,1,2,3,3];

        assert_eq!(Solution::to_vec(Solution::delete_duplicates(Solution::to_list(v1))), vec![1,2]);
        assert_eq!(Solution::to_vec(Solution::delete_duplicates(Solution::to_list(v2))), vec![1,2,3]);
    }
}