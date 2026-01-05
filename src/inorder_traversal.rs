// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
	pub val: i32,
	pub left: Option<Rc<RefCell<TreeNode>>>,
	pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
	#[inline]
	pub fn new(val: i32) -> Self {
		TreeNode {
			val,
			left: None,
			right: None
		}
	}
}

pub struct Tree {
	root: Option<Rc<RefCell<TreeNode>>>,
}

impl Tree {
	pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
		Tree {
			root
		}
	}

	pub fn iter(&self) -> InOrderIter {
		InOrderIter::new(self.root.as_ref())
	}
}

pub struct InOrderIter<'a> {
	stack: Vec<&'a Rc<RefCell<TreeNode>>>
}

impl <'a> InOrderIter<'a> {
	pub fn new(root: Option<&'a Rc<RefCell<TreeNode>>>) -> Self {
		if let Some(node) = root {
			InOrderIter { stack: vec![node] }
		} else {
			InOrderIter { stack: vec![] }
		}
	}
}

impl<'a> Iterator for InOrderIter<'a> {
	type Item = &'a Rc<RefCell<TreeNode>>;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(node) = self.stack.pop() {
			if let Some(right) = &node.borrow().right {
				self.stack.push(&right)
			}

			if let Some(left) = &node.borrow().left {
				self.stack.push(&left)
			}

			return Some(node)
		}

		return None
	}
}

use std::rc::Rc;
use std::cell::RefCell;
pub struct Solution;
impl Solution {
	pub fn find_inorder_predecessor(&mut current: &mut Option<Rc<RefCell<TreeNode>>>) -> &Option<Rc<RefCell<TreeNode>>> {
		while current.as_ref().unwrap().borrow().left.is_some() || current.unwrap().borrow().right.is_some() {
			
		}

		&current
	}
	/*
		morris traversal algorithm:
		- for each node, check if it has a left child
		- if not, visit it and move to the right child
		- if it has a left child, find the in-order predecessor (rightmost node in the left tree)
		- make the current node as the right child of its in-order predecessor (temp link)
		- move to the left child
		- when you encounter a temporary link again, the left subtree is fully visited
		- remove the temporary link
		- visit the current node,
		- move to the right child
	 */
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
		let mut result: Vec<i32> = vec![];
		let mut root_copy = root.clone();
		let mut current = &root_copy;

		result
    }
}

// Input: root = [1,null,2,3] -> Output: [1,3,2]
/* 
         1
          \
           2
          /
         3
*/
// Input: root = [1,2,3,4,5,null,8,null,null,6,7,9] -> Output: [4,2,6,5,7,1,3,9,8]
/*
 */
// Input: root = [] -> Output: []
// Input: root = [1] -> Output: [1]