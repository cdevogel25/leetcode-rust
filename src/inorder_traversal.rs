// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
pub struct Solution;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        
    }
}

// Input: root = [1,null,2,3] -> Output: [1,3,2]
// Input: root = [1,2,3,4,5,null,8,null,null,6,7,9] -> Output: [4,2,6,5,7,1,3,9,8]
// Input: root = [] -> Output: []
// Input: root = [1] -> Output: [1]