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

use std::rc::Rc;
use std::cell::RefCell;
pub struct Solution;
impl Solution {
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
		let mut result: Vec<i32> = Vec::new();
		let mut current = root;

		// for each node
		while let Some(node) = current.clone() {
			// does it have a left child?
			let left_exists = node.borrow().left.is_some();

			//if not, visit and move to its right child
			if !left_exists {
				result.push(node.borrow().val);
				let right = node.borrow().right.clone();
				current = right;
			} else {
				let mut predecessor = node.borrow().left.clone().unwrap();

				loop {
					let right_child = predecessor.borrow().right.clone();
					if right_child.is_none() || Rc::ptr_eq(right_child.as_ref().unwrap(), &node) {
						break;
					}

					predecessor = right_child.unwrap();
				}

				let is_linked = predecessor.borrow().right.is_some();
				if !is_linked {
					predecessor.borrow_mut().right = Some(node.clone());
					let left = node.borrow().left.clone();
					current = left;
				} else {
					predecessor.borrow_mut().right = None;
					result.push(node.borrow().val);
					let right = node.borrow().right.clone();
					current = right;
				}
			}
		}

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